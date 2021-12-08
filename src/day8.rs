use color_eyre::Report;
use nom::bytes::complete::tag;
use nom::character::complete::{alpha1, newline, space0};
use nom::combinator::{all_consuming, opt};
use nom::IResult;
use nom::multi::{count, many1};
use nom::sequence::{terminated, tuple};
use tracing::info;

#[derive(Debug)]
struct Digit {
    segments: Vec<char>
}

impl Digit {
    fn has_unique_number_of_segments(&self) -> bool {
        matches!(self.segments.len(), 2 | 4 | 3 | 7)
    }
}

#[derive(Debug)]
struct TestCase {
    all_digits: Vec<Digit>,
    display: Vec<Digit>

}

fn digit_parser(i: &str) -> IResult<&str, Digit> {
    alpha1(i)
        .map(|(left, segments)| (left, Digit { segments: segments.chars().collect() }))
}

fn line_parser(i: &str) -> IResult<&str, TestCase> {
    terminated(
        tuple((
            count(
                tuple((
                    space0,
                    digit_parser,
                    space0
                )),
                10),

            tag("|"),

            count(
                tuple((
                    space0,
                    digit_parser,
                    space0,
                )),
                4),
        )),
        opt(newline),
    )(i)
        .map(|(left, (all_digits, _, display))|
            (left, TestCase {
                all_digits: all_digits.into_iter().map(|(_, digit, _)| digit).collect(),
                display: display.into_iter().map(|(_, digit, _)| digit).collect()
            }))
}

pub(crate) fn solve(input: String) -> Result<(), Report> {
    let cases = match all_consuming(many1(line_parser))(&input) {
        Ok(("", cases)) => cases,
        Ok((leftovers, _)) => return Err(Report::msg(format!("Didn't parse all lines: {:?} left", leftovers))),
        Err(e) => return Err(Report::msg(format!("Failed to parse input: {:?}", e)))
    };

    let output_digits_with_unique_number_of_segments = cases.iter().map(|m| m.display.iter().filter(|d| d.has_unique_number_of_segments()).count()).sum::<usize>();
    info!(day=8, part=1, answer=output_digits_with_unique_number_of_segments);

    Ok(())
}