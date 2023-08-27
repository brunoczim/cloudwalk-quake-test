use crate::game::{
    Game,
    KillCount,
    Killer,
    MeansOfKilling,
    PlayerName,
    MEANS_OF_KILLING,
};
use indexmap::{IndexMap, IndexSet};

pub type GameName = String;

#[derive(Debug, Clone, serde::Serialize)]
pub struct GameReport {
    pub total_kills: KillCount,
    pub players: IndexSet<PlayerName>,
    pub kills: IndexMap<PlayerName, KillCount>,
    pub kills_by_means: IndexMap<MeansOfKilling, KillCount>,
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
            .map(|means| (MeansOfKilling::from(means), 0))
            .collect();

        let mut this = Self { total_kills, players, kills, kills_by_means };

        for kill in &game.kills {
            match kill.killer {
                Killer::World => {
                    let player_name = &game.players[&kill.target];
                    *this.kills.get_mut(player_name).unwrap() -= 1;
                },
                Killer::Player(killer_id) => {
                    let player_name = &game.players[&killer_id];
                    *this.kills.get_mut(player_name).unwrap() += 1;
                },
            }
            *this.kills_by_means.get_mut(&kill.means).unwrap() += 1;
        }

        this
    }
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct LogReport {
    pub games: IndexMap<GameName, GameReport>,
}

impl LogReport {
    pub fn generate<I, E>(game_iter: I) -> Result<Self, E>
    where
        I: IntoIterator<Item = Result<Game, E>>,
    {
        let mut this = Self { games: IndexMap::new() };
        for (i, result) in game_iter.into_iter().enumerate() {
            let game = result?;
            let game_report = GameReport::generate(&game);
            let game_id = format!("game_{}", i + 1);
            this.games.insert(game_id, game_report);
        }
        Ok(this)
    }
}
