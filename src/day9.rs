use std::collections::HashMap;
use color_eyre::Report;
use itertools::Itertools;
use tracing::{debug, info};

const WALL: u8 = 9;

fn fill(v: &mut Vec<Vec<u8>>, grid: &[Vec<u8>], row: usize, col: usize, fill_with: u8) {
    if grid[row][col] == WALL { return }
    if v[row][col] != 0 { return }
    v[row][col] = fill_with;
    if row > 0 { fill(v, grid, row - 1, col, fill_with) }
    if row < v.len() - 1 { fill(v, grid, row + 1, col, fill_with) }
    if col > 0 { fill(v, grid, row, col - 1, fill_with) }
    if col < v[0].len() - 1 { fill(v, grid, row, col + 1, fill_with) }
}

pub(crate) fn solve(input: String) -> Result<(), Report> {
    let lines: Vec<_> = input.trim().split('\n').collect();
    let mut grid = Vec::with_capacity(lines.len());
    for line in lines {
        let row: Vec<_> = line.bytes().map(|b| b - b'0').collect();
        grid.push(row)
    }

    debug!("{:?}", grid);

    let rows = grid.len();
    let cols = grid[0].len();

    let mut part1_sum = 0;

    for row in 0..rows {
        for col in 0..cols {
            if (col == cols - 1 || grid[row][col+1] > grid[row][col]) &&
                (col == 0 || grid[row][col-1] > grid[row][col]) &&
                (row == rows - 1 || grid[row+1][col] > grid[row][col]) &&
                (row == 0 || grid[row-1][col] > grid[row][col]) {
                debug!("low point at {}, {}: {}", row, col, grid[row][col]);
                part1_sum += grid[row][col] as u32 + 1;
            }
        }
    }

    info!(day=9, part=1, answer=part1_sum);

    let mut partitions = vec![vec![0u8; cols]; rows];
    let mut current_partition = 1;

    for row in 0..rows {
        for col in 0..cols {
            if grid[row][col] == WALL { continue }
            if partitions[row][col] == 0 {
                fill(&mut partitions, &grid, row, col, current_partition);
                current_partition += 1;
            }
        }
    }

    let mut sizes = HashMap::new();
    for row in &partitions {
        for &col in row {
            if col != 0 {
                *sizes.entry(col).or_insert(0) += 1;
            }
        }
    }

    let top_3: Vec<_> = sizes.iter().sorted_by_key(|(_, &v)| -v).take(3).collect();
    debug!("{:?}", sizes);
    debug!("{:?}", top_3);
    // debug!("{:?}", partitions);

    let answer: i32 = top_3.iter().map(|(_, &v)| v).product();
    info!(day=9, part=2, answer=answer);

    Ok(())
}
