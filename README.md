# The "Project"

The "Project" is a Rust library that implements parsing and data grouping of
Quake III: Arena logs.

# The "Script"

The "Script" is a Rust binary implemented in `src/bin/main.rs`. It uses the
library to extract the data from the log and prints it as JSON.

## Usage

Usage can be seen with `--help` argument to the CLI program.

By default, the log path is `./qgames.log`, but it can be given by a positional
argument. The report is printed to the stdout as JSON.

Example usage (with default path): `$ cargo run`
Example usage (with given log path): `$ cargo run -- myfile.log`
Example usage (requesting help): `$ cargo run -- --help`

# Design Principles

## Library

### Architecture

The library uses a small architecture, but consists essentially of three parts:

- Common data, can be interpreted as equivalent to the "domain" in DDD;
- Parser utilities, for parsing the log;
- Report generation from common data.

### Independence From The "Script"

The library has the "script" as its use-case, but it should not be overfit with
the "script". It still needs to be flexible.

### No Circular Dependencies Between Modules

Circular dependencies between modules make them coupled, harder to refactor,
and increase complexity of navigating through the code and understanding the
achitecture. Therefore, this project will break any circular dependency between
two (or more) modules. As a consequence, items (such as functions) causing the
circular dependency will be moved to a different module.

### Dependency Inversion

In a more naive implementation, the report generation module could depend on the
parser module, but in order to break this dependency and make these modules more
independent, report generation was coded to depend on an iterator over game data
`Iterator<Item = Result<Game>>` and not on the parser. In the end, the parser
module depends only on common data module, and the report generation module
depends only on common data module as well.

### Type Aliases for Bare Datatypes With Extra Semantics

Primitives and datatypes such as `String`, when used consistently with a given
semantics, are generally not referenced directly. Instead, they are given a type
alias with a specific name and then they are referenced using this type alias.

### Avoid Allocating Memory At Once For The Whole File

The quake log file is relatively big, and could be even bigger. If one would
read the whole file and then process it, lots of memory would be unnecessarily
used. So, the library reads one line at a time. Of course, this could be
inefficient in terms of I/O, but a buffered reader is used, which reads a chunk
of bytes big enough to be I/O efficient.

Besides that, one could still use unnecessary memory (and thus iterate twice) by
collecting all games from the log file into a vector and then processing this
vector into another structure (the report). Rather than that, the parser
was coded to be an iterator of games. If one wants to generate a report from
parser output, it should feed the iterator to `LogReport::generate` function.

### Tradeoff Between Library Usability And Efficiency

A few tradeoffs between library usability and efficiency have been made
(in favor of library usability). For instance, instead of having a `LogReport`
structure with all `GameReport`s (easily rendered with `serde_json` library),
one could iterate through the `Game`s yielded by the parser and generate the
JSON string manually (or at least parts of it). However, that wouldn't be very
flexible and could be judged as overfitting the library with the "script".

### Using Enum for MODs (Means of Death) vs. Using Strings

Using `enum` to represent MODs was considered but it was not chosen for a few
reasons. In first place, Quake's source code has some enum cases which are
conditioned by the presence of `MISSIONPACK`, and since `MOD_GRAPPLE` comes
after them, `MOD_GRAPPLE`'s integer value could vary.

Therefore, one cannot convert a MOD's code from the log directly into a Rust
`enum`, because some integer values vary. Because of that, using an `enum` would
imply in some scheme for converting strings to enums and vice-versa, and that
would increase development complexity compared to just using bare string
literals.

Enums have some advantages over bare strings, indeed, but currently none of them
have been required to deal with MOD in this project. One could worry about
strings being expensive, but static string literals are cheap to copy.
Therefore, using bare string literals sounds like a better alternative.

### Log Parsing Strategy

1. Do not stop parsing the whole file because of a single line:
    Logs do not have a well-defined syntax, it is possible that an event might
    be misidentified by the parser, and even if something is indeed wrong,
    it is better to extract data in a best-effort manner.
2. Do not attempt to parse what is not necessary.
3. Use player ID to track a player, since the player can change its name.
4. In the `Kill` event:
    1. Do not assume a fixed ID (such as `1022`) for the `<world>` as the
        killer. Instead look for the string `"<world>"`.
    2. If the killer is not `<world>`, it is safe to the to use the killer ID
        as a player ID.
    3. The target is always a player, it easier to use identified it by its ID.
    4. MOD integer value might not always be the same because of `MISSIONPACK`,
        do not use the integer value. Instead, get the MOD string.
5. If an event `InitGame` happens while another game is active, shutdown the 
    active game immediately and start a new game.
6. If an in-game event appears but no game is active, ignore it.
7. If a player never has their name mentioned, ignore them.

### Logging Instead of Panicking

There are some codepaths that are judged unreachable, such that reaching it
would be a bug. Instead of stopping the application and panicking, logging such
"impossible" scenario and keeping the application running is preferred.

### Map and Set Key Order

`HashMap` and `HashSet` do not have any order for their keys/elements.
`BTreeMap` and `BTreeSet` use lexicographic order for their keys/elements.
However, having insertion order was considered desirable for a prettier JSON,
and so, the `indexmap` library is used.

## Script

Script should have as little as code possible, only glueing different
parts of the library (and other libraries). The exception is executable-specific
logic, such as CLI argument parsing.

## Tests

The Library is tested with unit tests covering as much code as possible. Not
only that, tests use data from an actual Quake III: Arena log file.
