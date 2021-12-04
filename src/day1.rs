use color_eyre::Report;
use itertools::Itertools;
use tracing::info;

pub(crate) fn solve(input: String) -> Result<(), Report> {

    let numbers: Vec<u32> = input.split("\n").map(|l| l.parse::<u32>()).collect::<Result<Vec<_>, _>>()?;
    let part1 = numbers.iter()
        .tuple_windows()
        .filter(|(a, b)| b > a)
        .count();

    info!(day=1, part=1, answer=part1);

    let part2 = numbers.iter()
        .tuple_windows()
        .tuple_windows()
        .filter(|((&a1, &b1, &c1), (&a2, &b2, &c2))| a1 + b1 + c1 < a2 + b2 + c2)
        .count();

    info!(day=1, part=2, answer=part2);

    Ok(())
}
