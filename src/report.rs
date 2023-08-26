use crate::game::{
    Game,
    KillCount,
    Killer,
    MeanOfKilling,
    PlayerName,
    MEANS_OF_KILLING,
};
use std::collections::{HashMap, HashSet};

pub type GameName = String;

#[derive(Debug, Clone, serde::Serialize)]
pub struct GameReport {
    pub total_kills: KillCount,
    pub players: HashSet<PlayerName>,
    pub kills: HashMap<PlayerName, KillCount>,
    pub kills_by_means: HashMap<MeanOfKilling, KillCount>,
}

impl GameReport {
    pub fn generate(game: &Game) -> Self {
        let total_kills =
            KillCount::try_from(game.kills.len()).unwrap_or(KillCount::MAX);

        let players = game.players.values().cloned().collect();

        let kills =
            game.players.values().cloned().map(|name| (name, 0)).collect();

        let kills_by_means = MEANS_OF_KILLING
            .iter()
            .copied()
            .map(|mean| (MeanOfKilling::from(mean), 0))
            .collect();

        let mut this = Self { total_kills, players, kills, kills_by_means };

        for kill in &game.kills {
            match kill.killer {
                Killer::World => {
                    let mut player_name = &game.players[&kill.target];
                    this.kills[player_name] -= 1;
                },
                Killer::Player(killer_id) => {
                    let mut player_name = &game.players[&killer_id];
                    this.kills[player_name] += 1;
                },
            }
            this.kills_by_means[&kill.mean] += 1;
        }

        this
    }
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct LogReport {
    pub games: HashMap<GameName, GameReport>,
}

impl LogReport {
    pub fn generate<I, E>(game_iter: I) -> Result<Self, E>
    where
        I: IntoIterator<Item = Result<Game, E>>,
    {
        let mut this = Self { games: HashMap::new() };
        for (i, result) in game_iter.into_iter().enumerate() {
            let game = result?;
            let game_report = GameReport::generate(&game);
            let game_id = format!("game_{}", i + 1);
            this.games.insert(game_id, game_report);
        }
        Ok(this)
    }
}
