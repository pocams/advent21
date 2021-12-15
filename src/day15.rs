use color_eyre::Report;
use tracing::info;

#[derive(Debug)]
struct Cave {
    risks: Vec<Vec<u8>>,
    costs: Vec<Vec<u64>>,
    to_consider: Vec<(usize, usize)>
}

impl Cave {
    fn new(risks: Vec<Vec<u8>>) -> Cave {
        let mut costs = vec![vec![u64::MAX; risks[0].len()]; risks.len()];
        costs[0][0] = 0;
        Cave { risks, costs, to_consider: vec![(0, 0)] }
    }

    fn consider(&mut self, row: usize, col: usize, cost: u64) {
        if self.costs[row][col] > cost {
            self.costs[row][col] = cost;
            self.to_consider.push((row, col));
        }
    }

    fn width(&self) -> usize { self.risks[0].len() }
    fn height(&self) -> usize { self.risks.len() }

    fn answer(&self) -> u64 {
        *self.costs.last().unwrap().last().unwrap()
    }

    fn walk(&mut self) {
        while let Some((row, col)) = self.to_consider.pop() {
            let cost = self.costs[row][col];

            if row > 0 {
                self.consider(row-1, col, cost + self.risks[row-1][col] as u64);
            }

            if row < self.height() - 1 {
                self.consider(row+1, col, cost + self.risks[row+1][col] as u64);
            }

            if col > 0 {
                self.consider(row, col-1, cost + self.risks[row][col-1] as u64);
            }

            if col < self.width() - 1 {
                self.consider(row, col+1, cost + self.risks[row][col+1] as u64);
            }
        }
    }

    fn into_part2_cave(self) -> Cave {
        let mut new_risks = self.risks.clone();
        for row in new_risks.iter_mut() {
            for i in 0..(self.width() * 4) {
                let new_risk = row[i] % 9 + 1;
                row.push(new_risk);
            }
        }

        for i in 0..self.height() * 4 {
            new_risks.push(
                new_risks[i].iter().map(|r| r % 9 + 1).collect()
            )
        }

        Cave::new(new_risks)
    }
}

pub(crate) fn solve(input: String) -> Result<(), Report> {
    let grid: Vec<Vec<u8>> = input.trim().split('\n')
        .map(|line| line.bytes().map(|b| b - b'0').collect())
        .collect();

    let mut cave = Cave::new(grid);

    cave.walk();

    info!(day=15, part=1, answer=cave.answer());

    let mut part2_cave = cave.into_part2_cave();

    part2_cave.walk();

    info!(day=15, part=2, answer=part2_cave.answer());

    Ok(())
}
