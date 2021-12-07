use std::collections::HashMap;
use std::str::FromStr;
use color_eyre::Report;
use tracing::{debug, info};

const MAX_OFFSET: i32 = 200;


fn part2_cost_to_move(distance: i32) -> i32 {
    if distance % 2 == 0 {
        (distance + 1) * (distance / 2)
    } else {
        (distance + 1) * (distance / 2) + (distance + 1) / 2
    }
}

pub(crate) fn solve(input: String) -> Result<(), Report> {
    let positions = input.split(',').map(i32::from_str).collect::<Result<Vec<_>, _>>()?;
    let average = positions.iter().sum::<i32>() / positions.len() as i32;
    debug!("average position is {}", average);

    let mut scores = HashMap::new();

    for position in (average - MAX_OFFSET).max(0)..(average + MAX_OFFSET) {
        let total_distance = positions.iter().map(|pos| (position - pos).abs()).sum::<i32>();
        scores.insert(position, total_distance);
    }

    let best = scores.iter().min_by_key(|(_position, score)| **score).unwrap();

    info!(day=7, part=1, position=best.0, answer=best.1);

    scores.clear();

    for position in (average - MAX_OFFSET).max(0)..(average + MAX_OFFSET) {
        let total_distance = positions.iter().map(|pos| part2_cost_to_move((position - pos).abs())).sum::<i32>();
        scores.insert(position, total_distance);
    }

    let best = scores.iter().min_by_key(|(_position, score)| **score).unwrap();

    info!(day=7, part=2, position=best.0, answer=best.1);

    Ok(())
}