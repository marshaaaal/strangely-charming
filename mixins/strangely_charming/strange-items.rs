use super::Game;
use horizon_client::{entities::Experience, items::ItemStack, text::Text};
use horizon_world::nbt::{Compound, Tag};

pub(super) const TOTEM: &str = "strangely_charming:totem_of_enchanting";
const POINTS: &str = "strangely_charming:experience";

pub(super) fn custom<'a>(stack: &'a ItemStack, key: &str) -> Option<&'a Tag> {
    stack.data()?.other("minecraft:custom_data")?.get(key)
}
pub(super) fn set_custom(stack: &mut ItemStack, key: &str, value: Tag) {
    let mut tag = stack
        .data()
        .and_then(|d| d.other("minecraft:custom_data"))
        .cloned()
        .unwrap_or_else(|| Tag::Compound(Compound::new()));
    if let Tag::Compound(fields) = &mut tag {
        fields.insert(key.into(), value);
        stack
            .data_mut()
            .set_other("minecraft:custom_data", Some(tag));
    }
}
fn base(level: i64) -> i64 {
    match level {
        0..=16 => level * (level + 6),
        17..=31 => (5 * level * level - 81 * level + 720) / 2,
        _ => (9 * level * level - 325 * level + 4440) / 2,
    }
}
fn points(xp: Experience) -> Option<i32> {
    let level = xp.level;
    if !(0..=21864).contains(&level) || !xp.progress.is_finite() {
        return None;
    }
    let partial = (f64::from(xp.progress) * f64::from(Experience::needed(level))).round() as i64;
    i32::try_from(base(i64::from(level)) + partial.max(0)).ok()
}
fn from_points(points: i32) -> Experience {
    let mut lo = 0i64;
    let mut hi = 21865;
    while lo + 1 < hi {
        let mid = (lo + hi) / 2;
        if base(mid) <= i64::from(points) {
            lo = mid;
        } else {
            hi = mid;
        }
    }
    Experience {
        level: lo as i32,
        total: points,
        progress: (i64::from(points) - base(lo)) as f32 / Experience::needed(lo as i32) as f32,
    }
}
impl Game<'_> {
    pub(super) fn sc_notice(&mut self, text: impl Into<String>) {
        let text = Text::literal(text.into());
        let who = self.native_acting_id();
        if !self.native_tell(who, text.clone()) {
            self.system_message(text);
        }
    }
    pub(super) fn sc_use_item(&mut self, stack: &ItemStack) -> bool {
        if stack.count == 0 {
            return false;
        }
        if horizon_mods::evanders_config::enabled("scrollkeepers")
            && let Some(kind) = super::sc_quests::Kind::item(&stack.item)
        {
            self.sc_scroll(stack, kind);
            return true;
        }
        if stack.item.as_ref() != TOTEM
            || !horizon_mods::evanders_config::enabled("enchanting_totems")
        {
            return false;
        }
        let stored = match custom(stack, POINTS) {
            Some(Tag::Int(n)) if *n >= 0 => *n,
            None => 0,
            _ => {
                self.sc_notice("This totem's stored XP could not be read.");
                return true;
            }
        };
        let Some(balance) = points(self.experience) else {
            self.sc_notice("Your XP exceeds the totem's capacity.");
            return true;
        };
        let Some(sum) = stored.checked_add(balance) else {
            self.sc_notice("There is too much XP to combine.");
            return true;
        };
        if self.native_sneaking() {
            if balance == 0 {
                self.sc_notice(format!(
                    "Totem holds {stored} XP. You have no XP to deposit."
                ));
                return true;
            }
            let Some(held) = self.held_mut().as_mut() else {
                return true;
            };
            set_custom(held, POINTS, Tag::Int(sum));
            held.data_mut().set_other(
                "minecraft:lore",
                Some(Tag::List(vec![Tag::String(
                    format!("Stored XP: {sum}").as_str().into(),
                )])),
            );
            self.experience = Experience::default();
            self.sc_notice(format!("Deposited {balance} XP. Totem holds {sum} XP."));
        } else if stored == 0 {
            self.sc_notice("Sneak and use this totem to store your XP.");
        } else {
            *self.held_mut() = None;
            self.experience = from_points(sum);
            self.sc_notice(format!("The totem broke and returned {stored} XP."));
        }
        true
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn bank_round_trips_spendable_points() {
        for n in (0..20000).chain([999_999, i32::MAX - 1, i32::MAX]) {
            assert_eq!(points(from_points(n)), Some(n));
        }
        let mut spent = from_points(353);
        spent.total = 2500;
        assert_eq!(points(spent), Some(353));
    }
}
