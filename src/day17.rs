use color_eyre::eyre::eyre;
use color_eyre::Report;
use nom::bytes::complete::tag;
use nom::{character, IResult};
use nom::character::complete::newline;
use nom::combinator::{all_consuming, opt};
use nom::sequence::tuple;
use tracing::{debug, info};

#[derive(Debug)]
struct TargetArea {
    x1: i32,
    x2: i32,
    y1: i32,
    y2: i32
}

impl TargetArea {
    fn contains(&self, x: i32, y: i32) -> bool {
        self.x1 <= x && x <= self.x2 && self.y1 <= y && y <= self.y2
    }

    fn part1_x_velocity(&self) -> Result<i32, Report> {
        let mut total = 0;
        for x in 0..100 {
            total += x;
            if total >= self.x1 && total <= self.x2 { return Ok(x) }
        }
        Err(eyre!("No valid X velocity found"))
    }
}

#[derive(Debug)]
struct Probe {
    x: i32,
    y: i32,
    vel_x: i32,
    vel_y: i32,
}

impl Probe {
    fn fire(vel_x: i32, vel_y: i32) -> Probe {
        Probe {
            x: 0,
            y: 0,
            vel_x,
            vel_y
        }
    }
}

impl Iterator for Probe {
    type Item = (i32, i32);

    fn next(&mut self) -> Option<Self::Item> {
        self.x += self.vel_x;
        self.y += self.vel_y;
        self.vel_x -= self.vel_x.signum();
        self.vel_y -= 1;
        Some((self.x, self.y))
    }
}

fn parse_target_area(i: &str) -> IResult<&str, TargetArea> {
    tuple((
        tag("target area: x="),
        character::complete::i32,
        tag(".."),
        character::complete::i32,
        tag(", y="),
        character::complete::i32,
        tag(".."),
        character::complete::i32,
        opt(newline)
    ))(i).map(|(left, (_, x1, _, x2, _, y1, _, y2, _))| (left, TargetArea { x1, x2, y1, y2 }))
}

pub(crate) fn solve(input: String) -> Result<(), Report> {
    let target = match all_consuming(parse_target_area)(&input) {
        Ok((_, target)) => target,
        Err(e) => return Err(Report::msg(format!("Failed to parse input: {:?}", e)))
    };

    debug!("{:?}", target);

    let vel_x = target.part1_x_velocity()?;

    let mut overall_max_y = 0;
    for vel_y in 1..2000 {
        let mut max_y = 0;
        for (x, y) in Probe::fire(vel_x, vel_y) {
            if y > max_y { max_y = y }
            if target.contains(x, y) {
                if max_y > overall_max_y {
                    debug!("new max y: {} @ {}, {}", max_y, vel_x, vel_y);
                    overall_max_y = max_y;
                }
            } else if x > target.x2 || y < target.y1 { break }
        }
    }

    info!(day=17, part=1, answer=overall_max_y);

    let mut possibilities = 0;

    for vel_x in 0..=target.x2 {
        for vel_y in target.y1..2000 {
            for (x, y) in Probe::fire(vel_x, vel_y) {
                if target.contains(x, y) {
                    debug!("possibility: {}, {}", vel_x, vel_y);
                    possibilities += 1;
                    break;
                } else if x > target.x2 || y < target.y1 {
                    break
                }
            }
        }
    }

    info!(day=17, part=2, answer=possibilities);

    Ok(())
}
