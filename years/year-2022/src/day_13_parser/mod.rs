mod pair_parser;

use std::cmp::Ordering;

pub use pair_parser::{parse_packet, parse_packet_pairs};

#[derive(Clone, Debug, Eq)]
pub enum Packet {
    List(Vec<Packet>),
    Number(i32),
}

impl PartialEq for Packet {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Number(value), Self::Number(other)) => value.eq(other),
            (Self::List(packet), Self::List(other)) => packet.eq(other),
            (Self::Number(value), Self::List(other)) => {
                [Self::Number(*value)].as_slice().eq(other.as_slice())
            }
            (Self::List(_), Self::Number(_)) => other.eq(self),
        }
    }
}

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Packet {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Self::Number(value), Self::Number(other)) => value.cmp(other),
            (Self::List(packet), Self::List(other)) => packet.cmp(other),
            (Self::Number(value), Self::List(other)) => {
                [Self::Number(*value)].as_slice().cmp(other.as_slice())
            }
            (Self::List(_), Self::Number(_)) => other.cmp(self).reverse(),
        }
    }
}
