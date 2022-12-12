use std::collections::HashMap;

fn main() {
    let input = include_str!("../input");

    let monkeys: Vec<_> = input
        .split("\n\n")
        .map(|monkey| {
            let mut lines = monkey.lines();
            lines.next().unwrap(); // Monkey N:
            let (_, starting_items) = lines
                .next()
                .unwrap()
                .split_once("  Starting items: ")
                .unwrap();
            let starting_items: Vec<_> = starting_items
                .split(", ")
                .map(|n| n.parse::<u64>().unwrap())
                .collect();
            let (_, operation) = lines
                .next()
                .unwrap()
                .split_once("  Operation: new = old ")
                .unwrap();
            let (_, test_divisible_by) = lines
                .next()
                .unwrap()
                .split_once("  Test: divisible by ")
                .unwrap();
            let test_divisible_by: u64 = test_divisible_by.parse().unwrap();
            let (_, if_true) = lines
                .next()
                .unwrap()
                .split_once("    If true: throw to monkey ")
                .unwrap();
            let if_true: usize = if_true.parse().unwrap();
            let (_, if_false) = lines
                .next()
                .unwrap()
                .split_once("    If false: throw to monkey ")
                .unwrap();
            let if_false: usize = if_false.parse().unwrap();

            (
                starting_items,
                operation,
                test_divisible_by,
                if_true,
                if_false,
            )
        })
        .collect();
    let mut monkeys_1 = monkeys.clone();
    let mut monkeys_2 = monkeys;

    // Part 1
    let mut inspection_counts = HashMap::new();
    for _ in 0..20 {
        for i in 0..monkeys_1.len() {
            let (starting_items, operation, test_divisible_by, if_true, if_false) =
                monkeys_1[i].clone();
            for worry_level in starting_items {
                inspection_counts
                    .entry(i)
                    .and_modify(|v| *v += 1)
                    .or_insert(1);

                let mut worry_level = if let Some(n) = operation.strip_prefix("* ") {
                    if n == "old" {
                        worry_level * worry_level
                    } else {
                        let n: u64 = n.parse().unwrap();
                        worry_level * n
                    }
                } else if let Some(n) = operation.strip_prefix("+ ") {
                    if n == "old" {
                        worry_level + worry_level
                    } else {
                        let n: u64 = n.parse().unwrap();
                        worry_level + n
                    }
                } else {
                    unreachable!();
                };
                worry_level /= 3;
                let _ = monkeys_1.get_mut(i).unwrap().0.remove(0);
                if worry_level % test_divisible_by == 0 {
                    monkeys_1.get_mut(if_true).unwrap().0.push(worry_level);
                } else {
                    monkeys_1.get_mut(if_false).unwrap().0.push(worry_level);
                }
            }
        }
    }

    let mut inspection_counts: Vec<_> = inspection_counts.values().collect();
    inspection_counts.sort_unstable();
    let part_1 = inspection_counts.into_iter().rev().take(2).product::<u64>();
    assert_eq!(151312, part_1);
    println!("Part 1: {}", part_1);

    // Part 2
    //
    // super_modulo is used to keep the numbers from growing too large
    let super_modulo: u64 = monkeys_2.iter().map(|(_, _, n, _, _)| n).product();
    let mut inspection_counts = HashMap::new();
    for _ in 0..10000 {
        for i in 0..monkeys_2.len() {
            let (starting_items, operation, test_divisible_by, if_true, if_false) =
                monkeys_2[i].clone();
            for mut worry_level in starting_items {
                inspection_counts
                    .entry(i)
                    .and_modify(|v| *v += 1)
                    .or_insert(1);

                worry_level %= super_modulo;
                let worry_level = if let Some(n) = operation.strip_prefix("* ") {
                    if n == "old" {
                        worry_level * worry_level
                    } else {
                        let n: u64 = n.parse().unwrap();
                        worry_level * n
                    }
                } else if let Some(n) = operation.strip_prefix("+ ") {
                    if n == "old" {
                        worry_level + worry_level
                    } else {
                        let n: u64 = n.parse().unwrap();
                        worry_level + n
                    }
                } else {
                    unreachable!();
                };
                let _ = monkeys_2.get_mut(i).unwrap().0.remove(0);
                if worry_level % test_divisible_by == 0 {
                    monkeys_2.get_mut(if_true).unwrap().0.push(worry_level);
                } else {
                    monkeys_2.get_mut(if_false).unwrap().0.push(worry_level);
                }
            }
        }
    }

    let mut inspection_counts: Vec<_> = inspection_counts.values().collect();
    inspection_counts.sort_unstable();
    let part_2 = inspection_counts.into_iter().rev().take(2).product::<u64>();
    assert_eq!(51382025916, part_2);
    println!("Part 2: {}", part_2);
}
