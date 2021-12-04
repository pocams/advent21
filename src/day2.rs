use color_eyre::Report;
use itertools::Itertools;
use nom::{character, IResult};
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::multispace1;
use nom::combinator::complete;
use nom::error::context;
use nom::Finish;
use nom::sequence::tuple;
use tracing::info;

#[derive(Debug)]
enum Direction {
    Forward(i32),
    Up(i32),
    Down(i32),
}

#[derive(Debug)]
struct Position {
    depth: i32,
    forward: i32,
    aim: i32
}

impl Position {
    fn new() -> Position {
        Position { depth: 0, forward: 0, aim: 0 }
    }

    fn update(&mut self, direction: &Direction) {
        match direction {
            Direction::Forward(n) => self.forward += n,
            Direction::Up(n) => self.depth -= n,
            Direction::Down(n) => self.depth += n
        }
    }

    fn update_part2(&mut self, direction: &Direction) {
        match direction {
            Direction::Forward(n) => {
                self.forward += n;
                self.depth += n * self.aim;
            },
            Direction::Up(n) => self.aim -= n,
            Direction::Down(n) => self.aim += n
        }
    }

    fn answer(&self) -> i32 {
        self.depth * self.forward
    }
}

fn direction_parser(i: &str) -> IResult<&str, Direction> {
    complete(
        tuple((
            context(
                "direction",
                alt((tag("forward"), tag("up"), tag("down")))
            ),
            multispace1,
            character::complete::i32
        )),
        )(i)
        .map(|(s, (direction, _, distance))| (s, match direction {
            "forward" => Direction::Forward(distance),
            "up" => Direction::Up(distance),
            "down" => Direction::Down(distance),
            _ => unreachable!()
        }))
}

pub(crate) fn solve(input: String) -> Result<(), Report> {
    let lines: Vec<_> = input.split("\n").collect();
    let mut directions = Vec::with_capacity(lines.len());
    for line in lines {
        match direction_parser(line) {
            Ok((_, direction)) => directions.push(direction),
            Err(e) => {
                return Err(Report::msg(format!("Failed to parse directions: {:?}", e)));
            }
        }
    }

    let mut position = Position::new();
    for direction in &directions {
        position.update(direction)
    }

    info!(day=2, part=1, answer=position.answer());

    let mut position = Position::new();
    for direction in &directions {
        position.update_part2(direction)
    }

    info!(day=2, part=2, answer=position.answer());

    Ok(())
}
