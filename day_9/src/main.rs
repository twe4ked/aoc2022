// https://adventofcode.com/2022/day/9

use std::collections::HashSet;

#[derive(Default, Debug, Copy, Clone, PartialEq, Hash, Eq)]
struct Pos {
    x: i32,
    y: i32,
}

impl Pos {
    fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}

fn main() {
    let debug = false;
    let input = include_str!("../input");

    let mut part_1 = HashSet::new();
    let mut part_2 = HashSet::new();

    let mut rope = vec![Pos::default(); 10];

    if debug {
        println!("== Initial State ==");
        println!();
        draw(&rope);
    }

    for line in input.lines() {
        let (dir, steps) = line.split_once(' ').expect("invalid line");
        let steps: i32 = steps.parse().unwrap();

        if debug {
            println!();
            println!("== {} ==", line);
        }
        for _ in 0..steps {
            let (xx, yy) = direction(dir);
            rope.first_mut().unwrap().y += yy;
            rope.first_mut().unwrap().x += xx;

            move_knots(&mut rope);
            part_1.insert(*rope.get(1).unwrap());
            part_2.insert(*rope.last().unwrap());

            if debug {
                println!();
                draw(&rope);
            }
        }
    }

    let part_1 = part_1.len();
    assert_eq!(6212, part_1);
    println!("Part 1: {}", part_1);

    let part_2 = part_2.len();
    assert_eq!(2522, part_2);
    println!("Part 2: {}", part_2);
}

#[must_use]
fn direction(direction: &str) -> (i32, i32) {
    match direction {
        "U" => (0, -1),
        "D" => (0, 1),
        "L" => (-1, 0),
        "R" => (1, 0),
        _ => unreachable!(),
    }
}

fn move_knots(knots: &mut Vec<Pos>) {
    for i in 1..knots.len() {
        let prev = *knots.get(i - 1).unwrap();
        let curr = knots.get_mut(i).unwrap();
        move_tail(&prev, curr);
    }
}

fn move_tail(h: &Pos, t: &mut Pos) {
    #[rustfmt::skip]
    let moves = [
        (-1, -1), (0, -1), (1, -1),
        (-1,  0), (0,  0), (1,  0),
        (-1,  1), (0,  1), (1,  1),
    ];

    let adjacent_x = ((h.x - 1)..(h.x + 1)).contains(&t.x); // adjacent + overlapping
    let adjacent_y = ((h.y - 1)..(h.y + 1)).contains(&t.y); // adjacent + overlapping
    let touching = moves
        .iter()
        .any(|(xx, yy)| xx + t.x == h.x && yy + t.y == h.y);

    if touching {
        return;
    }

    if !touching {
        t.x = move_toward_by_1(t.x, h.x);
        t.y = move_toward_by_1(t.y, h.y);
    } else if !adjacent_x {
        t.x = move_toward_by_1(t.x, h.x);
    } else if !adjacent_y {
        t.y = move_toward_by_1(t.y, h.y);
    }
}

#[must_use]
// https://stackoverflow.com/a/69953858/826820
fn move_toward_by_1(value: i32, target: i32) -> i32 {
    let step = 1;
    i32::max(value - step, i32::min(value + step, target))
}

fn draw(rope: &[Pos]) {
    for y in -10..=10 {
        for x in -10..10 {
            let pos = Pos::new(x, y);

            if rope.contains(&pos) {
                let i = rope.iter().position(|x| x == &pos).unwrap();
                if i == 0 {
                    print!("H");
                } else if i == 10 {
                    print!("T");
                } else {
                    print!("{}", i);
                }
            } else if pos == Pos::new(0, 0) {
                print!("s");
            } else {
                print!(".");
            }
        }
        println!();
    }
}
