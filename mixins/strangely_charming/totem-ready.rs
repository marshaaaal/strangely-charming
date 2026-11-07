fn hook(game: &mut crate::game::Game<'_>) -> core::ops::ControlFlow<()> {
    if !horizon_mods::evanders_config::enabled("inventory_totem") {
        return core::ops::ControlFlow::Continue(());
    }
    fn run(game: &mut crate::game::Game<'_>) {
        let ready = game.held().is_some_and(|s| &*s.item == TOTEM)
            || game.offhand.as_ref().is_some_and(|s| &*s.item == TOTEM)
            || (horizon_mods::evanders_config::enabled("inventory_totem")
                && game
                    .inventory
                    .iter()
                    .flatten()
                    .any(|s| s.item.as_ref() == TOTEM && s.count > 0));
        if let Some(p) = game.player.as_mut() {
            p.health.totem_ready = ready;
        }
    }

    core::ops::ControlFlow::Break(run(game))
}
