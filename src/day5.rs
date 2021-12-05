use std::collections::HashMap;
use color_eyre::Report;
use nom::bytes::complete::tag;
use nom::{character, IResult};
use nom::character::complete::{newline, space0, space1};
use nom::combinator::all_consuming;
use nom::multi::many1;
use nom::sequence::{terminated, tuple};
use tracing::{debug, info};

#[derive(Debug)]
struct Line {
    x1: i32,
    y1: i32,
    x2: i32,
    y2: i32
}

impl Line {
    fn new(x1: i32, y1: i32, x2: i32, y2: i32) -> Line {
        if x1 < x2 || (x1 == x2 && y1 <= y2) {
            Line { x1, y1, x2, y2 }
        } else {
            Line { x1: x2, y1: y2, x2: x1, y2: y1 }
        }
    }

    fn is_straight(&self) -> bool {
        self.x1 == self.x2 || self.y1 == self.y2
    }

    fn covered_coordinates(&self) -> Vec<(i32, i32)> {
        let mut v = vec![];
        let mut pos = (self.x1, self.y1);
        loop {
            v.push(pos);
            if pos.0 == self.x2 && pos.1 == self.y2 { break }
            pos.0 += (self.x2 - pos.0).signum();
            pos.1 += (self.y2 - pos.1).signum();
        }
        v
    }
}

fn pair_parser(i: &str) -> IResult<&str, (i32, i32)> {
    tuple((
        character::complete::i32,
        space0,
        tag(","),
        space0,
        character::complete::i32
        ))(i).map(|(rest, (x, _, _, _, y))| (rest, (x, y)))
}

fn line_parser(i: &str) -> IResult<&str, Line> {
    terminated(
        tuple((
            pair_parser,
            space1,
            tag("->"),
            space1,
            pair_parser
        )),
        newline
    )(i).map(|(rest, (p1, _, _, _, p2))| (rest, Line::new(p1.0, p1.1, p2.0, p2.1)))
}

pub(crate) fn solve(input: String) -> Result<(), Report> {
    let lines = match all_consuming(many1(line_parser))(&input) {
        Ok(("", lines)) => lines,
        Ok((leftovers, _)) => return Err(Report::msg(format!("Didn't parse all lines: {:?} left", leftovers))),
        Err(e) => return Err(Report::msg(format!("Failed to parse input: {:?}", e)))
    };

    let mut part1_counts: HashMap<(i32, i32), i32> = HashMap::new();
    let mut counts: HashMap<(i32, i32), i32> = HashMap::new();

    for line in &lines {
        debug!("{:?}: {:?}", line, line.covered_coordinates());
        for pair in line.covered_coordinates() {
            if line.is_straight() { *part1_counts.entry(pair).or_insert(0) += 1 }
            *counts.entry(pair).or_insert(0) += 1
        }
    }

    let part1 = part1_counts.values().filter(|&&c| c > 1).count();
    info!(day=5, part=1, answer=part1);
    let part2 = counts.values().filter(|&&c| c > 1).count();
    info!(day=5, part=2, answer=part2);

    Ok(())
}
