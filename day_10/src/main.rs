// https://adventofcode.com/2022/day/10

fn main() {
    let mut x = 1;
    let mut c = 0;
    let mut part_1 = Vec::new();
    let mut part_2 = String::new();

    for line in include_str!("../input").lines() {
        c += 1;
        process_cycle(c, x, &mut part_1, &mut part_2);

        if let Some(operand) = line.strip_prefix("addx ") {
            c += 1;
            process_cycle(c, x, &mut part_1, &mut part_2);

            x += operand.parse::<i32>().unwrap();
        }
    }

    let part_1 = part_1.iter().sum::<i32>();
    assert_eq!(11780, part_1);
    println!("Part 1: {}", part_1);

    print!("Part 2:\n{}", part_2);
    // PZULBAUA
}

fn process_cycle(c: i32, x: i32, part_1: &mut Vec<i32>, part_2: &mut String) {
    // Part 1
    if c == 20 || (c + 20) % 40 == 0 {
        part_1.push(c * x);
    }

    // Part 2
    let position = (c - 1) % 40;
    let sprite = (x - 1)..=(x + 1);
    if sprite.contains(&position) {
        part_2.push('#');
    } else {
        part_2.push('.');
    }
    if position == 39 {
        part_2.push('\n');
    }
}
