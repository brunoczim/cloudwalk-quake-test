//! This module exposes game datatype and related items common to all other
//! modules.

use indexmap::IndexSet;
use std::{collections::HashMap, sync::OnceLock};

/// Player ID in the log file. This integer bit size should be Ok for an old
/// game.
pub type PlayerId = u32;

/// Kill count of a player/MOD in the log file. Allows negative values because a
/// player can have negative score since a player loses points when killed by
/// the world.
pub type KillCount = i64;

/// Player name in the log file. This is an expensive-clone string buffer, but
/// with the current software requirements, it wouldn't be cloned as much. In
/// the future it could be a reference-counted string or an interned string.
pub type PlayerName = String;

/// Means of Death (MOD) as referenced by the log file. It could be an `enum`,
/// but `enum` is not necessary in this case. A simple string literal is enough
/// and it is cheap to copy.
pub type MeansOfDeath = &'static str;

/// Returns a set with all valid MODs. This set is created only in the first
/// call.
pub fn all_means_of_death() -> &'static IndexSet<MeansOfDeath> {
    static MOD_CELL: OnceLock<IndexSet<MeansOfDeath>> = OnceLock::new();
    MOD_CELL.get_or_init(|| {
        IndexSet::from([
            "MOD_UNKNOWN", "MOD_SHOTGUN", "MOD_GAUNTLET", "MOD_MACHINEGUN",
            "MOD_GRENADE", "MOD_GRENADE_SPLASH", "MOD_ROCKET",
            "MOD_ROCKET_SPLASH", "MOD_PLASMA", "MOD_PLASMA_SPLASH",
            "MOD_RAILGUN", "MOD_LIGHTNING", "MOD_BFG", "MOD_BFG_SPLASH",
            "MOD_WATER", "MOD_SLIME", "MOD_LAVA", "MOD_CRUSH", "MOD_TELEFRAG",
            "MOD_FALLING", "MOD_SUICIDE", "MOD_TARGET_LASER",
            "MOD_TRIGGER_HURT", "MOD_NAIL", "MOD_CHAINGUN",
            "MOD_PROXIMITY_MINE", "MOD_KAMIKAZE", "MOD_JUICED", "MOD_GRAPPLE",
        ])
    })
}

/// The agent that kills another agent in the `Kill` event.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Killer {
    /// The world is the killer, e.g. the target died as an accident.
    World,
    /// The killer is an actual player.
    Player(PlayerId),
}

/// A `Kill` event as read by the log file.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Kill {
    /// The killer agent, player or world.
    pub killer: Killer,
    /// The target agent, always a dead player.
    pub target: PlayerId,
    /// The way this killing happened.
    pub means: MeansOfDeath,
}

/// A game, a full match as read by the logs.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Game {
    /// Dictionary mapping player IDs to the names they last used in the game.
    pub players: HashMap<PlayerId, PlayerName>,
    /// A list of `Kill` events in the order they happened.
    pub kills: Vec<Kill>,
}
