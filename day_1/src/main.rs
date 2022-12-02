// https://adventofcode.com/2022/day/1

fn main() {
    let [a, b, c, _] = include_str!("../input")
        .lines()
        .map(|l| l.parse::<u32>().ok())
        .fold(
            [
                0, 0, 0, // highest 3 numbers
                0, // current sum
            ],
            |mut acc, calories| {
                if let Some(c) = calories {
                    acc[3] += c;
                    acc
                } else {
                    acc.sort_unstable();
                    [acc[3], acc[2], acc[1], 0]
                }
            },
        );

    let part_1 = a;
    assert_eq!(74394, part_1);
    println!("Part 1: {}", part_1);

    let part_2 = a + b + c;
    assert_eq!(212836, part_2);
    println!("Part 2: {}", part_2);
}
