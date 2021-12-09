use color_eyre::eyre::eyre;
use color_eyre::Report;
use itertools::Itertools;
use nom::bytes::complete::tag;
use nom::character::complete::{alpha1, newline, space0};
use nom::combinator::{all_consuming, opt};
use nom::IResult;
use nom::multi::{count, many1};
use nom::sequence::{terminated, tuple};
use tracing::{debug, info};

#[derive(Debug)]
struct TestCase {
    all_digits: Vec<String>,
    display: Vec<String>
}

// fn get_digits() -> Vec<&'static str> {
//     vec![
//         "abcefg",
//         "cf",
//         "acdeg",
//         "acdfg",
//         "bcdf",
//         "abdfg",
//         "abdefg",
//         "acf",
//         "abcdefg",
//         "abdcfg"
//     ]
// }
//
fn digit_parser(i: &str) -> IResult<&str, String> {
    alpha1(i)
        .map(|(left, segments)| (left, segments.chars().sorted().collect()))
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

    let output_digits_with_unique_number_of_segments = cases.iter().map(|m| m.display.iter().filter(|d| matches!(d.len(), 2 | 3 | 4 | 7)).count()).sum::<usize>();
    info!(day=8, part=1, answer=output_digits_with_unique_number_of_segments);

    let mut sum = 0;

    for case in cases {
        debug!("{:?}", case.all_digits);
        let mut v = vec![""; 10];

        // The only 2-segment digit is 1
        v[1] = case.all_digits.iter().find(|&n| n.len() == 2).ok_or_else(|| eyre!("1 not found"))?;
        debug!("1 = {}", v[1]);
        // The only 4-segment digit is 4
        v[4] = case.all_digits.iter().find(|&n| n.len() == 4).ok_or_else(|| eyre!("4 not found"))?;
        debug!("4 = {}", v[4]);
        // The only 3-segment digit is 7
        v[7] = case.all_digits.iter().find(|&n| n.len() == 3).ok_or_else(|| eyre!("7 not found"))?;
        debug!("7 = {}", v[7]);
        // The only 7-segment digit is 8
        v[8] = case.all_digits.iter().find(|&n| n.len() == 7).ok_or_else(|| eyre!("8 not found"))?;
        debug!("8 = {}", v[8]);

        // The only 6-segment digit that contains only one segment of 1 is 6
        v[6] = case.all_digits.iter().find(|&n| n.len() == 6 && n.chars().filter(|&c| v[1].contains(c)).count() == 1).ok_or_else(|| eyre!("6 not found"))?;
        debug!("6 = {}", v[6]);
        // The only 6-segment digit that contains all 4 segments of 4 is 9
        v[9] = case.all_digits.iter().find(|&n| n.len() == 6 && n.chars().filter(|&c| v[4].contains(c)).count() == 4).ok_or_else(|| eyre!("9 not found"))?;
        debug!("9 = {}", v[9]);
        // The remaining 6-segment digit is 0
        v[0] = case.all_digits.iter().find(|&n| n.len() == 6 && n != v[6] && n != v[9]).ok_or_else(|| eyre!("0 not found"))?;
        debug!("0 = {}", v[0]);

        // The only 5-segment digit that contains both segments of 1 is 3
        v[3] = case.all_digits.iter().find(|&n| n.len() == 5 && n.chars().filter(|&c| v[1].contains(c)).count() == 2).ok_or_else(|| eyre!("3 not found"))?;
        debug!("3 = {}", v[3]);
        // The only 5-segment digit that is entirely contained within the segments of 6 is 5
        v[5] = case.all_digits.iter().find(|&n| n.len() == 5 && n.chars().filter(|&c| v[6].contains(c)).count() == 5).ok_or_else(|| eyre!("5 not found"))?;
        debug!("5 = {}", v[5]);
        // The remaining 5-segment digit is 2
        v[2] = case.all_digits.iter().find(|&n| n.len() == 5 && n != v[3] && n != v[5]).ok_or_else(|| eyre!("2 not found"))?;
        debug!("2 = {}", v[2]);

        let mut result = 0;
        for digit in case.display {
            result *= 10;
            result += v.iter().position(|&n| n == digit).ok_or_else(|| eyre!("Number for {} not found!", digit))?;
        }

        debug!("result: {:?}", result);
        sum += result;
    }

    info!(day=8, part=2, answer=sum);

    Ok(())
}
