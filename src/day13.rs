use color_eyre::Report;
use nom::{character, IResult};
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{newline, one_of, space0, space1};
use nom::combinator::{all_consuming, opt};
use nom::multi::many1;
use nom::sequence::tuple;
use tracing::{debug, info};
use crate::day13::Instruction::{FoldAlongX, FoldAlongY};

#[derive(Debug, Eq, PartialEq)]
struct Dot {
    x: usize,
    y: usize
}

impl Dot {
    fn fold(self, i: &Instruction) -> Dot {
        match i {
            FoldAlongX(xfold) =>
            if self.x > *xfold {
                Dot { x: *xfold - (self.x - *xfold), y: self.y }
            } else {
                self
            }
            FoldAlongY(yfold) =>
            if self.y > *yfold {
                Dot { x: self.x, y : *yfold - (self.y - *yfold)}
            } else {
                self
            }
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
enum Instruction {
    FoldAlongX(usize),
    FoldAlongY(usize)
}

#[derive(Debug, Eq, PartialEq)]
enum Line {
    Dot(Dot),
    Instruction(Instruction),
    Empty
}

fn dot_parser(i: &str) -> IResult<&str, Line> {
    tuple((
        character::complete::i64,
        space0,
        tag(","),
        space0,
        character::complete::i64,
        opt(newline)
        ))(i)
        .map(|(left, (x, _, _, _, y, _))| (left, Line::Dot(Dot { x: x as usize, y: y as usize })))
}

fn instruction_parser(i: &str) -> IResult<&str, Line> {
    tuple((
        tag("fold along"),
        space1,
        one_of("xy"),
        tag("="),
        character::complete::i64,
        opt(newline))
        )(i)
        .map(|(left, (_, _, axis, _, coordinate, _))| (left, match axis {
            'x' => Line::Instruction(FoldAlongX(coordinate as usize)),
            'y' => Line::Instruction(FoldAlongY(coordinate as usize)),
            // We said one_of("xy") so we'll only get "x" or "y"
            _ => unreachable!()
        }))
}

fn empty_line_parser(i: &str) -> IResult<&str, Line> {
    newline(i).map(|(left, _)| (left, Line::Empty))
}

fn lines_parser(i: &str) -> IResult<&str, Vec<Line>> {
    many1(
        alt((
            dot_parser,
            instruction_parser,
            empty_line_parser
        ))
    )(i)
}

fn fold(dots: Vec<Dot>, instruction: &Instruction) -> Vec<Dot> {
    let mut new_dots = Vec::with_capacity(dots.len());
    for dot in dots {
        let dot = dot.fold(instruction);
        if !new_dots.iter().any(|nd| *nd == dot) {
            new_dots.push(dot)
        }
    }
    new_dots
}

fn show(dots: &[Dot]) -> String {
    let max_x = dots.iter().max_by_key(|d| d.x).map(|d| d.x).unwrap() + 1;
    let max_y = dots.iter().max_by_key(|d| d.y).map(|d| d.y).unwrap() + 1;
    let mut s = String::with_capacity((max_x + 1) * max_y);
    for y in 0..max_y {
        for x in 0..max_x {
            if dots.iter().any(|d| d.x == x && d.y == y) {
                s.push('#');
            } else {
                s.push('.');
            }
        }
        s.push('\n');
    }
    s
}

pub(crate) fn solve(input: String) -> Result<(), Report> {
    let lines = match all_consuming(lines_parser)(&input) {
        Ok(("", lines)) => lines,
        // all_consuming won't return a success with anything left over
        Ok(_) => unreachable!(),
        Err(e) => return Err(Report::msg(format!("Failed to parse input: {:?}", e)))
    };

    debug!("{:?}", lines);

    // There must be a better way to handle multiple data types being parsed from a single file, right?
    let mut dots = vec![];
    let mut instructions = vec![];
    for line in lines {
        match line {
            Line::Dot(d) => dots.push(d),
            Line::Instruction(i) => instructions.push(i),
            Line::Empty => {}
        }
    }

    dots = fold(dots, &instructions.remove(0));

    debug!("{:?}", dots);

    info!(day=13, part=1, answer=dots.len());

    for i in &instructions {
        dots = fold(dots, i);
    }

    // This is a bit hinky, but whatever
    for row in show(&dots).split('\n') {
        if row.len() > 0 {
            info!(day=13, part=2, answer=row);
        }
    }

    Ok(())
}
