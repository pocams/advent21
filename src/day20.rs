use color_eyre::Report;
use nom::branch::alt;
use nom::{character, IResult};
use nom::character::complete::newline;
use nom::combinator::{all_consuming, opt};
use nom::multi::{count, many1};
use nom::sequence::tuple;

fn parse_cell(i: &str) -> IResult<&str, bool> {
    alt((
        character::complete::char('#'),
        character::complete::char('.')
    ))(i).map(|(left, t)| (left, t == '#') )
}

fn parse_algorithm(i: &str) -> IResult<&str, Vec<bool>> {
    tuple((
        count(parse_cell, 512),
        newline
    ))(i).map(|(left, (chars, _))| (left, chars))
}

fn parse_image_line(i: &str) -> IResult<&str, Vec<bool>> {
    tuple((
        many1(parse_cell),
        newline
    ))(i).map(|(left, (chars, _))| (left, chars))
}

fn parse_input(i: &str) -> IResult<&str, (Vec<bool>, Vec<Vec<bool>>)> {
    tuple((
        parse_algorithm,
        newline,
        many1(parse_image_line),
        opt(newline)
    ))(i).map(|(left, (algorithm, _, image_lines, _))|
                      (left, (algorithm, image_lines)))
}


pub(crate) fn solve(input: String) -> Result<(), Report> {
    let (algorithm, image_lines) = match all_consuming(parse_input)(&input) {
        Ok((_, (algorithm, image_lines))) => (algorithm, image_lines),
        Err(e) => return Err(Report::msg(format!("Failed to parse input: {:?}", e)))
    };


    Ok(())
}
