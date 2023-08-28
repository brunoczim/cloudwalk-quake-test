//! This module exposes structures with the finality of building report
//! datatypes from game information collected from the log file.

use crate::{
    error::Result,
    game::{
        all_means_of_death,
        Game,
        KillCount,
        Killer,
        MeansOfDeath,
        PlayerName,
    },
};
use anyhow::anyhow;
use indexmap::{IndexMap, IndexSet};

#[cfg(test)]
mod test;

/// Game name in the dictionary of games. This is an expensive-clone string
/// buffer, but with the current software requirements, it wouldn't be cloned as
/// much. In the future it could be a reference-counted string or an interned
/// string.
pub type GameName = String;

/// A datatype representing the report of a single game, friendly to `serde`
/// (serialization/deserialization library).
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
pub struct GameReport {
    /// Total kill count in the game, including the world's.
    pub total_kills: KillCount,
    /// Set of player names that were in the match, more specifically, the last
    /// name they used.
    pub players: IndexSet<PlayerName>,
    /// The mapping of player names to their scores in terms of killing,
    /// discounting `1` for each time they died because of the "world".
    pub kills: IndexMap<PlayerName, KillCount>,
    /// The dictionary counting how many killings happened using each means of
    /// death.
    pub kills_by_means: IndexMap<MeansOfDeath, KillCount>,
}

impl GameReport {
    /// Generate the report object from the given game data.
    pub fn generate(game: &Game) -> Result<Self> {
        let total_kills =
            KillCount::try_from(game.kills.len()).unwrap_or(KillCount::MAX);

        let players = game.players.values().cloned().collect();

        let kills =
            game.players.values().cloned().map(|name| (name, 0)).collect();

        let kills_by_means = all_means_of_death()
            .iter()
            .copied()
            .map(|means| (MeansOfDeath::from(means), 0))
            .collect();

        let mut this = Self { total_kills, players, kills, kills_by_means };

        for kill in &game.kills {
            match kill.killer {
                Killer::World => {
                    let player_name = &game.players[&kill.target];
                    if let Some(kills) = this.kills.get_mut(player_name) {
                        *kills -= 1;
                    } else {
                        log::error!(
                            "Bad game report: player {:?} (kill target) was \
                             not found",
                            player_name
                        );
                    }
                },
                Killer::Player(killer_id) => {
                    let player_name = &game.players[&killer_id];
                    if let Some(kills) = this.kills.get_mut(player_name) {
                        *kills += 1;
                    } else {
                        log::error!(
                            "Bad game report: player {:?} (killer) was not \
                             found",
                            player_name
                        );
                    }
                },
            }
            let kills =
                this.kills_by_means.get_mut(&kill.means).ok_or_else(|| {
                    anyhow!("unknown means of killing: {}", kill.means)
                })?;
            *kills += 1;
        }

        Ok(this)
    }
}

/// A report of the full Quake III: Arena log file.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
pub struct LogReport {
    /// Dictionary of game reports.
    pub games: IndexMap<GameName, GameReport>,
}

impl LogReport {
    /// Generates a report of the whole log file using an iterator over game
    /// data.
    pub fn generate<I>(game_iter: I) -> Result<Self>
    where
        I: IntoIterator<Item = Result<Game>>,
    {
        let mut this = Self { games: IndexMap::new() };
        for (i, result) in game_iter.into_iter().enumerate() {
            let game = result?;
            let game_report = GameReport::generate(&game)?;
            let game_id = format!("game_{}", i + 1);
            this.games.insert(game_id, game_report);
        }
        Ok(this)
    }
}
