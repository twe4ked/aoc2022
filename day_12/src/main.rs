// https://adventofcode.com/2022/day/12

use std::collections::HashMap;

fn main() {
    let (map, start, end) = {
        let mut map: Vec<Vec<char>> = include_str!("../input")
            .lines()
            .map(|l| l.chars().collect())
            .collect();
        let mut start = None;
        let mut end = None;
        for (y, row) in map.iter_mut().enumerate() {
            for (x, c) in row.iter_mut().enumerate() {
                if *c == 'S' {
                    *c = 'a';
                    start = Some((x as i32, y as i32));
                }

                if *c == 'E' {
                    *c = 'z';
                    end = Some((x as i32, y as i32));
                }
            }
        }
        (map, start.expect("start"), end.expect("end"))
    };

    let part_1 = a_star(&map, start, end).expect("part_1");
    assert_eq!(497, part_1);
    println!("Part 1: {}", part_1);

    let mut starting_points = Vec::new();
    for (y, row) in map.iter().enumerate() {
        for (x, c) in row.iter().enumerate() {
            if c == &'a' {
                starting_points.push((x as i32, y as i32));
            }
        }
    }
    let mut part_2 = usize::MAX;
    for s in starting_points {
        if let Some(count) = a_star(&map, s, end) {
            part_2 = part_2.min(count);
        }
    }
    assert_eq!(492, part_2);
    println!("Part 2: {}", part_2);
}

fn a_star(map: &Vec<Vec<char>>, start: (i32, i32), end: (i32, i32)) -> Option<usize> {
    let mut open = HashMap::new();
    let mut closed = HashMap::new();
    let mut f_scores = HashMap::new();
    let mut g_scores: HashMap<(i32, i32), i32> = HashMap::new();
    let mut came_from = HashMap::new();

    let start = (start.0, start.1);
    g_scores.insert((start.0, start.1), 0);
    open.insert((start.0, start.1), start);

    let mut found = None;
    while !open.is_empty() {
        if open.len() % 5000 == 0 {
            println!("{}", open.len());
        }
        let (_, current) = open
            .iter()
            .min_by_key(|(k, _)| *f_scores.get(*k).unwrap_or(&i32::MAX))
            .unwrap();

        let current = open.remove(&(current.0, current.1)).unwrap();
        closed.insert((current.0, current.1), current);

        if current.0 == end.0 && current.1 == end.1 {
            found = Some(current);
            break;
        }

        let walkable_children = [(0, -1), (-1, 0), (1, 0), (0, 1)]
            .iter()
            .map(|(x, y)| (current.0 + x, current.1 + y))
            .filter(|(x, y)| *x >= 0 && *y >= 0)
            .filter(|(x, y)| *x < map[0].len() as i32 && *y < map.len() as i32)
            .filter(|(x, y)| {
                let current = map[current.1 as usize][current.0 as usize];
                let child = map[*y as usize][*x as usize];
                can_move(current, child)
            });

        for (child_x, child_y) in walkable_children {
            if closed.contains_key(&(child_x, child_y)) {
                continue;
            }
            let child = (child_x, child_y);

            let h = 0;
            let g_score = g_scores[&(current.0, current.1)] + 1;
            let f_score = g_score + h;

            if let Some(open_node) = open.get(&(child.0, child.1)) {
                let open_node_g = g_scores[&(open_node.0, open_node.1)];
                if g_score > open_node_g {
                    continue;
                }
            }

            came_from.insert(child, current);
            open.insert((child_x, child_y), child);
            f_scores.insert((child_x, child_y), f_score);
            g_scores.insert((child_x, child_y), g_score);
        }
    }

    found.map(|f| count_path(&came_from, f))
}

fn can_move(current_tile: char, neighbor_tile: char) -> bool {
    neighbor_tile as i32 - current_tile as i32 <= 1
}

fn count_path(came_from: &HashMap<(i32, i32), (i32, i32)>, mut current: (i32, i32)) -> usize {
    let mut total_path = 0;
    while let Some(n) = came_from.get(&current) {
        current = *n;
        total_path += 1;
    }
    total_path
}
