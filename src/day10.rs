use color_eyre::eyre::eyre;
use color_eyre::Report;
use tracing::{debug, info};

#[derive(Debug)]
enum Opener {
    Parenthesis,
    Bracket,
    CurlyBrace,
    LessThan
}

impl Opener {
    fn expected_closer(&self) -> char {
        match self {
            Opener::Parenthesis => ')',
            Opener::Bracket => ']',
            Opener::CurlyBrace => '}',
            Opener::LessThan => '>',
        }
    }

    fn closing_score(&self) -> u64 {
        match self {
            Opener::Parenthesis => 1,
            Opener::Bracket => 2,
            Opener::CurlyBrace => 3,
            Opener::LessThan => 4
        }
    }
}

impl TryInto<Opener> for char {
    type Error = Report;

    fn try_into(self) -> Result<Opener, Self::Error> {
        match self {
            '(' => Ok(Opener::Parenthesis),
            '[' => Ok(Opener::Bracket),
            '{' => Ok(Opener::CurlyBrace),
            '<' => Ok(Opener::LessThan),
            other => Err(eyre!("Unrecognized delimiter {}", other))
        }
    }
}

enum LineParseError {
    UnexpectedCloser(char),
    UnexpectedEndOfLine(u64)
}

fn character_score(ch: &char) -> u64 {
    match ch {
        ')' => 3,
        ']' => 57,
        '}' => 1197,
        '>' => 25137,
        _ => 0
    }
}

fn parse_line(line: &str) -> Result<(), LineParseError> {
    let mut stack: Vec<Opener> = vec![];
    for char in line.chars() {
        if let Ok(opener) = char.try_into() {
            stack.push(opener);
        } else {
            // This must be a closer
            if let Some(top) = stack.pop() {
                if char == top.expected_closer() {
                    // Okay
                } else {
                    return Err(LineParseError::UnexpectedCloser(char))
                }
            }
        }
    }

    if stack.is_empty() {
        Ok(())
    } else {
        let mut score = 0;
        debug!("leftovers: {:?}", stack);
        stack.reverse();
        for leftover in stack {
            score *= 5;
            score += leftover.closing_score();
        }
        Err(LineParseError::UnexpectedEndOfLine(score))
    }
}

pub(crate) fn solve(input: String) -> Result<(), Report> {
    let mut part1_score = 0;
    let mut part2_scores = vec![];
    for line in input.split('\n') {
        match parse_line(line) {
            Ok(()) => debug!("ok: {}", line),
            Err(LineParseError::UnexpectedCloser(c)) => {
                let score = character_score(&c);
                debug!("bad closer: {} ({} pts)", c, score);
                part1_score += score;
            },
            Err(LineParseError::UnexpectedEndOfLine(score)) => {
                debug!("incomplete: {} ({} pts)", line, score);
                part2_scores.push(score);
            },
        }
    }

    info!(day=10, part=1, answer=part1_score);

    part2_scores.sort_unstable();

    info!(day=10, part=2, answer=part2_scores[part2_scores.len() / 2]);
    Ok(())
}
