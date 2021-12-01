use std::path::PathBuf;

use color_eyre::Report;
use structopt::StructOpt;
use tracing_subscriber::EnvFilter;

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
        std::env::set_var("RUST_LIB_BACKTRACE", "1" );
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
    Ok(())
}
