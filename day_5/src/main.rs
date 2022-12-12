// https://adventofcode.com/2022/day/5

use std::collections::HashMap;

fn main() {
    let (stacks, moves) = include_str!("../input")
        .split_once("\n\n")
        .expect("invalid input");

    let stacks = parse_stacks(stacks);
    let mut part_1_stacks = Stacks::new(stacks.clone());
    let mut part_2_stacks = Stacks::new(stacks);

    for [quantity, from, to] in moves.lines().map(parse_move) {
        part_1_stacks.move_n(from, to, quantity);
        part_2_stacks.move_many_n(from, to, quantity);
    }

    let part_1 = part_1_stacks.answer();
    assert_eq!("ZBDRNPMVH", &part_1);
    println!("Part 1: {}", part_1);

    let part_2 = part_2_stacks.answer();
    assert_eq!("WDLPFNNNB", &part_2);
    println!("Part 2: {}", part_2);
}

type Map = HashMap<u32, Vec<char>>;
struct Stacks(Map, Option<Vec<char>>);

impl Stacks {
    fn new(map: Map) -> Self {
        Self(map, None)
    }

    fn move_n(&mut self, from: u32, to: u32, n: u32) {
        for _ in 0..n {
            let c = self.0.get_mut(&from).expect("stack").pop().expect("crate");
            self.0.get_mut(&to).expect("stack").push(c);
        }
    }

    fn move_many_n(&mut self, from: u32, to: u32, n: u32) {
        let mut temp = self
            .1
            .take()
            .unwrap_or_else(|| Vec::with_capacity(n as usize));
        for _ in 0..n {
            let c = self.0.get_mut(&from).expect("stack").pop().expect("crate");
            temp.push(c);
        }
        temp.reverse();
        self.0.get_mut(&to).expect("stack").append(&mut temp);
        // The temp Vec is drained by append() so we can store it again for re-use
        self.1 = Some(temp);
    }

    fn answer(self) -> String {
        let max_stack = *self.0.keys().max().expect("key");
        (1..=max_stack)
            .map(|stack_number| {
                self.0
                    .get(&stack_number)
                    .expect("stack")
                    .last()
                    .expect("crate")
            })
            .collect()
    }
}

fn parse_stacks(input: &str) -> Map {
    input
        .lines()
        // Reverse the lines so we build up stacks from the bottom,
        // then skip the first bottom most line, which are the stack numbers
        .rev()
        .skip(1)
        .fold(Map::new(), |mut stacks, line| {
            // Skip the first char then step by 4 chars so we jump to each interesting char
            for (i, c) in line.chars().skip(1).step_by(4).enumerate() {
                // Save anything that's not an empty space
                if c != ' ' {
                    let stack_number = i as u32 + 1;
                    stacks
                        .entry(stack_number)
                        .and_modify(|v: &mut Vec<char>| v.push(c))
                        .or_insert_with(|| vec![c]);
                }
            }
            stacks
        })
}

fn parse_move(line: &str) -> [u32; 3] {
    let mut parts = line.split(' ').skip(1).step_by(2);
    [parts.next(), parts.next(), parts.next()]
        .map(|p| p.expect("invalid line").parse().expect("invalid line"))
}
