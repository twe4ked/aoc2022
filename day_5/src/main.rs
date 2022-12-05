// https://adventofcode.com/2022/day/5

type Stacks = std::collections::HashMap<u32, Vec<char>>;

fn main() {
    let (stacks, moves) = include_str!("../input")
        .split_once("\n\n")
        .expect("invalid input");

    let stacks = parse_stacks(stacks);
    let mut part_1_stacks = stacks.clone();
    let mut part_2_stacks = stacks;

    let mut part_2_crates_temp = Vec::new();
    for (quantity, from, to) in moves.lines().map(parse_move) {
        // Part 1
        for _ in 0..quantity {
            let c = part_1_stacks
                .get_mut(&from)
                .expect("stack")
                .pop()
                .expect("crate");
            part_1_stacks.get_mut(&to).expect("stack").push(c);
        }

        // Part 2
        for c in (0..quantity).map(|_| {
            part_2_stacks
                .get_mut(&from)
                .expect("stack")
                .pop()
                .expect("crate")
        }) {
            part_2_crates_temp.push(c);
        }
        part_2_crates_temp.reverse();
        part_2_stacks
            .get_mut(&to)
            .expect("stack")
            .append(&mut part_2_crates_temp);
    }

    let part_1 = answer(&part_1_stacks);
    assert_eq!("ZBDRNPMVH", &part_1);
    println!("Part 1: {}", part_1);

    let part_2 = answer(&part_2_stacks);
    assert_eq!("WDLPFNNNB", &part_2);
    println!("Part 2: {}", part_2);
}

fn parse_stacks(input: &str) -> Stacks {
    input
        .lines()
        // Reverse the lines so we build up stacks from the bottom,
        // then skip the first bottom most line, which are the stack numbers
        .rev()
        .skip(1)
        .fold(Stacks::new(), |mut stacks, line| {
            // Skip the first char then step by 4 chars so we jump to each interesting char
            for (i, c) in line.chars().skip(1).step_by(4).enumerate() {
                // Save anything that's not an empty space
                if c != ' ' {
                    let stack_number = i as u32 + 1;
                    stacks
                        .entry(stack_number)
                        .and_modify(|v| v.push(c))
                        .or_insert_with(|| vec![c]);
                }
            }
            stacks
        })
}

fn parse_move(line: &str) -> (u32, u32, u32) {
    let mut parts = line.split(' ').skip(1).step_by(2);
    (
        parts
            .next()
            .expect("invalid line")
            .parse()
            .expect("invalid line"),
        parts
            .next()
            .expect("invalid line")
            .parse()
            .expect("invalid line"),
        parts
            .next()
            .expect("invalid line")
            .parse()
            .expect("invalid line"),
    )
}

fn answer(stacks: &Stacks) -> String {
    let max_stack = *stacks.keys().max().expect("key");
    (1..=max_stack)
        .map(|stack_number| {
            stacks
                .get(&stack_number)
                .expect("stack")
                .last()
                .expect("crate")
        })
        .collect()
}
