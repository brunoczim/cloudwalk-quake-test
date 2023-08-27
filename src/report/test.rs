use super::GameReport;
use crate::game::{
    Game,
    Kill,
    Killer,
    MeansOfKilling,
    PlayerName,
    MEANS_OF_KILLING,
};
use indexmap::{IndexMap, IndexSet};
use std::collections::HashMap;

fn game_1() -> Game {
    Game {
        players: HashMap::from([(2, PlayerName::from("Isgalamido"))]),
        kills: Vec::new(),
    }
}

fn game_2() -> Game {
    Game {
        players: HashMap::from([
            (2, PlayerName::from("Dono da Bola")),
            (3, PlayerName::from("Isgalamido")),
            (4, PlayerName::from("Zeh")),
        ]),
        kills: vec![
            Kill {
                killer: Killer::World,
                target: 3,
                means: MeansOfKilling::from("MOD_TRIGGER_HURT"),
            },
            Kill {
                killer: Killer::World,
                target: 2,
                means: MeansOfKilling::from("MOD_FALLING"),
            },
            Kill {
                killer: Killer::World,
                target: 3,
                means: MeansOfKilling::from("MOD_FALLING"),
            },
            Kill {
                killer: Killer::Player(2),
                target: 4,
                means: MeansOfKilling::from("MOD_ROCKET"),
            },
        ],
    }
}

fn game_report_1() -> GameReport {
    GameReport {
        total_kills: 0,
        players: IndexSet::from([PlayerName::from("Isgalamido")]),
        kills: IndexMap::from([(PlayerName::from("Isgalamido"), 0)]),
        kills_by_means: MEANS_OF_KILLING
            .iter()
            .copied()
            .map(|means| (MeansOfKilling::from(means), 0))
            .collect(),
    }
}

fn game_report_2() -> GameReport {
    GameReport {
        total_kills: 4,
        players: IndexSet::from([
            PlayerName::from("Dono da Bola"),
            PlayerName::from("Isgalamido"),
            PlayerName::from("Zeh"),
        ]),
        kills: IndexMap::from([
            (PlayerName::from("Dono da Bola"), 0),
            (PlayerName::from("Isgalamido"), -2),
            (PlayerName::from("Zeh"), 0),
        ]),
        kills_by_means: MEANS_OF_KILLING
            .iter()
            .copied()
            .map(|means| {
                let count = match means {
                    "MOD_TRIGGER_HURT" => 1,
                    "MOD_FALLING" => 2,
                    "MOD_ROCKET" => 1,
                    _ => 0,
                };
                (MeansOfKilling::from(means), count)
            })
            .collect(),
    }
}

#[test]
fn generate_game_report_1() {
    let expected = game_report_1();
    let actual = GameReport::generate(&game_1());
    assert_eq!(expected, actual);
}

#[test]
fn generate_game_report_2() {
    let expected = game_report_2();
    let actual = GameReport::generate(&game_2());
    assert_eq!(expected, actual);
}
