// https://adventofcode.com/2022/day/13

use std::cmp::Ordering;

fn main() {
    let packets: Vec<_> =
        include_str!("../input")
            .split("\n\n")
            .fold(Vec::new(), |mut acc, pair| {
                let (left, right) = pair.split_once('\n').expect("pair");
                acc.push(Packet::parse(left));
                acc.push(Packet::parse(right));
                acc
            });

    let part_1: usize = packets
        .chunks_exact(2)
        .enumerate()
        .map(|(i, pair)| if pair[0] < pair[1] { i + 1 } else { 0 })
        .sum();
    assert_eq!(5605, part_1);
    println!("Part 1: {}", part_1);

    let div_1 = Packet::parse("[[2]]");
    let div_2 = Packet::parse("[[6]]");

    let packets = {
        let mut packets = packets;
        packets.push(div_1.clone());
        packets.push(div_2.clone());
        packets.sort_unstable();
        packets
    };

    let div_1_pos = packets.iter().position(|p| *p == div_1).expect("div_1") + 1;
    let div_2_pos = packets.iter().position(|p| *p == div_2).expect("div_2") + 1;
    let part_2 = div_1_pos * div_2_pos;
    assert_eq!(24969, part_2);
    println!("Part 2: {}", part_2);
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Packet {
    Integer(u32),
    List(Vec<Packet>),
}

impl Packet {
    fn parse(input: &str) -> Packet {
        fn parse_list(mut input: &str, mut list: Vec<Packet>) -> (Vec<Packet>, &str) {
            while !input.is_empty() {
                if let Some(rest) = input.strip_prefix('[') {
                    let (l, rest) = parse_list(rest, Vec::new());
                    input = rest;
                    list.push(Packet::List(l));
                } else if let Some(rest) = input.strip_prefix(']') {
                    return (list, rest);
                } else if let Some(rest) = input.strip_prefix(',') {
                    input = rest;
                } else if let Some(rest) = input.strip_prefix('\n') {
                    return (list, rest);
                } else {
                    let pos = input
                        .chars()
                        .position(|c| c == ',' || c == ']')
                        .expect(", or ]");
                    let (n, rest) = input.split_at(pos);
                    input = rest;
                    list.push(Packet::Integer(n.parse().expect("integer")));
                }
            }

            (list, input)
        }

        let (v, _) = parse_list(input, Vec::new());
        Packet::List(v)
    }
}

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Packet {
    fn cmp(&self, other: &Self) -> Ordering {
        use Packet::*;
        match (self, other) {
            (Integer(l), Integer(r)) => l.cmp(r),
            // Vec implements ordering lexicographically.
            // https://doc.rust-lang.org/std/cmp/trait.Ord.html#lexicographical-comparison
            (List(l_list), List(r_list)) => l_list.cmp(r_list),
            (List(_), Integer(r)) => self.cmp(&List(vec![Integer(*r)])),
            (Integer(l), List(_)) => List(vec![Integer(*l)]).cmp(other),
        }
    }
}
