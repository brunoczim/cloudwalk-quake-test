use std::collections::HashMap;

pub type PlayerId = u64;

pub type KillCount = i64;

pub type PlayerName = String;

pub type MeanOfKilling = String;

pub const MEANS_OF_KILLING: &[&str] = &[
    "MOD_UNKNOWN", "MOD_SHOTGUN", "MOD_GAUNTLET", "MOD_MACHINEGUN",
    "MOD_GRENADE", "MOD_GRENADE_SPLASH", "MOD_ROCKET", "MOD_ROCKET_SPLASH",
    "MOD_PLASMA", "MOD_PLASMA_SPLASH", "MOD_RAILGUN", "MOD_LIGHTNING",
    "MOD_BFG", "MOD_BFG_SPLASH", "MOD_WATER", "MOD_SLIME", "MOD_LAVA",
    "MOD_CRUSH", "MOD_TELEFRAG", "MOD_FALLING", "MOD_SUICIDE",
    "MOD_TARGET_LASER", "MOD_TRIGGER_HURT", "MOD_NAIL", "MOD_CHAINGUN",
    "MOD_PROXIMITY_MINE", "MOD_KAMIKAZE", "MOD_JUICED", "MOD_GRAPPLE",
];

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Killer {
    World,
    Player(PlayerId),
}

#[derive(Debug, Clone)]
pub struct Kill {
    pub killer: Killer,
    pub mean: MeanOfKilling,
    pub target: PlayerId,
}

#[derive(Debug, Clone, Default)]
pub struct Game {
    pub players: HashMap<PlayerId, PlayerName>,
    pub kills: Vec<Kill>,
}
