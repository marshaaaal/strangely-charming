fn hook(game: &mut crate::game::Game<'_>, button: MouseButton, pressed: bool) {
    if !pressed || button != MouseButton::Right || game.sleeping.is_some() {
        return;
    }
    if let Some(minutes) = game.cooldowns.evanders_horn_minutes()
        && game
            .held()
            .into_iter()
            .chain(game.offhand.as_ref())
            .any(|s| s.item.as_ref() == "strangely_charming:raid_horn")
        && horizon_mods::evanders_config::enabled("raid_horns")
    {
        // play_mouse runs in the acting window. system_message appends to its local chat.
        let unit = if minutes == 1 { "minute" } else { "minutes" };
        game.system_message(horizon_client::text::Text::literal(format!(
            "Retreat Horn can't be used for another {minutes} {unit}."
        )));
    }
}
