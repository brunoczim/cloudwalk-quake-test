use std::collections::{HashMap, HashSet};

pub type KillCount = i64;

pub type GameName = String;

pub type PlayerName = String;

pub type MeanOfKilling = String;

#[derive(Debug, Clone, Default, serde::Serialize)]
pub struct Game {
    pub total_kills: KillCount,
    pub players: HashSet<PlayerName>,
    pub kills: HashMap<PlayerName, KillCount>,
    pub kills_by_means: HashMap<MeanOfKilling, KillCount>,
}

#[derive(Debug, Clone, Default, serde::Serialize)]
pub struct Report {
    pub games: HashMap<GameName, Game>,
}
