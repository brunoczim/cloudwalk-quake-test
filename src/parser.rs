use crate::report::{FinishedGame, Report};
use std::{
    collections::HashMap,
    io::{self, BufRead, BufReader, Read},
    mem,
};

pub fn parse<R>(reader: R) -> io::Result<Report>
where
    R: Read,
{
    let mut buf_reader = BufReader::new(reader);
    let mut line_buf = String::new();
    let mut parser = Parser::new();
    loop {
        line_buf.clear();

        if let Err(error) = buf_reader.read_line(&mut line_buf) {
            let result = match error.kind() {
                io::ErrorKind::UnexpectedEof => Ok(parser.finish()),
                _ => Err(error),
            };
            return result;
        }

        parser.process_line(&line_buf);
    }
}

#[derive(Debug, Clone, Copy)]
struct RawEvent<'line> {
    key: &'line str,
    raw_data: &'line str,
}

impl<'line> RawEvent<'line> {
    fn from_line(line: &'line str) -> Option<Self> {
        let split_index = line.find(char::is_alphabetic)?;
        let event_str = &line[split_index ..];
        let (key, raw_data) = event_str.split_once(':')?;
        Some(Self { key: key.trim(), raw_data: raw_data.trim() })
    }

    fn parse(self) -> Option<Event<'line>> {
        match self.key {
            "InitGame" => Some(Event::Init),
            "ShutdownGame" => Some(Event::Shutdown),
            "ClientUserinfoChanged" => {
                let (_, name_trailing) = self.raw_data.split_once("n\\")?;
                let name = match name_trailing.split_once("t\\") {
                    Some((name, _)) => name,
                    None => name_trailing,
                };
                Some(Event::ClientChanged { name })
            },
            _ => None,
        }
    }
}

#[derive(Debug)]
enum Event<'line> {
    Init,
    Shutdown,
    ClientChanged { name: &'line str },
}

#[derive(Debug, Clone)]
enum State {
    NoGame,
    InGame(FinishedGame),
}

impl Default for State {
    fn default() -> Self {
        Self::NoGame
    }
}

#[derive(Debug)]
struct Parser {
    game_id: u64,
    report: Report,
    state: State,
}

impl Parser {
    fn new() -> Self {
        Self { game_id: 1, report: Report::default(), state: State::NoGame }
    }

    fn process_line(&mut self, line: &str) {
        if let Some(event) = RawEvent::from_line(line).and_then(RawEvent::parse)
        {
            match event {
                Event::Init => {
                    self.finish_game();
                    self.start_game();
                },
                Event::Shutdown => self.finish_game(),
                Event::ClientChanged { name } => {
                    self.touch_player(name);
                },
            }
        }
    }

    fn finish(mut self) -> Report {
        self.finish_game();
        self.report
    }

    fn finish_game(&mut self) {
        match mem::take(&mut self.state) {
            State::InGame(game) => {
                let game_name = format!("game_{}", self.game_id);
                self.report.games.insert(game_name, game);
                self.game_id += 1;
            },
            State::NoGame => (),
        }
    }

    fn start_game(&mut self) {
        self.state = State::InGame(FinishedGame::default());
    }

    fn touch_player(&mut self, name: &str) {
        if let State::InGame(game) = &mut self.state {
            if !game.players.contains(name) {
                game.players.insert(String::from(name));
                game.kills.insert(String::from(name), 0);
            }
        }
    }
}
