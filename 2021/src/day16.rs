use core::{num, panic};
use std::collections::HashMap;

pub fn run() {
    let input = std::fs::read_to_string("src/day16_input.txt").unwrap();
    let packets = parse(&input);

    assert_eq!(1, packets.len());

    println!("Day 16A: {}", sum_versions(&packets));
    println!("Day 16B: {}", packets[0].expression.evaluate());
}

#[derive(Debug, PartialEq, Eq)]
struct Packet {
    version: u8,
    type_id: u8,
    expression: Expression,
}

#[derive(Debug, PartialEq, Eq)]
enum Expression {
    Literal(u64),
    Sum(Vec<Packet>),
    Product(Vec<Packet>),
    Minimum(Vec<Packet>),
    Maximum(Vec<Packet>),
    GreaterThan(Vec<Packet>),
    LessThan(Vec<Packet>),
    Equal(Vec<Packet>),
}

impl Expression {
    fn evaluate(&self) -> u64 {
        return match self {
            Self::Literal(v) => *v,
            Self::Sum(children) => children.iter().map(|p| p.expression.evaluate()).sum(),
            Self::Product(children) => children.iter().map(|p| p.expression.evaluate()).product(),
            Self::Minimum(children) => children
                .iter()
                .map(|p| p.expression.evaluate())
                .min()
                .unwrap(),
            Self::Maximum(children) => children
                .iter()
                .map(|p| p.expression.evaluate())
                .max()
                .unwrap(),
            Self::GreaterThan(children) => {
                assert_eq!(2, children.len());
                let a = children[0].expression.evaluate();
                let b = children[1].expression.evaluate();
                if a > b {
                    1
                } else {
                    0
                }
            }
            Self::LessThan(children) => {
                assert_eq!(2, children.len());
                let a = children[0].expression.evaluate();
                let b = children[1].expression.evaluate();
                if a < b {
                    1
                } else {
                    0
                }
            }
            Self::Equal(children) => {
                assert_eq!(2, children.len());
                let a = children[0].expression.evaluate();
                let b = children[1].expression.evaluate();
                if a == b {
                    1
                } else {
                    0
                }
            }
        };
    }
}

fn read_header(input: &str, index: usize) -> (u8, u8, usize) {
    let version_str = &input[index..(index + 3)];
    let version = u8::from_str_radix(version_str, 2).unwrap();
    let type_id_str = &input[(index + 3)..(index + 6)];
    let type_id = u8::from_str_radix(type_id_str, 2).unwrap();

    return (version, type_id, index + 6);
}

fn read_decimal(input: &str, index: usize) -> (Expression, usize) {
    let mut i = index;
    let mut num = String::new();

    loop {
        let bits = &input[i..(i + 5)];

        let is_last_group = &bits[0..1] == "0";
        num.push_str(&bits[1..]);

        i += 5;

        if is_last_group {
            break;
        }
    }

    let parsed = u64::from_str_radix(&num, 2).unwrap();

    return (Expression::Literal(parsed), i);
}

fn read_operator_packets(input: &str, index: usize) -> (Vec<Packet>, usize) {
    let length_type = &input[index..(index + 1)];
    let mut packets = Vec::new();
    let next: usize = match length_type {
        "0" => {
            let num_bits_str = &input[(index + 1)..(index + 16)];
            let num_bits = usize::from_str_radix(num_bits_str, 2).unwrap();
            let mut next = index + 16;
            let index_when_done = next + num_bits;

            while next < index_when_done {
                let (packet, n) = read_packet(&input, next).unwrap();
                packets.push(packet);
                next = n;
            }

            index + 16 + num_bits
        }
        "1" => {
            let num_packets_str = &input[(index + 1)..(index + 12)];
            let num_packets = usize::from_str_radix(num_packets_str, 2).unwrap();
            let mut next = index + 12;

            for _ in 0..num_packets {
                let (packet, n) = read_packet(input, next).unwrap();
                next = n;
                packets.push(packet);
            }

            next
        }
        _ => panic!("Unknown length type {}.", length_type),
    };

    return (packets, next);
}

fn read_packet(input: &str, index: usize) -> Option<(Packet, usize)> {
    // detect hex padding.
    if (input.len() - index) < 8 {
        return None;
    }

    let (version, type_id, next) = read_header(input, index);
    let (expr, next) = match type_id {
        4 => read_decimal(input, next),
        _ => {
            let (children, next) = read_operator_packets(input, next);
            let expr = match type_id {
                0 => Expression::Sum(children),
                1 => Expression::Product(children),
                2 => Expression::Minimum(children),
                3 => Expression::Maximum(children),
                5 => Expression::GreaterThan(children),
                6 => Expression::LessThan(children),
                7 => Expression::Equal(children),
                _ => panic!("Unexpected type_id {}.", type_id),
            };
            (expr, next)
        }
    };

    let packet = Packet {
        version,
        type_id,
        expression: expr,
    };

    return Some((packet, next));
}

fn to_binary_string(input: &str) -> String {
    return input
        .trim()
        .chars()
        .map(|ch| {
            let num = u8::from_str_radix(&ch.to_string(), 16).unwrap();
            return format!("{:04b}", num);
        })
        .collect::<String>();
}

fn parse(input: &str) -> Vec<Packet> {
    let bin_str = to_binary_string(input);
    let mut packets = Vec::<Packet>::new();
    let mut i = 0;

    loop {
        if let Some((packet, next)) = read_packet(&bin_str, i) {
            packets.push(packet);
            i = next;
        } else {
            break;
        }
    }

    return packets;
}

fn sum_versions(packets: &[Packet]) -> u64 {
    return packets.iter().fold(0, |carry, p| {
        return carry
            + p.version as u64
            + match &p.expression {
                Expression::Literal(_) => 0,
                Expression::Sum(children) => sum_versions(&children),
                Expression::Product(children) => sum_versions(&children),
                Expression::Minimum(children) => sum_versions(&children),
                Expression::Maximum(children) => sum_versions(&children),
                Expression::GreaterThan(children) => sum_versions(&children),
                Expression::LessThan(children) => sum_versions(&children),
                Expression::Equal(children) => sum_versions(&children),
            };
    });
}

mod tests {
    use crate::day16::*;

    #[test]
    fn converts_to_binary() {
        assert_eq!("110100101111111000101000", to_binary_string("D2FE28"));
        assert_eq!(
            "00111000000000000110111101000101001010010001001000000000",
            to_binary_string("38006F45291200")
        );
    }

    #[test]
    fn reads_header_correctly() {
        let str = to_binary_string("D2FE28");
        let (version, type_id, next) = read_header(&str, 0);

        assert_eq!(6, version);
        assert_eq!(4, type_id);
        assert_eq!(6, next);
    }

    #[test]
    fn reads_decimals_correctly() {
        let str = to_binary_string("D2FE28");
        let (data, next) = read_decimal(&str, 6);

        assert_eq!(Expression::Literal(2021), data);
        assert_eq!(21, next);
    }

    #[test]
    fn reads_operator_correctly() {
        let packets = parse("38006F45291200");

        assert_eq!(1, packets.len());
        assert_eq!(
            Packet {
                version: 1,
                type_id: 6,
                expression: Expression::LessThan(vec![
                    Packet {
                        version: 6,
                        type_id: 4,
                        expression: Expression::Literal(10),
                    },
                    Packet {
                        version: 2,
                        type_id: 4,
                        expression: Expression::Literal(20),
                    },
                ])
            },
            packets[0]
        );
    }
}
