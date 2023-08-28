# The "Project"

It is a Rust library that implements parsing and data grouping of
Quake III: Arena logs.

# The "Script"

It is a Rust binary implemented in `src/bin/main.rs`. It uses the library to
extract the data from the log and prints it as JSON.

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

This is a small architecture, but consists essentially of three parts:

- Common data, can be interpreted as equivalent to the "domain" in DDD;
- Parser utilities, for parsing the log;
- Common data grouping into report data.

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
depends only on common ddata module as well.

### Avoid Allocating Memory At Once For The Whole File

The quake log file is relatively big, and could be even bigger. If one would
read the whole file and then process it, lots of memory would be unnecessarily
used. So, the library reads one line at a time, which is a unit in the quake
log format. Of course, this could be inefficient in terms of processing time,
but a buffered reader was used, which reads a chunk of bytes big enough to be
time efficient.

Besides that, one could still use unnecessary memory (and iterate twice) by
collecting all games from the log file into a vector and only then processing
this vector into another structure (the report). Rather than that, the parser
was coded to be an iterator of games. If one wants to generate a report from
parser output, it should feed the iterator to `LogReport::generate` function.

### Tradeoff Between Library Usability And Efficiency

A few tradeoffs between library usability and efficiency have been made
in favor of library usability. For instance, instead of having a `LogReport`
structure with all `GameReport`s, with support for `serde_json` library, one
could have programmed this library with a function that generates the JSON
string manually, or at least parts of it. However, that could be judged as
overfitting the library with the "script" and could lead to loss of
flexibility.

## Script

Script should have as little as code possible, only glueing different
parts of the library (and other libraries). The exception is executable-specific
logic, such as CLI argument parsing.
