use std::str::FromStr;
use color_eyre::Report;
use tracing::{debug, info};

const MAX_TIMER: usize = 10;

fn iterate(counts: &mut [u64; MAX_TIMER]) {
    let add = counts[0];
    for i in 0..9 {
        counts[i] = counts[i+1];
    }
    counts[6] += add;
    counts[8] += add;
}

pub(crate) fn solve(input: String) -> Result<(), Report> {
    let fish = input.split(',').map(i32::from_str).collect::<Result<Vec<_>, _>>()?;
    let mut counts: [u64; MAX_TIMER] = Default::default();

    for f in fish {
        counts[f as usize] += 1
    }

    debug!("Initial counts: {:?}", counts);
    for _ in 0..18 { iterate(&mut counts) }
    debug!("After 18 days: {} fish ({:?})", counts.iter().sum::<u64>(), counts);
    for _ in 18..80 { iterate(&mut counts) }

    info!(day=6, part=1, answer=counts.iter().sum::<u64>());

    for _ in 80..256 { iterate(&mut counts) }

    info!(day=6, part=2, answer=counts.iter().sum::<u64>());

    Ok(())
}
