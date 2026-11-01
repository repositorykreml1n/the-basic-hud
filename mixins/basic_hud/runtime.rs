// The Basic HUD: the effects on the player with their time left, the potions,
// golden apples and totems carried, the durability of armour, elytra and
// shield, and how many totems each player has used. This file is both the
// mod's Cargo library (its tests) and the module the bundle adds to the game,
// so it knows nothing of the game's types.
use std::cell::RefCell;
use std::collections::{BTreeSet, HashMap};

/// The mod's id (its settings folder), name and version.
pub const ID: &str = "basic_hud";
pub const NAME: &str = "The Basic HUD";
pub const VERSION: &str = "0.1.0";
pub const DESCRIPTION: &str = "Effect timers, potions, golden apples and totems carried, totems used by each player, and armour, elytra and shield durability.";
pub const BACKGROUND: u32 = 0x7800_0000;
pub const INK: u32 = 0xFF_FFFF;
pub const GOOD: u32 = 0x7C_E0_7C;
pub const BAD: u32 = 0xFF_70_70;
pub const WARNING: u32 = 0xFF_C8_40;
pub const TOTEM: u32 = 0xFF_D8_50;
/// Effects this close to their end show in `WARNING`, in ticks.
pub const ENDING: i32 = 200;
pub const GOLDEN_APPLE: &str = "minecraft:golden_apple";
pub const ENCHANTED_APPLE: &str = "minecraft:enchanted_golden_apple";
pub const TOTEM_ITEM: &str = "minecraft:totem_of_undying";
/// Beside a count of totems used.
pub const POPPED: &str = "Totems used";

/// An effect the settings can pin.
pub struct Known {
    pub id: &'static str,
    /// Comes from a brewed potion, so it is pinned until the player says not.
    pub potion: bool,
    pub harmful: bool,
}

const fn known(id: &'static str, potion: bool, harmful: bool) -> Known {
    Known { id, potion, harmful }
}

/// Instant health and damage are left out: they have no time to show.
pub const EFFECTS: [Known; 34] = [
    known("speed", true, false),
    known("slowness", true, true),
    known("strength", true, false),
    known("jump_boost", true, false),
    known("regeneration", true, false),
    known("resistance", true, false),
    known("fire_resistance", true, false),
    known("water_breathing", true, false),
    known("invisibility", true, false),
    known("night_vision", true, false),
    known("poison", true, true),
    known("weakness", true, true),
    known("slow_falling", true, false),
    known("luck", true, false),
    known("wind_charged", true, true),
    known("weaving", true, true),
    known("oozing", true, true),
    known("infested", true, true),
    known("haste", false, false),
    known("mining_fatigue", false, true),
    known("nausea", false, true),
    known("blindness", false, true),
    known("hunger", false, true),
    known("saturation", false, false),
    known("wither", false, true),
    known("health_boost", false, false),
    known("absorption", false, false),
    known("glowing", false, false),
    known("levitation", false, true),
    known("unluck", false, true),
    known("conduit_power", false, false),
    known("dolphins_grace", false, false),
    known("darkness", false, true),
    known("bad_omen", false, true),
];

/// What the player chose, kept as one line in the `config` setting.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Config {
    pub effects: bool,
    pub items: bool,
    pub durability: bool,
    pub totems: bool,
    /// Every effect on the player, not only the pinned ones.
    pub all_effects: bool,
    /// Index into [`CORNERS`].
    pub corner: usize,
    pub pinned: BTreeSet<String>,
}

pub const CORNERS: [&str; 4] = ["top left", "top right", "bottom left", "bottom right"];

impl Default for Config {
    fn default() -> Self {
        Self {
            effects: true,
            items: true,
            durability: true,
            totems: true,
            all_effects: false,
            corner: 0,
            pinned: EFFECTS.iter().filter(|e| e.potion).map(|e| e.id.to_owned()).collect(),
        }
    }
}

impl Config {
    /// `key=value` pairs by `;`; a missing or broken one keeps its default.
    pub fn parse(stored: Option<&str>) -> Self {
        let mut config = Self::default();
        for pair in stored.unwrap_or("").split(';') {
            let Some((key, value)) = pair.trim().split_once('=') else {
                continue;
            };
            let on = value != "0";
            match key {
                "effects" => config.effects = on,
                "items" => config.items = on,
                "durability" => config.durability = on,
                "totems" => config.totems = on,
                "all" => config.all_effects = on,
                "corner" => config.corner = value.parse::<usize>().map_or(0, |c| c % CORNERS.len()),
                "pinned" => {
                    config.pinned = value.split(',').filter(|i| !i.is_empty()).map(str::to_owned).collect();
                }
                _ => {}
            }
        }
        config
    }
    pub fn line(&self) -> String {
        let pinned: Vec<&str> = self.pinned.iter().map(String::as_str).collect();
        format!(
            "effects={};items={};durability={};totems={};all={};corner={};pinned={}",
            u8::from(self.effects),
            u8::from(self.items),
            u8::from(self.durability),
            u8::from(self.totems),
            u8::from(self.all_effects),
            self.corner,
            pinned.join(","),
        )
    }
    /// Whether an active effect belongs on the HUD. Beacons and conduits
    /// (`ambient`) are not potions, whatever effect they give.
    pub fn shows_effect(&self, id: &str, ambient: bool) -> bool {
        self.all_effects || (!ambient && self.pinned.contains(id))
    }
    /// The click on settings button `button`, in the order of [`labels`].
    pub fn click(&mut self, button: usize) {
        match button {
            0 => self.effects = !self.effects,
            1 => self.all_effects = !self.all_effects,
            2 => self.items = !self.items,
            3 => self.durability = !self.durability,
            4 => self.totems = !self.totems,
            5 => self.corner = (self.corner + 1) % CORNERS.len(),
            n => {
                if let Some(e) = EFFECTS.get(n - 6) {
                    if !self.pinned.remove(e.id) {
                        self.pinned.insert(e.id.to_owned());
                    }
                }
            }
        }
    }
}

fn switch(on: bool) -> &'static str {
    if on { "ON" } else { "OFF" }
}

/// The settings page: `name` gives an effect's name in the player's language.
pub fn labels(config: &Config, name: impl Fn(&str) -> String) -> Vec<String> {
    let mut labels = vec![
        format!("Effects: {}", switch(config.effects)),
        format!("Unpinned effects too: {}", switch(config.all_effects)),
        format!("Potions, apples, totems: {}", switch(config.items)),
        format!("Durability: {}", switch(config.durability)),
        format!("Totems used: {}", switch(config.totems)),
        format!("Corner: {}", CORNERS[config.corner]),
    ];
    labels.extend(EFFECTS.iter().map(|e| {
        format!("{}: {}", name(e.id), if config.pinned.contains(e.id) { "pinned" } else { "-" })
    }));
    labels
}

/// Time left as the game's own effect list writes it: `m:ss`, `h:mm:ss`,
/// and `**:**` for an endless effect (a duration of -1).
pub fn duration(ticks: i32) -> String {
    if ticks < 0 {
        return "**:**".to_owned();
    }
    let seconds = (ticks + 19) / 20;
    let (hours, minutes, seconds) = (seconds / 3600, seconds / 60 % 60, seconds % 60);
    if hours > 0 {
        format!("{hours}:{minutes:02}:{seconds:02}")
    } else {
        format!("{minutes}:{seconds:02}")
    }
}

pub fn harmful(id: &str) -> bool {
    EFFECTS.iter().any(|e| e.id == id && e.harmful)
}

pub fn effect_colour(id: &str, ticks: i32) -> u32 {
    if (0..ENDING).contains(&ticks) {
        WARNING
    } else if harmful(id) {
        BAD
    } else {
        GOOD
    }
}

/// Green while it is fine, then amber, then red under a tenth.
pub fn durability_colour(left: u32, max: u32) -> u32 {
    match left.saturating_mul(100) / max.max(1) {
        0..=10 => BAD,
        11..=35 => WARNING,
        _ => GOOD,
    }
}

pub fn durability_text(left: u32, max: u32) -> String {
    format!("{left}/{max}")
}

/// Counts of the same `key`, in the order of each key's first stack.
#[derive(Debug, Default, PartialEq, Eq)]
pub struct Tally<K> {
    pub rows: Vec<(K, u32)>,
}

impl<K: PartialEq> Tally<K> {
    pub fn add(&mut self, key: K, count: u32) {
        match self.rows.iter_mut().find(|(k, _)| *k == key) {
            Some((_, n)) => *n += count,
            None => self.rows.push((key, count)),
        }
    }
}

/// Totems each player has used, by name. A player keeps their count until
/// they die or this player leaves the world.
#[derive(Debug, Default)]
pub struct Pops {
    by_name: HashMap<String, u32>,
}

impl Pops {
    pub fn add(&mut self, name: &str) {
        *self.by_name.entry(name.to_owned()).or_default() += 1;
    }
    pub fn get(&self, name: &str) -> u32 {
        self.by_name.get(name).copied().unwrap_or(0)
    }
    pub fn forget(&mut self, name: &str) {
        self.by_name.remove(name);
    }
    pub fn clear(&mut self) {
        self.by_name.clear();
    }
}

thread_local! {
    static CONFIG: RefCell<Option<Config>> = const { RefCell::new(None) };
    static POPS: RefCell<Pops> = RefCell::new(Pops::default());
}

/// The settings, read once through `get` (the `config` setting).
pub fn config<R>(get: impl FnOnce() -> Option<String>, read: impl FnOnce(&Config) -> R) -> R {
    CONFIG.with(|c| {
        let mut c = c.borrow_mut();
        read(c.get_or_insert_with(|| Config::parse(get().as_deref())))
    })
}

/// Changes the settings; the line to keep.
pub fn change(get: impl FnOnce() -> Option<String>, write: impl FnOnce(&mut Config)) -> String {
    CONFIG.with(|c| {
        let mut c = c.borrow_mut();
        let config = c.get_or_insert_with(|| Config::parse(get().as_deref()));
        write(config);
        config.line()
    })
}

pub fn pops<R>(read: impl FnOnce(&mut Pops) -> R) -> R {
    POPS.with(|p| read(&mut p.borrow_mut()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn potions_are_pinned_by_default_and_others_are_not() {
        let c = Config::default();
        assert!(c.shows_effect("strength", false));
        assert!(c.shows_effect("fire_resistance", false));
        assert!(!c.shows_effect("haste", false));
        assert!(!c.shows_effect("bad_omen", false));
    }

    #[test]
    fn a_beacon_effect_is_not_a_potion() {
        let mut c = Config::default();
        assert!(!c.shows_effect("speed", true));
        c.all_effects = true;
        assert!(c.shows_effect("speed", true));
        assert!(c.shows_effect("some_mod:effect", false));
    }

    #[test]
    fn durations_read_like_the_game() {
        assert_eq!(duration(-1), "**:**");
        assert_eq!(duration(0), "0:00");
        assert_eq!(duration(1), "0:01");
        assert_eq!(duration(20), "0:01");
        assert_eq!(duration(3600), "3:00");
        assert_eq!(duration(9600), "8:00");
        assert_eq!(duration(72_000 + 1200 + 20), "1:01:01");
    }

    #[test]
    fn the_config_survives_its_line() {
        let mut c = Config::default();
        c.click(1);
        c.click(5);
        c.click(6);
        c.click(6 + 18);
        assert!(!c.pinned.contains("speed"));
        assert!(c.pinned.contains("haste"));
        assert_eq!(Config::parse(Some(&c.line())), c);
    }

    #[test]
    fn an_empty_pin_list_stays_empty() {
        let mut c = Config::default();
        c.pinned.clear();
        assert!(Config::parse(Some(&c.line())).pinned.is_empty());
    }

    #[test]
    fn a_broken_line_keeps_the_defaults() {
        assert_eq!(Config::parse(Some("nonsense;corner=x;=")), Config::default());
        assert_eq!(Config::parse(None), Config::default());
        assert_eq!(Config::parse(Some("corner=7")).corner, 3);
    }

    #[test]
    fn every_label_is_a_button_the_click_understands() {
        let c = Config::default();
        let labels = labels(&c, |id| id.to_owned());
        assert_eq!(labels.len(), 6 + EFFECTS.len());
        assert_eq!(labels[6], "speed: pinned");
        assert_eq!(labels[6 + 18], "haste: -");
        let mut c = c;
        c.click(labels.len());
        assert_eq!(c, Config::default());
    }

    #[test]
    fn effect_ids_are_unique() {
        let ids: BTreeSet<_> = EFFECTS.iter().map(|e| e.id).collect();
        assert_eq!(ids.len(), EFFECTS.len());
    }

    #[test]
    fn colours_warn_near_the_end() {
        assert_eq!(effect_colour("speed", 1000), GOOD);
        assert_eq!(effect_colour("poison", 1000), BAD);
        assert_eq!(effect_colour("speed", 100), WARNING);
        assert_eq!(effect_colour("speed", -1), GOOD);
        assert_eq!(durability_colour(432, 432), GOOD);
        assert_eq!(durability_colour(100, 432), WARNING);
        assert_eq!(durability_colour(10, 432), BAD);
        assert_eq!(durability_colour(0, 0), BAD);
    }

    #[test]
    fn stacks_of_one_kind_add_up() {
        let mut t = Tally::default();
        t.add("a", 1);
        t.add("b", 16);
        t.add("a", 2);
        assert_eq!(t.rows, vec![("a", 3), ("b", 16)]);
    }

    #[test]
    fn pops_count_per_player_until_forgotten() {
        let mut p = Pops::default();
        p.add("Alex");
        p.add("Alex");
        p.add("Steve");
        assert_eq!((p.get("Alex"), p.get("Steve"), p.get("Kai")), (2, 1, 0));
        p.forget("Alex");
        assert_eq!(p.get("Alex"), 0);
        p.clear();
        assert_eq!(p.get("Steve"), 0);
    }
}
