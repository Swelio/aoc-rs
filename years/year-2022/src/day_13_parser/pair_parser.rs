use std::error::Error;

use pest::{
    iterators::{Pair, Pairs},
    Parser,
};

use utils::SantaError;

use crate::day_13_parser::Packet;

#[derive(pest_derive::Parser)]
#[grammar = "assets/day_13_packets.pest"]
struct PairParser;

pub fn parse_packet(input: &str) -> Result<Packet, Box<dyn Error>> {
    let packet_pair = PairParser::parse(Rule::packet, input)?
        .next()
        .unwrap()
        .into_inner()
        .next()
        .unwrap();
    parse_packet_pair(packet_pair)
}

pub fn parse_packet_pairs(input: &str) -> Result<(Packet, Packet), Box<dyn Error>> {
    let pairs = PairParser::parse(Rule::pair, input)?
        .next()
        .unwrap()
        .into_inner();
    let mut packets = parse_packets(pairs)?;
    let right_packet = packets.pop().ok_or_else(|| {
        SantaError::InvalidInput("Missing right packet from parsing.".to_string())
    })?;
    let left_packet = packets
        .pop()
        .ok_or_else(|| SantaError::InvalidInput("Missing left packet from parsing.".to_string()))?;

    Ok((left_packet, right_packet))
}

fn parse_packets(packet_pairs: Pairs<Rule>) -> Result<Vec<Packet>, Box<dyn Error>> {
    let mut packets = Vec::new();

    for pair in packet_pairs {
        let packet = parse_packet_pair(pair.into_inner().next().unwrap())?;
        packets.push(packet);
    }

    Ok(packets)
}

fn parse_packet_pair(packet_pair: Pair<Rule>) -> Result<Packet, Box<dyn Error>> {
    let packet = match packet_pair.as_rule() {
        Rule::number => Packet::Number(packet_pair.as_str().parse()?),
        Rule::list => {
            let mut packets = Vec::new();

            for pair in packet_pair.into_inner() {
                let sub_packet = parse_packet_pair(pair.into_inner().next().unwrap())?;
                packets.push(sub_packet);
            }

            Packet::List(packets)
        }
        _ => unreachable!(),
    };

    Ok(packet)
}
