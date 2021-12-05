use std::fs::read_to_string;
use std::path::PathBuf;

use color_eyre::Report;
use structopt::StructOpt;
use tracing::debug;
use tracing_subscriber::EnvFilter;

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;

#[derive(Debug, StructOpt)]
#[structopt()]
struct Options {
    #[structopt(short, long)]
    debug: bool,

    #[structopt(short, long)]
    puzzle: u32,

    #[structopt(short, long, parse(from_os_str))]
    input: PathBuf
}

fn set_up_logging(debug: bool) -> Result<(), Report> {
    if std::env::var("RUST_LIB_BACKTRACE").is_err() && debug {
        std::env::set_var("RUST_LIB_BACKTRACE", "full" );
    }

    color_eyre::install()?;

    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var("RUST_LOG", if debug { "debug" } else { "info" });
    }

    tracing_subscriber::fmt::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    Ok(())
}

fn main() -> Result<(), Report> {
    let options = Options::from_args();
    set_up_logging(options.debug)?;

    let input = read_to_string(&options.input)?;

    debug!("{file:?}: read {count} bytes", file=options.input, count=input.len());

    match options.puzzle {
        1 => day1::solve(input)?,
        2 => day2::solve(input)?,
        3 => day3::solve(input)?,
        4 => day4::solve(input)?,
        5 => day5::solve(input)?,
        _ => panic!("No such puzzle: {day}", day=options.puzzle)
    }

    Ok(())
}
