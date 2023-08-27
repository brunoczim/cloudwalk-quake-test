use std::collections::HashMap;

pub type PlayerId = u32;

pub type KillCount = i64;

pub type PlayerName = String;

pub type MeansOfKilling = String;

pub const MEANS_OF_KILLING: &[&str] = &[
    "MOD_UNKNOWN", "MOD_SHOTGUN", "MOD_GAUNTLET", "MOD_MACHINEGUN",
    "MOD_GRENADE", "MOD_GRENADE_SPLASH", "MOD_ROCKET", "MOD_ROCKET_SPLASH",
    "MOD_PLASMA", "MOD_PLASMA_SPLASH", "MOD_RAILGUN", "MOD_LIGHTNING",
    "MOD_BFG", "MOD_BFG_SPLASH", "MOD_WATER", "MOD_SLIME", "MOD_LAVA",
    "MOD_CRUSH", "MOD_TELEFRAG", "MOD_FALLING", "MOD_SUICIDE",
    "MOD_TARGET_LASER", "MOD_TRIGGER_HURT", "MOD_NAIL", "MOD_CHAINGUN",
    "MOD_PROXIMITY_MINE", "MOD_KAMIKAZE", "MOD_JUICED", "MOD_GRAPPLE",
];

pub const WORLD_ID: PlayerId = 1022;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Killer {
    World,
    Player(PlayerId),
}

impl Killer {
    pub fn from_id(id: PlayerId) -> Self {
        if id == WORLD_ID {
            Self::World
        } else {
            Self::Player(id)
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Kill {
    pub killer: Killer,
    pub target: PlayerId,
    pub means: MeansOfKilling,
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Game {
    pub players: HashMap<PlayerId, PlayerName>,
    pub kills: Vec<Kill>,
}

#[cfg(test)]
mod test {
    use crate::game::{Killer, WORLD_ID};

    #[test]
    fn killer_from_world() {
        let expected = Killer::World;
        let actual = Killer::from_id(WORLD_ID);
        assert_eq!(expected, actual);
    }

    #[test]
    fn killer_from_player() {
        let expected = Killer::Player(12);
        let actual = Killer::from_id(12);
        assert_eq!(expected, actual);
    }
}
