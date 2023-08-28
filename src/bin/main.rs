use clap::Parser as CliParser;
use quake_log_parser::{error::Result, parser::Parser, report::LogReport};
use simplelog::{Config, WriteLogger};
use std::{
    fs::{File, OpenOptions},
    io,
    path::PathBuf,
    process::exit,
};

#[derive(Debug, Clone, CliParser)]
struct Arguments {
    #[arg(default_value = "qgames.log")]
    input_path: PathBuf,
    #[arg(short = 'l', long = "app-log", default_value = "script.log")]
    log_path: PathBuf,
}

fn try_main() -> Result<()> {
    let args = Arguments::parse();

    let script_log_file = OpenOptions::new()
        .read(false)
        .write(true)
        .append(true)
        .create(true)
        .open(&args.log_path)?;
    WriteLogger::init(
        log::LevelFilter::Warn,
        Config::default(),
        script_log_file,
    )?;

    let quake_file = File::open(&args.input_path)?;
    let parser = Parser::new(quake_file);
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
