use super::{Event, Parser, RawEvent, State};
use crate::{
    error::Result,
    game::{Game, Kill, Killer, MeansOfDeath, PlayerName},
};
use std::collections::HashMap;

const SMALL_LOG: &str = concat!(
    "  0:00 ------------------------------------------------------------\n",
    "  0:00 InitGame: \
     \\sv_floodProtect\\1\\sv_maxPing\\0\\sv_minPing\\0\\sv_maxRate\\10000\\\
     sv_minRate\\0\\sv_hostname\\Code Miner \
     Server\\g_gametype\\0\\sv_privateClients\\2\\sv_maxclients\\16\\\
     sv_allowDownload\\0\\dmflags\\0\\fraglimit\\20\\timelimit\\15\\\
     g_maxGameClients\\0\\capturelimit\\8\\version\\ioq3 1.36 linux-x86_64 \
     Apr 12 2009\\protocol\\68\\mapname\\q3dm17\\gamename\\baseq3\\g_needpass\\
     \
     \0\n",
    " 15:00 Exit: Timelimit hit.\n",
    " 20:34 ClientConnect: 2\n",
    " 20:34 ClientUserinfoChanged: 2 \
     n\\Isgalamido\\t\\0\\model\\xian/default\\hmodel\\xian/default\\\
     g_redteam\\\\g_blueteam\\\\c1\\4\\c2\\5\\hc\\100\\w\\0\\l\\0\\tt\\0\\tl\\\
     0\n",
    " 20:37 ClientUserinfoChanged: 2 \
     n\\Isgalamido\\t\\0\\model\\uriel/zael\\hmodel\\uriel/zael\\g_redteam\\\\\
     g_blueteam\\\\c1\\5\\c2\\5\\hc\\100\\w\\0\\l\\0\\tt\\0\\tl\\0\n",
    " 20:37 ClientBegin: 2\n",
    " 20:37 ShutdownGame:\n",
    " 20:37 ------------------------------------------------------------\n",
    "  1:47 ------------------------------------------------------------\n",
    "  1:47 InitGame: \
     \\sv_floodProtect\\1\\sv_maxPing\\0\\sv_minPing\\0\\sv_maxRate\\10000\\\
     sv_minRate\\0\\sv_hostname\\Code Miner \
     Server\\g_gametype\\0\\sv_privateClients\\2\\sv_maxclients\\16\\\
     sv_allowDownload\\0\\bot_minplayers\\0\\dmflags\\0\\fraglimit\\20\\\
     timelimit\\15\\g_maxGameClients\\0\\capturelimit\\8\\version\\ioq3 1.36 \
     linux-x86_64 Apr 12 \
     2009\\protocol\\68\\mapname\\q3dm17\\gamename\\baseq3\\g_needpass\\0\n",
    "  1:47 ClientConnect: 2\n",
    "  1:47 ClientUserinfoChanged: 2 n\\Dono da \
     Bola\\t\\0\\model\\sarge\\hmodel\\sarge\\g_redteam\\\\g_blueteam\\\\c1\\\
     4\\c2\\5\\hc\\95\\w\\0\\l\\0\\tt\\0\\tl\\0\n",
    "  1:47 ClientBegin: 2\n",
    "  1:47 ClientConnect: 3\n",
    "  1:47 ClientUserinfoChanged: 3 \
     n\\Isgalamido\\t\\0\\model\\uriel/zael\\hmodel\\uriel/zael\\g_redteam\\\\\
     g_blueteam\\\\c1\\5\\c2\\5\\hc\\100\\w\\0\\l\\0\\tt\\0\\tl\\0\n",
    "  1:47 ClientBegin: 3\n",
    "  1:47 ClientConnect: 4\n",
    "  1:47 ClientUserinfoChanged: 4 \
     n\\Zeh\\t\\0\\model\\sarge/default\\hmodel\\sarge/default\\g_redteam\\\\\
     g_blueteam\\\\c1\\1\\c2\\5\\hc\\100\\w\\0\\l\\0\\tt\\0\\tl\\0\n",
    "  1:47 ClientBegin: 4\n",
    "  1:48 Item: 4 ammo_rockets\n",
    "  1:48 Item: 4 weapon_rocketlauncher\n",
    "  1:51 Item: 3 item_armor_shard\n",
    "  1:51 Item: 3 item_armor_shard\n",
    "  1:51 Item: 3 item_armor_shard\n",
    "  1:51 Item: 3 item_armor_combat\n",
    "  1:54 Item: 3 weapon_rocketlauncher\n",
    "  1:54 Item: 3 ammo_rockets\n",
    "  1:57 Item: 2 weapon_rocketlauncher\n",
    "  2:00 Kill: 1022 3 22: <world> killed Isgalamido by MOD_TRIGGER_HURT\n",
    "  2:02 Item: 3 weapon_rocketlauncher\n",
    "  2:04 Kill: 1022 2 19: <world> killed Dono da Bola by MOD_FALLING\n",
    "  2:04 Item: 4 item_armor_body\n",
    "  2:04 Kill: 1022 3 19: <world> killed Isgalamido by MOD_FALLING\n",
    "  2:07 Item: 2 weapon_rocketlauncher\n",
    "  2:11 Kill: 2 4 6: Dono da Bola killed Zeh by MOD_ROCKET\n",
    " 12:13 ShutdownGame:\n",
);

fn expected_games_from_small_log() -> Vec<Game> {
    vec![
        Game {
            players: HashMap::from([(2, PlayerName::from("Isgalamido"))]),
            kills: Vec::new(),
        },
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
                    means: MeansOfDeath::from("MOD_TRIGGER_HURT"),
                },
                Kill {
                    killer: Killer::World,
                    target: 2,
                    means: MeansOfDeath::from("MOD_FALLING"),
                },
                Kill {
                    killer: Killer::World,
                    target: 3,
                    means: MeansOfDeath::from("MOD_FALLING"),
                },
                Kill {
                    killer: Killer::Player(2),
                    target: 4,
                    means: MeansOfDeath::from("MOD_ROCKET"),
                },
            ],
        },
    ]
}

#[test]
fn split_raw_event_easy_case() {
    let line = " 20:34 ClientConnect: 2\n";
    let expected = Some(RawEvent { key: "ClientConnect", raw_data: "2" });
    let actual = RawEvent::from_line(line);
    assert_eq!(expected, actual);
}

#[test]
fn split_raw_event_single_digit_hour() {
    let line = "  1:48 Item: 4 ammo_rockets\n";
    let expected = Some(RawEvent { key: "Item", raw_data: "4 ammo_rockets" });
    let actual = RawEvent::from_line(line);
    assert_eq!(expected, actual);
}

#[test]
fn split_raw_event_filler() {
    let line =
        " 20:37 ------------------------------------------------------------\n";
    let expected = None;
    let actual = RawEvent::from_line(line);
    assert_eq!(expected, actual);
}

#[test]
fn split_raw_event_corrupted_line() {
    let line =
        " 26  0:00 ------------------------------------------------------------\n";
    let expected = None;
    let actual = RawEvent::from_line(line);
    assert_eq!(expected, actual);
}

#[test]
fn parse_init_game() {
    let line = "  0:00 InitGame: \
                \\\\capturelimit\\\\8\\\\g_maxGameClients\\\\0\\\\timelimit\\\\
                \
                \\15\\\\fraglimit\\\\20\\\\dmflags\\\\0\\\\sv_allowDownload\\\
                \
                \\0\\\\sv_maxclients\\\\16\\\\sv_privateClients\\\\2\\\\\
                g_gametype\\\\0\\\\sv_hostname\\\\Code Miner \
                Server\\\\sv_minRate\\\\0\\\\sv_maxRate\\\\10000\\\\\
                sv_minPing\\\\0\\\\sv_maxPing\\\\0\\\\sv_floodProtect\\\\1\\\\\
                version\\\\ioq3 1.36 linux-x86_64 Apr 12 \
                2009\\\\protocol\\\\68\\\\mapname\\\\q3dm17\\\\gamename\\\\\
                baseq3\\\\g_needpass\\\\0\n";
    let expected = Some(Event::Init);
    let actual = RawEvent::from_line(line).unwrap().parse();
    assert_eq!(expected, actual);
}

#[test]
fn parse_shutdown_game() {
    let line = " 54:21 ShutdownGame:\n";
    let expected = Some(Event::Shutdown);
    let actual = RawEvent::from_line(line).unwrap().parse();
    assert_eq!(expected, actual);
}

#[test]
fn parse_player_name_changed() {
    let line = "  0:07 ClientUserinfoChanged: 2 n\\Fasano \
                Again\\t\\0\\model\\razor/id\\hmodel\\razor/id\\g_redteam\\\\\
                g_blueteam\\\\c1\\3\\c2\\5\\hc\\100\\w\\0\\l\\0\\tt\\0\\tl\\0\\
                \
                \n";
    let expected = Some(Event::PlayerNameChanged {
        id: 2,
        name: PlayerName::from("Fasano Again"),
    });
    let actual = RawEvent::from_line(line).unwrap().parse();
    assert_eq!(expected, actual);
}

#[test]
fn parse_kill_by_player() {
    let line = "  0:25 Kill: 2 4 6: Oootsimo killed Zeh by MOD_ROCKET\n";
    let expected = Some(Event::Kill {
        killer: Killer::Player(2),
        target: 4,
        means: MeansOfDeath::from("MOD_ROCKET"),
    });
    let actual = RawEvent::from_line(line).unwrap().parse();
    assert_eq!(expected, actual);
}

#[test]
fn parse_kill_by_world() {
    let line = " 15:27 Kill: 1022 5 22: <world> killed Assasinu Credi by \
                MOD_TRIGGER_HURT\n";
    let expected = Some(Event::Kill {
        killer: Killer::World,
        target: 5,
        means: MeansOfDeath::from("MOD_TRIGGER_HURT"),
    });
    let actual = RawEvent::from_line(line).unwrap().parse();
    assert_eq!(expected, actual);
}

#[test]
fn parse_irrelevant() {
    let line = " 15:43 Item: 2 weapon_shotgun\n";
    let expected = None;
    let actual = RawEvent::from_line(line).unwrap().parse();
    assert_eq!(expected, actual);
}

#[test]
fn state_parse_games() {
    let expected = expected_games_from_small_log();

    let mut state = State::default();
    let mut actual_games = Vec::new();
    for line in SMALL_LOG.lines() {
        if let Some(game) = state.process_line(line) {
            actual_games.push(game);
        }
    }
    if let Some(game) = state.finish_game() {
        actual_games.push(game);
    }

    assert_eq!(expected, actual_games);
}

#[test]
fn actual_parse_games() {
    let expected = expected_games_from_small_log();

    let result: Result<Vec<_>> =
        Parser::new(&mut SMALL_LOG.as_bytes()).collect();

    let actual = result.unwrap();

    assert_eq!(expected, actual);
}
