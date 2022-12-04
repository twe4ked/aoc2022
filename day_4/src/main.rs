// https://adventofcode.com/2022/day/4

fn main() {
    fn parse(line: &str) -> [u32; 4] {
        let (elf_1, elf_2) = line.split_once(',').expect("line format");
        let (min_1, max_1) = elf_1.split_once('-').expect("line format");
        let (min_2, max_2) = elf_2.split_once('-').expect("line format");
        [min_1, max_1, min_2, max_2].map(|n| n.parse().expect("integer"))
    }

    let (part_1, part_2) = include_str!("../input").lines().map(parse).fold(
        (0, 0),
        |mut acc, [min_1, max_1, min_2, max_2]| {
            // Part 1
            if (min_1 <= min_2 && max_1 >= max_2) || (min_2 <= min_1 && max_2 >= max_1) {
                acc.0 += 1;
            }

            // Part 2
            if !(min_1 > max_2 || min_2 > max_1) {
                acc.1 += 1;
            }

            acc
        },
    );

    assert_eq!(599, part_1);
    println!("Part 1: {}", part_1);

    assert_eq!(928, part_2);
    println!("Part 2: {}", part_2);
}
