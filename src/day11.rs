use color_eyre::Report;
use tracing::{debug, info};

const EDGE: usize = 10;

fn step(grid: &mut [[i8; EDGE]; EDGE]) -> u64 {
    for row in grid.iter_mut() {
        for col in row.iter_mut() {
            *col += 1
        }
    }

    let mut total_flashes = 0;
    loop {
        let mut flashes = 0;
        for r in 0..EDGE {
            for c in 0..EDGE {
                if grid[r][c] == 10 {
                    flashes += 1;

                    // Increment this cell, even though it's not part of the spec, so we can keep track
                    // of which have already flashed
                    grid[r][c] += 1;

                    for dr in &[-1, 0, 1] {
                        for dc in &[-1, 0, 1] {
                            let target_r = r as i32 + dr;
                            let target_c = c as i32 + dc;
                            if target_r >= 0 && target_r < EDGE as i32 && target_c >= 0 && target_c < EDGE as i32 {
                                let target_r = target_r as usize;
                                let target_c = target_c as usize;
                                if grid[target_r][target_c] < 10 {
                                    grid[target_r][target_c] += 1;
                                }
                            }
                        }
                    }
                }
            }
        }
        if flashes == 0 {
            break
        } else {
            total_flashes += flashes;
        }
    }

    for row in grid.iter_mut() {
        for col in row.iter_mut() {
            if *col == 11 {
                *col = 0;
            }
        }
    }

    total_flashes
}

pub(crate) fn solve(input: String) -> Result<(), Report> {
    let mut grid = [[0i8; EDGE]; EDGE];
    for (row, grid_row) in input.split('\n').zip(grid.iter_mut()) {
        for (index, char) in row.bytes().enumerate() {
            grid_row[index] = i8::try_from(char - b'0')?;
        }
    }

    debug!("{:?}", grid);

    let mut flashes = 0;
    for _ in 0..100 {
        flashes += step(&mut grid);
    }

    info!(day=11, part=1, answer=flashes);

    let mut step_count = 100;
    while !grid.iter().all(|r| r.iter().all(|&c| c == 0)) {
        step(&mut grid);
        step_count += 1;
    }

    info!(day=11, part=2, answer=step_count);

    Ok(())
}
