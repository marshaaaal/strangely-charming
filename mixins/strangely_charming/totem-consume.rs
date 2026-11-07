fn hook(game: &mut crate::game::Game<'_>) -> core::ops::ControlFlow<Option<Body>> {
    if !horizon_mods::evanders_config::enabled("inventory_totem") {
        return core::ops::ControlFlow::Continue(());
    }
    fn run(game: &mut crate::game::Game<'_>) -> Option<Body> {
        if !game.player.as_ref()?.health.totem_used {
            return None;
        }
        // The mods hear a server player's totem before it is used up.
        if let (Some(uuid), Some(name)) = (game.lan_acting_uuid, game.lan_acting_name.clone()) {
            game.native_player_totem(uuid, &name);
        }
        let p = game.player.as_mut()?;
        p.health.totem_used = false;
        let _ = p.set_health(1.0, false);
        let body = Body {
            at: [p.position.x, p.position.y, p.position.z],
            width: f64::from(p.dimensions.width),
            height: f64::from(p.dimensions.height),
        };
        let slot = if game.held().is_some_and(|s| &*s.item == TOTEM) {
            Some(game.selected)
        } else if horizon_mods::evanders_config::enabled("inventory_totem")
            && !game
                .offhand
                .as_ref()
                .is_some_and(|s| s.item.as_ref() == TOTEM)
        {
            game.inventory.iter().position(|s| {
                s.as_ref()
                    .is_some_and(|s| s.item.as_ref() == TOTEM && s.count > 0)
            })
        } else {
            None
        };
        let cell = match slot {
            Some(i) => &mut game.inventory[i],
            None => &mut game.offhand,
        };
        if let Some(s) = cell.as_mut() {
            s.count -= 1;
            if s.count == 0 {
                *cell = None;
            }
        }
        game.award_stat("used", TOTEM, 1);
        game.trigger_item(
            "minecraft:used_totem",
            &horizon_client::items::ItemStack::new(TOTEM, 1),
        );
        // ItemStack.causeUseVibration: the totem's use effects make one.
        let ctx = game.player_context(None);
        game.post_game_event("minecraft:item_interact_finish", body.at, ctx);
        // DeathProtection.TOTEM_EFFECTS: clear, then regeneration II, absorption II, fire resistance.
        game.potions.effects.clear();
        for (id, ticks, amp) in [
            ("regeneration", 900, 1),
            ("absorption", 100, 1),
            ("fire_resistance", 800, 0),
        ] {
            if let Some(e) = horizon_client::brewing::effect(id) {
                game.add_player_effect(EffectInstance::new(e.id, ticks, amp));
            }
        }
        Some(body)
    }

    core::ops::ControlFlow::Break(run(game))
}
