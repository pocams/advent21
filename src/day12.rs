use color_eyre::Report;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{alpha1, newline};
use nom::combinator::{all_consuming, opt};
use nom::IResult;
use nom::multi::many1;
use nom::sequence::tuple;
use tracing::{debug, info};

#[derive(Debug, Eq, PartialEq, Clone)]
enum Cave {
    Start,
    End,
    Small(String),
    Big(String)
}

impl Cave {
    fn is_small(&self) -> bool { matches!(self, Cave::Start | Cave::End | Cave::Small(_)) }
}

#[derive(Debug)]
struct Path {
    from: Cave,
    to: Cave
}

#[derive(Debug)]
struct Trip {
    caves: Vec<Cave>,
    can_revisit_small: bool
}

impl Trip {
    fn location(&self) -> &Cave {
        self.caves.last().unwrap()
    }

    fn with_next_cave(&self, cave: Cave) -> Trip {
        let mut new_caves = self.caves.clone();
        new_caves.push(cave);
        Trip { caves: new_caves, can_revisit_small: self.can_revisit_small }
    }

}

fn cave_parser(i: &str) -> IResult<&str, Cave> {
    alt((
      tag("start"),
      tag("end"),
      alpha1
    ))(i).map(|(left, r)|
        (left, match r {
            "start" => Cave::Start,
            "end" => Cave::End,
            s if s == s.to_ascii_uppercase() => Cave::Big(s.to_owned()),
            s => Cave::Small(s.to_owned())
        }))
}

fn path_parser(i: &str) -> IResult<&str, Path> {
    tuple((
        cave_parser,
        tag("-"),
        cave_parser,
        opt(newline)
    ))(i).map(|(left, (from, _, to, _))|
        (left, Path { from, to })
    )
}

fn walk(paths: &Vec<Path>, can_revisit_small: bool) -> Vec<Trip> {
    let mut trips = vec![Trip { caves: vec![Cave::Start], can_revisit_small }];
    let mut complete_trips = vec![];

    while let Some(trip) = trips.pop() {
        if matches!(trip.location(), Cave::End) {
            complete_trips.push(trip);
            continue
        }

        for path in paths {
            for (a, b) in [(&path.from, &path.to), (&path.to, &path.from)] {
                // We aren't interested in going back to the start
                if matches!(b, Cave::Start) { continue }

                if a == trip.location() {
                    // This is a path we could potentially take
                    if b.is_small() && trip.caves.contains(&b) {
                        if trip.can_revisit_small {
                            // This is our one chance to revisit a small cave
                            let mut new_trip = trip.with_next_cave(b.clone());
                            new_trip.can_revisit_small = false;
                            trips.push(new_trip);
                        } else {
                            // Been there already and can't revisit a small cave
                        }
                    } else {
                        trips.push(trip.with_next_cave(b.clone()))
                    }
                }
            }

        }
    }

    complete_trips
}

pub(crate) fn solve(input: String) -> Result<(), Report> {
    let paths = match all_consuming(many1(path_parser))(&input) {
        Ok(("", paths)) => paths,
        // This is unreachable because all_consuming returns an error if it doesn't parse the whole string
        Ok(_) => unreachable!(),
        Err(e) => return Err(Report::msg(format!("Failed to parse input: {:?}", e)))
    };

    debug!("{:?}", paths);

    let part1_trips = walk(&paths, false);
    debug!("part 1 trips: {:?}", part1_trips);
    info!(day=12, part=1, answer=part1_trips.len());

    let part2_trips = walk(&paths, true);
    debug!("part 2 trips: {:?}", part2_trips);
    info!(day=12, part=2, answer=part2_trips.len());

    Ok(())
}
