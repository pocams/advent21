use std::ops::{Add, Sub};
use color_eyre::Report;
use nom::bytes::complete::tag;
use nom::{character, IResult};
use nom::character::complete::newline;
use nom::combinator::{all_consuming, opt};
use nom::multi::many1;
use nom::sequence::tuple;
use fnv::FnvHashSet;
use tracing::{debug, info};

// 12 sensors in common means any given sensor pair should share 11 neighbors
const MIN_OVERLAP: usize = 11;

#[derive(Copy, Clone, Debug)]
#[allow(clippy::upper_case_acronyms)]
enum Transform {
    XYZ,
    XZY,
    YXZ,
    YZX,
    ZXY,
    ZYX,
}

const TRANSFORMS: [Transform; 6] = [
    Transform::XYZ,
    Transform::XZY,
    Transform::YXZ,
    Transform::YZX,
    Transform::ZXY,
    Transform::ZYX,
];

impl Transform {
    fn apply(&self, x: i32, y: i32, z: i32) -> (i32, i32, i32) {
        use Transform::*;
        match self {
            XYZ => (x, y, z),
            XZY => (x, z, y),
            YXZ => (y, x, z),
            YZX => (y, z, x),
            ZXY => (z, x, y),
            ZYX => (z, y, x),
        }
    }
}

#[derive(Copy, Clone, Debug)]
#[allow(clippy::upper_case_acronyms)]
enum Flip {
    None,
    X,
    Y,
    Z,
    XY,
    XZ,
    YZ,
    XYZ
}

const FLIPS: [Flip; 8] = [
    Flip::None,
    Flip::X,
    Flip::Y,
    Flip::Z,
    Flip::XY,
    Flip::XZ,
    Flip::YZ,
    Flip::XYZ
];

impl Flip {
    fn apply(&self, x: i32, y: i32, z: i32) -> (i32, i32, i32) {
        use Flip::*;
        match self {
            None => (x, y, z),
            X => (-x, y, z),
            Y => (x, -y, z),
            Z => (x, y, -z),
            XY => (-x, -y, z),
            XZ => (-x, y, -z),
            YZ => (x, -y, -z),
            XYZ => (-x, -y, -z)
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, Ord, PartialOrd)]
struct Beacon(i32, i32, i32);

impl Beacon {
    fn squared_distance(&self, other: &Beacon) -> i32 {
        (other.0 - self.0).pow(2) + (other.1 - self.1).pow(2) + (other.2 - self.2).pow(2)
    }

    /*
    fn distance(&self, other: &Beacon) -> f64 {
        (self.squared_distance(other) as f64).sqrt()
    }
    */

    fn manhattan_distance(&self, other: &Beacon) -> i32 {
        (other.0 - self.0).abs() + (other.1 - self.1).abs() + (other.2 - self.2).abs()
    }

    fn transformed(&self, transform: Transform, flip: Flip) -> Beacon {
        let (x, y, z) = transform.apply(self.0, self.1, self.2);
        let (xf, yf, zf) = flip.apply(x, y, z);
        Beacon(xf, yf, zf)
    }
}

impl Sub for Beacon {
    type Output = Beacon;

    fn sub(self, rhs: Self) -> Self::Output {
        Beacon(self.0 - rhs.0, self.1 - rhs.1, self.2 - rhs.2)
    }
}

impl Add for Beacon {
    type Output = Beacon;

    fn add(self, rhs: Self) -> Self::Output {
        Beacon(self.0 + rhs.0, self.1 + rhs.1, self.2 + rhs.2)
    }
}

#[derive(Debug)]
struct Scanner {
    number: i32,
    beacons: Vec<Beacon>
}

impl Scanner {
    fn beacon_distances(&self) -> Vec<FnvHashSet<i32>> {
        self.beacons.iter()
            .map(|b|
            self.beacons.iter()
                .filter(|b2| *b2 != b)
                .map(|b2| b.squared_distance(b2))
                .collect()
        ).collect()
    }

    fn transformed(&self, transform: Transform, flip: Flip) -> Scanner {
        Scanner {
            number: self.number,
            beacons: self.beacons.iter().map(|b| b.transformed(transform, flip)).collect()
        }
    }
}

fn parse_header(i: &str) -> IResult<&str, i32> {
    tuple((
        tag("--- scanner "),
        character::complete::i32,
        tag(" ---"),
        newline
        ))(i).map(|(left, (_, n, _, _))| (left, n))
}

fn parse_point(i: &str) -> IResult<&str, Beacon> {
    tuple((
        character::complete::i32,
        tag(","),
        character::complete::i32,
        tag(","),
        character::complete::i32,
        newline
    ))(i).map(|(left, (x, _, y, _, z, _))|
        (left, Beacon(x, y, z)))
}

fn parse_scanner(i: &str) -> IResult<&str, Scanner> {
    tuple((
        parse_header,
        many1(parse_point),
        opt(newline)
        ))(i).map(|(left, (number, points, _))|
                          (left, Scanner { number, beacons: points }))
}

fn find_beacons_in_common(s1: &Scanner, s2: &Scanner) -> Vec<(Beacon, Beacon)> {
    let mut beacons = vec![];
    let s1_distances = s1.beacon_distances();
    let s2_distances = s2.beacon_distances();
    for (i, s1d) in s1_distances.iter().enumerate() {
        for (j, s2d) in s2_distances.iter().enumerate() {
            let overlap = s1d.intersection(s2d).count();
            if overlap >= MIN_OVERLAP {
                beacons.push((s1.beacons[i], s2.beacons[j]));
                // println!("{:?} - {:?}: {}", s1.beacons[i], s2.beacons[j], overlap);
                // println!("{:?}", (s1.beacons[i].0 + s2.beacons[j].0, s1.beacons[i].1 - s2.beacons[j].1, s1.beacons[i].2 - (-s2.beacons[j].2)));
            }
        }
    }
    beacons
}

fn find_transform(base: &Scanner, target: &Scanner) -> Option<(Beacon, Transform, Flip)> {
    let beacons = find_beacons_in_common(base, target);
    if beacons.is_empty() { return None }

    for transform in &TRANSFORMS {
        for flip in &FLIPS {
            let mut matched = true;
            let offset = beacons[0].0 - beacons[0].1.transformed(*transform, *flip);
            // debug!("{:?}/{:?}: trying offset {:?}", transform, flip, offset);
            for (base_beacon, target_beacon) in &beacons[1..] {
                if *base_beacon - target_beacon.transformed(*transform, *flip) != offset {
                    matched = false;
                    break;
                }
            }
            if matched { return Some((offset, *transform, *flip)) }
        }
    }
    None
}

pub(crate) fn solve(input: String) -> Result<(), Report> {
    let mut scanners = match all_consuming(many1(parse_scanner))(&input) {
        Ok((_, scanners)) => scanners,
        Err(e) => return Err(Report::msg(format!("Failed to parse input: {:?}", e)))
    };

    let mut solved_scanners = vec![(Beacon(0, 0, 0), scanners.remove(0))];
    while !scanners.is_empty() {
        let mut transformed_scanner = None;
        let mut scanner_pos = None;

        'outer: for (pos, solved_scanner) in &solved_scanners {
            for scanner in &scanners {
                if let Some(xform) = find_transform(solved_scanner, scanner) {
                    transformed_scanner = Some(scanner.transformed(xform.1, xform.2));
                    scanner_pos = Some(*pos + xform.0);
                    break 'outer;
                }
            }
        }

        // Remove the newly-solved scanner from scanners
        scanners.retain(|s| s.number != transformed_scanner.as_ref().unwrap().number);

        solved_scanners.push((scanner_pos.unwrap(), transformed_scanner.unwrap()));
    }

    let mut all_beacons: FnvHashSet<Beacon> = Default::default();
    for (pos, scanner) in &solved_scanners {
        for beacon in &scanner.beacons {
            all_beacons.insert(*beacon + *pos);
        }
    }
    let mut beacons: Vec<_> = all_beacons.into_iter().collect();
    beacons.sort_unstable();

    for b in &beacons {
        debug!("{:?}", b);
    }

    info!(day=19, part=1, answer=beacons.len());

    let mut max_distance = 0;
    for i in 0..solved_scanners.len() {
        for j in i+1..solved_scanners.len() {
            max_distance = max_distance.max(solved_scanners[i].0.manhattan_distance(&solved_scanners[j].0));
        }
    }

    info!(day=19, part=2, answer=max_distance);

    Ok(())
}
