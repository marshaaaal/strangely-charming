fn hook(
    game: &mut crate::game::Game<'_>,
    host: &mut LanHost,
    conn: u32,
    packet: &S,
) -> core::ops::ControlFlow<()> {
    use crate::game::strangely_charming::{ACTION_CHANNEL, PORTABLE_TABLE, carries_table, compact};
    if let S::CustomClickAction(p) = packet
        && let Some(action) = p.id.strip_prefix("strangely_charming:quest/")
    {
        if action.len() <= 80
            && host
                .hub
                .guests
                .iter()
                .any(|g| g.conn == conn && g.placed && !g.dead && !g.spectator() && g.health > 0.)
        {
            game.as_guest(host, conn, |g| g.sc_quest_action(action));
        }
        return core::ops::ControlFlow::Break(());
    }
    let S::CustomPayload(p) = packet else {
        return core::ops::ControlFlow::Continue(());
    };
    if p.channel != ACTION_CHANNEL {
        return core::ops::ControlFlow::Continue(());
    }
    let done = core::ops::ControlFlow::Break(());
    let [action, a, b, c, d] = p.data.as_slice() else {
        return done;
    };
    let window = i32::from_be_bytes([*a, *b, *c, *d]);
    let Some(i) = host.hub.guests.iter().position(|g| g.conn == conn) else {
        return done;
    };
    let guest = &host.hub.guests[i];
    if guest.dead || guest.spectator() || guest.health <= 0. {
        return done;
    }
    if *action == 1
        && window == 0
        && guest.menu.is_none()
        && carries_table(guest)
        && horizon_mods::evanders_config::enabled("portable_crafting")
    {
        game.lan_open_crafting(host, conn, PORTABLE_TABLE);
    } else if *action == 2 && horizon_mods::evanders_config::enabled("inventory_tidying") {
        if let Some(menu) = guest.menu.as_ref() {
            if window != menu.window || !menu.slots.still_valid(&game.world, guest, menu.pos) {
                return done;
            }
            let slots = menu.slots;
            let supported = match slots {
                Slots::Double(..) | Slots::Ender => true,
                Slots::Block((x, y, z)) => {
                    let block = game.block_name([x, y, z]);
                    matches!(
                        block.as_ref(),
                        "minecraft:chest" | "minecraft:barrel" | "minecraft:trapped_chest"
                    ) || block.ends_with("shulker_box")
                }
                _ => false,
            };
            if supported && let Some(mut items) = slots.read(&game.world, guest) {
                compact(&mut items, |s| game.max_stack(s));
                slots.write(&mut game.world, &mut host.hub.guests[i], items);
                host.hub.sync_containers(&game.world);
            }
        } else if window == 0 {
            let inventory = &mut host.hub.guests[i].inventory;
            if let Some(bag) = inventory.get_mut(9..36) {
                compact(bag, |s| game.max_stack(s));
                host.hub.send_inventory(i);
            }
        }
    }
    done
}
