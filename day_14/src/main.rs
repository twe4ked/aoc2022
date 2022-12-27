// https://adventofcode.com/2022/day/14

// If-let chains aren't stable
#![allow(clippy::unnecessary_unwrap)]

use std::collections::HashSet;

fn main() {
    // Parse and build cave
    let mut cave = HashSet::new();
    let mut max_y = 0;
    for line in include_str!("../input").lines() {
        let mut last_position: Option<(usize, usize)> = None;

        for position in line.split(" -> ") {
            let (x, y) = position.split_once(',').unwrap();
            let x: usize = x.parse().unwrap();
            let y: usize = y.parse().unwrap();
            max_y = max_y.max(y);

            if let Some(last_position) = last_position {
                if last_position.0 == x {
                    for yy in range(last_position.1, y) {
                        cave.insert((last_position.0, yy));
                    }
                } else {
                    for xx in range(last_position.0, x) {
                        cave.insert((xx, last_position.1));
                    }
                }
            }

            last_position = Some((x, y))
        }
    }

    let mut part_1 = 0;
    let mut cave_part_1 = cave.clone();
    'a: loop {
        let mut sand = (500, 0);
        loop {
            if sand.1 > max_y {
                break 'a;
            }

            if let Some(s) = move_sand(sand, &cave_part_1) {
                sand = s;
            } else {
                part_1 += 1;
                cave_part_1.insert(sand);
                break;
            }
        }
    }
    println!("Part 1: {}", part_1);
    assert_eq!(625, part_1);

    let mut part_2 = 0;
    let mut cave_part_2 = cave;
    let floor = max_y + 2;
    loop {
        let mut sand = (500, 0);
        if cave_part_2.contains(&sand) {
            break;
        }

        loop {
            let m = move_sand(sand, &cave_part_2);
            if m.is_some() && sand.1 + 1 != floor {
                sand = m.unwrap();
            } else {
                cave_part_2.insert(sand);
                part_2 += 1;
                break;
            }
        }
    }
    println!("Part 2: {}", part_2);
    assert_eq!(25193, part_2);
}

fn range(a: usize, b: usize) -> std::ops::Range<usize> {
    std::ops::Range {
        start: a.min(b),
        end: a.max(b) + 1,
    }
}

#[must_use]
fn move_sand(sand: (usize, usize), cave: &HashSet<(usize, usize)>) -> Option<(usize, usize)> {
    if !cave.contains(&(sand.0, sand.1 + 1)) {
        Some((sand.0, sand.1 + 1))
    } else if !cave.contains(&(sand.0 - 1, sand.1 + 1)) {
        Some((sand.0 - 1, sand.1 + 1))
    } else if !cave.contains(&(sand.0 + 1, sand.1 + 1)) {
        Some((sand.0 + 1, sand.1 + 1))
    } else {
        None
    }
}
