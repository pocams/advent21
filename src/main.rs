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
mod day6;
mod day7;
mod day8;
mod day9;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
mod day16;
mod day17;
mod day18;

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
        6 => day6::solve(input)?,
        7 => day7::solve(input)?,
        8 => day8::solve(input)?,
        9 => day9::solve(input)?,
        10 => day10::solve(input)?,
        11 => day11::solve(input)?,
        12 => day12::solve(input)?,
        13 => day13::solve(input)?,
        14 => day14::solve(input)?,
        15 => day15::solve(input)?,
        16 => day16::solve(input)?,
        17 => day17::solve(input)?,
        18 => day18::solve(input)?,
        _ => panic!("No such puzzle: {day}", day=options.puzzle)
    }

    Ok(())
}
