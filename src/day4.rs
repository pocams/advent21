use color_eyre::Report;
use nom::{character, IResult};
use nom::bytes::complete::tag;
use nom::character::complete::{newline, space0};
use nom::combinator::{all_consuming, opt};
use nom::multi::{count, many1, separated_list1};
use nom::sequence::{delimited, terminated, tuple};
use tracing::{debug, info};

fn call_parser(i: &str) -> IResult<&str, Vec<u8>> {
    terminated(
        separated_list1(tag(","), character::complete::u8),
        newline)(i)
}

fn board_parser(i: &str) -> IResult<&str, Vec<Vec<u8>>> {
    terminated(
        count(
        terminated(
            count(
                delimited(
                    space0,
                    character::complete::u8,
                    space0
                ), 5),
            newline),
        5),
    opt(newline))(i)
}

fn is_winning_board(board: &[Vec<u8>]) -> bool {
    for row in board {
        if row.iter().all(|c| c & 128 == 128) { return true }
    }
    for col in 0..board[0].len() {
        if board.iter().all(|r| r[col] & 128 == 128) { return true }
    }
    false
}

fn board_score(board: &[Vec<u8>]) -> u32 {
    board
        .iter()
        .map(|row|
            row
                .iter()
                .map(|c| if c & 128 == 0 { *c as u32 } else { 0 })
                .sum::<u32>())
        .sum()
}

pub(crate) fn solve(input: String) -> Result<(), Report> {
    let (calls, mut boards) = match all_consuming(tuple((
        call_parser,
        newline,
        many1(board_parser)
    )))(&input) {
        Ok(("", (calls, _, boards))) => (calls, boards),
        Ok((leftovers, _)) => return Err(Report::msg(format!("Didn't parse all directions: {:?} left", leftovers))),
        Err(e) => return Err(Report::msg(format!("Failed to parse directions: {:?}", e)))
    };

    debug!("calls: {:?}", calls);

    let mut found_winner = false;

    for call in calls {
        for board in boards.iter_mut() {
            for row in board.iter_mut() {
                for ch in row.iter_mut() {
                    if *ch == call { *ch |= 128 }
                }
            }

            if is_winning_board(board) && !found_winner {
                debug!("Found winning board: {:?}", board);
                let score = board_score(board);
                info!(day=4, part=1, last_called=call, board_score=score, answer=call as u32 * score);
                found_winner = true;
            }
        }

        if boards.len() == 1 && is_winning_board(&boards[0]) {
            debug!("Found last winning board: {:?}", boards[0]);
            let score = board_score(&boards[0]);
            info!(day=4, part=2, last_called=call, board_score=score, answer=call as u32 * score);
        }

        boards.retain(|b| !is_winning_board(b));
    }

    Ok(())
}
