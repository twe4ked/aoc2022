// https://adventofcode.com/2022/day/3

fn main() {
    let input = include_str!("../input");

    // a-z 1-26, A-Z 27-52
    let priority = |c: char| c as u32 - if c.is_lowercase() { 96 } else { 38 };

    let part_1: u32 = input
        .lines()
        .map(|line| {
            let (compartment_1, compartment_2) = line.split_at(line.len() / 2);
            compartment_1
                .chars()
                .find(|&c| compartment_2.contains(c))
                .expect("must have a duplicate")
        })
        .map(priority)
        .sum();

    assert_eq!(8072, part_1);
    println!("Part 1: {}", part_1);

    let part_2: u32 = input
        .lines()
        .collect::<Vec<_>>()
        .chunks(3)
        .map(|group| {
            group[0]
                .chars()
                .find(|&c| group[1].contains(c) && group[2].contains(c))
                .expect("must have a badge")
        })
        .map(priority)
        .sum();

    assert_eq!(2567, part_2);
    println!("Part 2: {}", part_2);
}
