//! This module exposes a Quake III: Arena log file parser.

use crate::{
    error::Result,
    game::{
        all_means_of_death,
        Game,
        Kill,
        Killer,
        MeansOfDeath,
        PlayerId,
        PlayerName,
    },
};
use std::{
    collections::hash_map,
    io::{self, BufRead, BufReader, Read},
    mem,
};

#[cfg(test)]
mod test;

/// Parser of Quake III: Arena log file. It works as an iterator. It reads a
/// file's line by line, and whenever a game is finished, it is yielded through
/// the iterator.
#[derive(Debug)]
pub struct Parser<R> {
    /// Where log data will be read from.
    reader: R,
    /// The buffer for the line, reused every time.
    line_buf: String,
    /// Parser state, fed with lines from the reader.
    state: State,
}

impl<R> Parser<BufReader<R>>
where
    R: Read,
{
    /// Creates the parser from a reader object (typically a file), wrapping it
    /// with a buffered reader.
    pub fn new(reader: R) -> Self {
        Self::with_bufread(BufReader::new(reader))
    }
}

impl<R> Parser<R>
where
    R: BufRead,
{
    /// Creates the parser from a buffered reader object (typically wrapping a
    /// file or equivalent).
    pub fn with_bufread(reader: R) -> Self {
        Self { reader, line_buf: String::new(), state: State::NoGame }
    }

    /// Finishes the parser when the file reaches its end. If a game was active,
    /// it is returned.
    fn finish(&mut self) -> Option<Game> {
        self.state.finish_game()
    }

    /// Reads a line from the underlying reader and returns whether a line was
    /// read (`true` = "read a line", `false` = "EOF reached").
    fn read_line(&mut self) -> Result<bool> {
        self.line_buf.clear();

        match self.reader.read_line(&mut self.line_buf) {
            Ok(0) => Ok(false),
            Err(error) if error.kind() == io::ErrorKind::UnexpectedEof => {
                Ok(false)
            },
            Err(error) => Err(error.into()),
            _ => Ok(true),
        }
    }

    /// Processes a line previously read from the underlying reader by feeding
    /// the line to the state.
    ///
    /// If a game is finished now, it is returned.
    fn process_line(&mut self) -> Option<Game> {
        self.state.process_line(&self.line_buf)
    }
}

impl<R> Iterator for Parser<R>
where
    R: BufRead,
{
    type Item = Result<Game>;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            match self.read_line() {
                Ok(false) => return self.finish().map(Ok),
                Ok(true) => (),
                Err(error) => return Some(Err(error)),
            }

            if let Some(game) = self.process_line() {
                return Some(Ok(game));
            }
        }
    }
}

/// Raw structure for an event that occurs in the log file.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct RawEvent<'line> {
    /// Key of the event, such as "InitGame".
    key: &'line str,
    /// Raw representation of the event's payload.
    raw_data: &'line str,
}

impl<'line> RawEvent<'line> {
    /// Tries to split key and data from a line of the file. If this line does
    /// not represent an event or is not valid for an event, `None` is
    /// returned.
    fn from_line(line: &'line str) -> Option<Self> {
        let split_index = line.find(char::is_alphabetic)?;
        let event_str = &line[split_index ..];
        let (key, raw_data) = event_str.split_once(':')?;
        Some(Self { key: key.trim(), raw_data: raw_data.trim() })
    }

    /// Parses the raw event representation into a structured event. If the
    /// event key represents an unused event, it is ignored, if the event
    /// data does not seem to be valid, it is ignored as well (ignored =
    /// returns `None`).
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
                let (killer_id_str, tail) =
                    self.raw_data.trim().split_once(' ')?;
                let (target_id_str, tail) = tail.trim().split_once(' ')?;
                let (_, tail) = tail.trim().split_once(':')?;
                let killer = if tail.trim().starts_with("<world> killed") {
                    Killer::World
                } else {
                    Killer::Player(killer_id_str.trim().parse().ok()?)
                };
                let target = target_id_str.trim().parse().ok()?;
                let (_, mean_str) = tail.rsplit_once("by ")?;
                let (_, means) =
                    all_means_of_death().get_full(mean_str.trim())?;
                Some(Event::Kill { killer, target, means })
            },

            _ => None,
        }
    }
}

/// A structured event representation.
#[derive(Debug, Clone, PartialEq, Eq)]
enum Event {
    /// Game initialization.
    Init,
    /// Game shutdown.
    Shutdown,
    /// A name of the player with the given ID has changed to the given name.
    PlayerNameChanged { id: PlayerId, name: PlayerName },
    /// Someone (target) was killed (by the killer) with the given means of
    /// death.
    Kill { killer: Killer, target: PlayerId, means: MeansOfDeath },
}

/// State of the parser.
#[derive(Debug, Clone)]
enum State {
    /// No game currently active.
    NoGame,
    /// A game is currently active and will be eventually yielded.
    InGame(Game),
}

impl Default for State {
    /// The default initial state, and also the state set using `mem::take`.
    fn default() -> Self {
        Self::NoGame
    }
}

impl State {
    /// Processes a line from the file, reacting to it.
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

    /// Finishes the current game, if any. This will return the game and change
    /// the state to `NoGame`.
    fn finish_game(&mut self) -> Option<Game> {
        match mem::take(self) {
            State::InGame(game) => Some(game),
            State::NoGame => None,
        }
    }

    /// Starts a new empty game as the current state. This assumes that the
    /// current state is `NoGame`.
    fn start_game(&mut self) {
        *self = State::InGame(Game::default());
    }

    /// Reacts to the event of a player changing its name. If the player was not
    /// accounted yet, an entry for them will be created.
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

    /// Reacts to a `Kill` event by pushing it into the kill list.
    fn kill(&mut self, killer: Killer, target: PlayerId, means: MeansOfDeath) {
        if let State::InGame(game) = self {
            game.kills.push(Kill { killer, target, means });
        }
    }
}
