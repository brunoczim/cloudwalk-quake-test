use clap::Parser as CliParser;
use quake_log_parser::{error::Result, parser::Parser, report::LogReport};
use std::{fs::File, io, path::PathBuf, process::exit};

#[derive(Debug, Clone, CliParser)]
struct Arguments {
    #[arg(default_value = "qgames.log")]
    input_path: PathBuf,
}

fn try_main() -> Result<()> {
    let args = Arguments::parse();
    let file = File::open(&args.input_path)?;
    let parser = Parser::new(file);
    let report = LogReport::generate(parser)?;
    serde_json::to_writer_pretty(io::stdout(), &report)?;
    Ok(())
}

fn main() {
    if let Err(error) = try_main() {
        eprintln!("{}", error);
        exit(1);
    }
}
