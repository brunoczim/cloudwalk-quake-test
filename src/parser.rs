use crate::{
    error::Result,
    game::{
        Game,
        Kill,
        Killer,
        MeansOfKilling,
        PlayerId,
        PlayerName,
        MEANS_OF_KILLING,
    },
};
use std::{
    collections::hash_map,
    io::{self, BufRead, BufReader, Read},
    mem,
};

#[cfg(test)]
mod test;

#[derive(Debug)]
pub struct Parser<R> {
    reader: R,
    line_buf: String,
    state: State,
}

impl<R> Parser<BufReader<R>>
where
    R: Read,
{
    pub fn new(reader: R) -> Self {
        Self::with_bufread(BufReader::new(reader))
    }
}

impl<R> Parser<R>
where
    R: BufRead,
{
    pub fn with_bufread(reader: R) -> Self {
        Self { reader, line_buf: String::new(), state: State::NoGame }
    }

    fn finish(&mut self) -> Option<Game> {
        self.state.finish_game()
    }
}

impl<R> Iterator for Parser<R>
where
    R: BufRead,
{
    type Item = Result<Game>;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            self.line_buf.clear();

            match self.reader.read_line(&mut self.line_buf) {
                Ok(0) => return self.finish().map(Ok),
                Err(error) if error.kind() == io::ErrorKind::UnexpectedEof => {
                    return self.finish().map(Ok)
                },
                Err(error) => return Some(Err(error.into())),
                _ => (),
            }

            if let Some(game) = self.state.process_line(&self.line_buf) {
                return Some(Ok(game));
            }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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

    fn parse(self) -> Option<Event> {
        match self.key {
            "InitGame" => Some(Event::Init),

            "ShutdownGame" => Some(Event::Shutdown),

            "ClientUserinfoChanged" => {
                let (id_str, tail) = self.raw_data.trim().split_once(' ')?;
                let id = id_str.trim().parse().ok()?;
                let (_, name_trailing) = tail.split_once("n\\")?;
                let name = match name_trailing.split_once("\\") {
                    Some((name, _)) => name,
                    None => name_trailing,
                };
                Some(Event::PlayerNameChanged {
                    id,
                    name: PlayerName::from(name),
                })
            },

            "Kill" => {
                let (killer_str, tail) =
                    self.raw_data.trim().split_once(' ')?;
                let (target_str, tail) = tail.trim().split_once(' ')?;
                let (mean_str, _) = tail.trim().split_once(':')?;
                let killer = Killer::from_id(killer_str.trim().parse().ok()?);
                let target = target_str.trim().parse().ok()?;
                let mean_index: usize = mean_str.parse().ok()?;
                let means = MeansOfKilling::from(MEANS_OF_KILLING[mean_index]);
                Some(Event::Kill { killer, target, means })
            },

            _ => None,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Event {
    Init,
    Shutdown,
    PlayerNameChanged { id: PlayerId, name: PlayerName },
    Kill { killer: Killer, target: PlayerId, means: MeansOfKilling },
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

impl State {
    fn process_line(&mut self, line: &str) -> Option<Game> {
        let event = RawEvent::from_line(line).and_then(RawEvent::parse)?;
        match event {
            Event::Init => {
                let maybe_game = self.finish_game();
                self.start_game();
                maybe_game
            },
            Event::Shutdown => self.finish_game(),
            Event::PlayerNameChanged { id, name } => {
                self.change_player_name(id, name);
                None
            },
            Event::Kill { killer, target, means } => {
                self.kill(killer, target, means);
                None
            },
        }
    }

    fn finish_game(&mut self) -> Option<Game> {
        match mem::take(self) {
            State::InGame(game) => Some(game),
            State::NoGame => None,
        }
    }

    fn start_game(&mut self) {
        *self = State::InGame(Game::default());
    }

    fn change_player_name(&mut self, id: PlayerId, name: PlayerName) {
        if let State::InGame(game) = self {
            match game.players.entry(id) {
                hash_map::Entry::Occupied(mut entry) => {
                    *entry.get_mut() = name;
                },
                hash_map::Entry::Vacant(entry) => {
                    entry.insert(name);
                },
            }
        }
    }

    fn kill(
        &mut self,
        killer: Killer,
        target: PlayerId,
        means: MeansOfKilling,
    ) {
        if let State::InGame(game) = self {
            game.kills.push(Kill { killer, target, means });
        }
    }
}
