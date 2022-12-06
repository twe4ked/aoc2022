// https://adventofcode.com/2022/day/6

#[derive(Default)]
struct UniqueChecker(std::collections::HashSet<char>);

impl UniqueChecker {
    fn check(&mut self, s: &str) -> bool {
        let b = s.chars().all(|c| self.0.insert(c));
        self.0.clear();
        b
    }
}

fn main() {
    let input = include_str!("../input");

    let mut unique_checker = UniqueChecker::default();

    let mut part_1 = None;
    let mut part_2 = None;

    for i in 0..input.len() {
        // Part 1
        if part_1.is_none() && i > 3 && unique_checker.check(&input[i - 4..i]) {
            part_1 = Some(i);

            if part_2.is_some() {
                break;
            }
        }

        // Part 2
        if part_2.is_none() && i > 13 && unique_checker.check(&input[i - 14..i]) {
            part_2 = Some(i);

            if part_1.is_some() {
                break;
            }
        }
    }

    let part_1 = part_1.expect("part 1 answer");
    assert_eq!(1282, part_1);
    println!("Part 1: {}", part_1);

    let part_2 = part_2.expect("part 2 answer");
    assert_eq!(3513, part_2);
    println!("Part 2: {}", part_2);
}
