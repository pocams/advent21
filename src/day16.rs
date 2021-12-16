use color_eyre::Report;
use nom::IResult;
use nom::bits::complete::{take, tag};
use nom::branch::alt;
use nom::multi::many0;
use nom::sequence::{terminated, tuple};
use tracing::debug;

const TAG_LITERAL: u8 = 4;

enum Packet {
    LiteralValue(u64),
    Operator
}

fn parse_version(input: (&[u8], usize))-> IResult<(&[u8], usize), u8> {
    take(3usize)(input)
}

fn build_number(groups: Vec<u8>, last_group: u8) -> u64 {
    let mut result = 0;
    for group in groups {
        result |= (group & 0xf) as u64;
        result <<= 4;
    }
    result |= last_group as u64;
    result
}

fn parse_number(input: (&[u8], usize))-> IResult<(&[u8], usize), u64> {
    tuple((
        many0(
            tuple((
                tag(1usize, 1usize),
                take(4usize)
            ))
        ),
        tuple((
            tag(0usize, 1usize),
            take(4usize)
        ))
    ))(input).map(|(left, (groups, (_, last_group)))|
        (left, build_number(groups.into_iter().map(|(_, group)| group).collect(), last_group))
    )
}

fn parse_literal_value_packet(input: (&[u8], usize))-> IResult<(&[u8], usize), Packet> {
    tuple((
        tag(TAG_LITERAL, 3usize),
        parse_number
    ))(input).map(|(left, (_, number))| (left, Packet::LiteralValue(number)))
}

fn parse_operator_total_length(input: (&[u8], usize))-> IResult<(&[u8], usize), usize> {
    tuple((
        tag(0usize, 1usize),
        take(15usize)
    ))(input).map(|(left, (_, length))| (left, length))
}

fn parse_operator_subpacket_count(input: (&[u8], usize))-> IResult<(&[u8], usize), usize> {
    tuple((
        tag(1usize, 1usize),
        take(11usize)
    ))(input).map(|(left, (_, length))| (left, length))
}

fn parse_operator_packet(input: (&[u8], usize))-> IResult<(&[u8], usize), Packet> {
    tuple((
        take(3usize),

        ))
}

fn parse_packet(input: (&[u8], usize))-> IResult<(&[u8], Packet), u8> {
    tuple((
        parse_version,
        alt((
            (tag(TAG_LITERAL)))
            ))
}

fn decode_hex(input: &str) -> Vec<u8> {
    let mut v = Vec::with_capacity(input.len() / 2);

    for i in (0..input.len()-1).step_by(2) {
        dbg!(i);
        v.push(u8::from_str_radix(&input[i..i+2], 16).unwrap());
    }

    v
}

pub(crate) fn solve(input: String) -> Result<(), Report> {
    let v = decode_hex(&input);
    debug!("{:?}", v);

    Ok(())
}
