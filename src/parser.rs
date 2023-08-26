use crate::data::{Game, Report};
use std::{
    io::{self, BufRead, BufReader, Read},
    mem,
};

pub fn parse<R>(reader: R) -> io::Result<Report>
where
    R: Read,
{
    let mut parser = Parser::new(reader);
    loop {
        if let Err(error) = parser.read_line() {
            let result = match error.kind() {
                io::ErrorKind::UnexpectedEof => Ok(parser.report),
                _ => Err(error),
            };
            break result;
        }

        parser.process_line();
    }
}

#[derive(Debug, Clone, Copy)]
struct RawEvent<'line> {
    key: &'line str,
    raw_data: &'line str,
}

impl<'line> RawEvent<'line> {
    fn from_line(line: &'line str) -> Option<Self> {
        let mut split_index = line.find(char::is_alphabetic)?;
        let event_str = &line[split_index ..];
        let (key, raw_data) = event_str.split_once(':')?;
        Some(Self { key: key.trim(), raw_data: raw_data.trim() })
    }

    fn parse(self) -> Option<Event> {
        match self.key {
            "InitGame" => Some(Event::Init),
            "ShutdownGame" => Some(Event::Shutdown),
            _ => None,
        }
    }
}

#[derive(Debug)]
enum Event {
    Init,
    Shutdown,
}

#[derive(Debug, Clone)]
enum State {
    NoGame,
    InGame(Game),
}

impl Default for State {
    fn default() -> Self {
        Self::NoGame
    }
}

#[derive(Debug)]
struct Parser<R> {
    reader: BufReader<R>,
    line_buf: String,
    game_id: u64,
    report: Report,
    state: State,
}

impl<R> Parser<R>
where
    R: Read,
{
    fn new(reader: R) -> Self {
        Self {
            reader: BufReader::new(reader),
            line_buf: String::new(),
            game_id: 1,
            report: Report::default(),
            state: State::NoGame,
        }
    }

    fn read_line(&mut self) -> io::Result<()> {
        self.line_buf.clear();
        self.reader.read_line(&mut self.line_buf)?;
        Ok(())
    }

    fn process_line(&mut self) {
        if let Some(event) =
            RawEvent::from_line(&self.line_buf).and_then(RawEvent::parse)
        {
            match event {
                Event::Init => {
                    self.finish_game();
                    self.start_game();
                },
                Event::Shutdown => self.finish_game(),
            }
        }
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
        self.state = State::InGame(Game::default());
    }
}
