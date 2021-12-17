use bitvec::prelude::*;
use nom_bitvec::BSlice;
use color_eyre::Report;
use nom::IResult;
use nom::branch::alt;
use nom::bytes::complete::{tag, take};
use nom::combinator::all_consuming;
use nom::multi::{count, length_count, length_value, many0};
use nom::sequence::{terminated, tuple};
use tracing::debug;

#[derive(Debug)]
struct Packet {
    version: u8,
    content: PacketType
}

impl Packet {
    fn sum_of_versions(&self) -> usize {
        self.version as usize + match &self.content {
            PacketType::Operator(op) => op.subpackets.iter().map(|p| p.sum_of_versions()).sum(),
            _ => 0
        }
    }
}

#[derive(Debug)]
enum PacketType {
    LiteralValue(LiteralValuePacket),
    Operator(OperatorPacket)
}

#[derive(Debug)]
struct LiteralValuePacket {
    value: u64
}

#[derive(Debug)]
struct OperatorPacket {
    type_id: u8,
    subpackets: Vec<Packet>
}

fn parse_version(input: BSlice<Msb0, u8>)-> IResult<BSlice<Msb0, u8>, u8> {
    take(3usize)(input).map(|(left, x)| (left, x.0[0..3].load_be::<u8>()))
}

fn build_number(groups: Vec<u8>, last_group: u8) -> u64 {
    debug!("Building number from {:?} and {:?}", groups, last_group);
    let mut result = 0;
    for group in groups {
        result |= (group & 0xf) as u64;
        result <<= 4;
    }
    result |= last_group as u64;
    result
}

fn parse_number(input: BSlice<Msb0, u8>)-> IResult<BSlice<Msb0, u8>, u64> {
    debug!("Parsing number from {:?}", input);
    tuple((
        many0(
            tuple((
                tag(BSlice(bits![1])),
                take(4usize)
            ))
        ),
        tuple((
            tag(BSlice(bits![0])),
            take(4usize)
        ))
    ))(input).map(|(left, (groups, (_, last_group)))|
        (left, build_number(groups.into_iter().map(|(_, group)| group.0.load_be::<u8>() ).collect(), last_group.0.load_be::<u8>()))
    )
}

fn parse_packet(input: BSlice<Msb0, u8>)-> IResult<BSlice<Msb0, u8>, Packet> {
    tuple((
        parse_version,
        alt((
            parse_literal_value_packet,
            parse_operator_packet
        ))
      ))(input).map(|(left, (version, content))| (left, Packet { version, content }))
}

fn parse_literal_value_packet(input: BSlice<Msb0, u8>)-> IResult<BSlice<Msb0, u8>, PacketType> {
    tuple((
        // "Literal value" tag is 4 (0b100)
        tag(BSlice(bits![1, 0, 0])),
        parse_number
    ))(input).map(|(left, (_, value))| (left, PacketType::LiteralValue(LiteralValuePacket { value })))
}

fn parse_operator_subpacket_total_length(input: BSlice<Msb0, u8>)-> IResult<BSlice<Msb0, u8>, usize> {
    tuple((
        tag(BSlice(bits![0])),
        take(15usize)
    ))(input).map(|(left, (_, value))| (left, value.0.load_be::<usize>()))
}

fn parse_operator_subpackets_by_total_length(input: BSlice<Msb0, u8>)-> IResult<BSlice<Msb0, u8>, Vec<Packet>> {
    length_value(
        parse_operator_subpacket_total_length,
        many0(parse_packet)
    )(input)
}

fn parse_operator_subpacket_count(input: BSlice<Msb0, u8>)-> IResult<BSlice<Msb0, u8>, usize> {
    tuple((
        tag(BSlice(bits![1])),
        take(11usize)
    ))(input).map(|(left, (_, value))| (left, value.0.load_be::<usize>()))
}

fn parse_operator_subpackets_by_count(input: BSlice<Msb0, u8>)-> IResult<BSlice<Msb0, u8>, Vec<Packet>> {
    length_count(
        parse_operator_subpacket_count,
        parse_packet
    )(input)
}

fn parse_operator_packet(input: BSlice<Msb0, u8>)-> IResult<BSlice<Msb0, u8>, PacketType> {
    tuple((
        take(3usize),
        alt((
            parse_operator_subpackets_by_total_length,
            parse_operator_subpackets_by_count
        ))
    ))(input)
        .map(|(left, (type_id, subpackets))|
            (left, PacketType::Operator(
                OperatorPacket { type_id: type_id.0.load_be::<u8>(), subpackets }
            ))
        )
}

fn decode_hex(input: &str) -> Vec<u8> {
    let mut v = Vec::with_capacity(input.len() / 2);

    for i in (0..input.len()-1).step_by(2) {
        v.push(u8::from_str_radix(&input[i..i+2], 16).unwrap());
    }

    v
}

pub(crate) fn solve(input: String) -> Result<(), Report> {
    let v = decode_hex(&input);
    let bits = v.view_bits::<Msb0>();
    let packet = match parse_packet(BSlice(bits)) {
        Ok((slice, packet)) if !slice.0.any() => packet,
        Ok((slice, _)) => return Err(Report::msg(format!("Leftover non-zero bits: {:?}", slice))),
        Err(e) => return Err(Report::msg(format!("Failed to parse input: {:?}", e)))
    };

    println!("sum of versions: {:?}", packet.sum_of_versions());

    Ok(())
}
