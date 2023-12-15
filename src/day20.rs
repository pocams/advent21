use color_eyre::Report;
use fnv::FnvHashMap;
use nom::{character, IResult};
use nom::branch::alt;
use nom::character::complete::newline;
use nom::combinator::{all_consuming, opt};
use nom::multi::{count, many1};
use nom::sequence::tuple;
use tracing::info;

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

fn calculate_cell(cells: &FnvHashMap<(i32, i32), bool>, coords: (i32, i32), algorithm: &[bool], unset: bool) -> bool {
    let mut cell_value = 0;
    for offset in &[(-1, -1), (-1, 0), (-1, 1), (0, -1), (0, 0), (0, 1), (1, -1), (1, 0), (1, 1)] {
        cell_value <<= 1;
        if *cells.get(&(coords.0 + offset.0, coords.1 + offset.1)).unwrap_or(&unset) { cell_value |= 1 }
    }
    algorithm[cell_value]
}

fn show(cells: &FnvHashMap<(i32, i32), bool>) {
    let top = cells.keys().min_by(|a, b| a.0.cmp(&b.0)).unwrap().0 - 1;
    let bottom = cells.keys().max_by(|a, b| a.0.cmp(&b.0)).unwrap().0 + 1;
    let left = cells.keys().min_by(|a, b| a.1.cmp(&b.1)).unwrap().1 - 1;
    let right = cells.keys().max_by(|a, b| a.1.cmp(&b.1)).unwrap().1 + 1;
    for row in top..=bottom {
        for col in left..=right {
            if *cells.get(&(row, col)).unwrap_or(&false) {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
    }
    println!();
}

pub(crate) fn solve(input: String) -> Result<(), Report> {
    let (algorithm, image_lines) = match all_consuming(parse_input)(&input) {
        Ok((_, (algorithm, image_lines))) => (algorithm, image_lines),
        Err(e) => return Err(Report::msg(format!("Failed to parse input: {:?}", e)))
    };
    let mut cells = FnvHashMap::default();
    for (row_num, row) in image_lines.iter().enumerate() {
        for (col_num, val) in row.iter().enumerate() {
            cells.insert((row_num as i32, col_num as i32), *val);
        }
    }
    show(&cells);

    let mut top = 0i32;
    let mut left = 0i32;
    let mut bottom = image_lines.len() as i32;
    let mut right = image_lines[0].len() as i32;

    let mut unset = false;
    for _ in 0..50 {
        let mut new_cells = FnvHashMap::default();
        top -= 1;
        left -= 1;
        bottom += 1;
        right += 1;
        for row in top..=bottom {
            for col in left..=right {
                new_cells.insert((row, col), calculate_cell(&cells, (row, col), &algorithm, unset));
            }
        }
        cells = new_cells;
        unset = !unset;
        show(&cells);
    }

    info!(day=20, part=1, answer=cells.values().filter(|c| **c).count());


    Ok(())
}
