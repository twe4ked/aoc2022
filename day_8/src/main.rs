// https://adventofcode.com/2022/day/8

fn main() {
    let input = include_str!("../input");

    let grid = input
        .lines()
        .map(|l| {
            l.chars()
                .map(|c| (c as u32 - '0' as u32) as u8)
                .collect::<Vec<u8>>()
        })
        .collect::<Vec<Vec<u8>>>();

    let grid_height = grid.len();
    let grid_width = grid.first().map(|r| r.len()).unwrap();

    let mut part_1 = (grid_height * 2 + grid_width * 2)
        // Don't count the corners twice
        - 4;
    let mut part_2 = 0;

    for y in 1..(grid_height - 1) {
        for x in 1..(grid_width - 1) {
            let height = grid[y][x];

            // Part 1
            // Up, down, left, right
            if (0..y).all(|yy| grid[yy][x] < height)
                || ((y + 1)..grid_height).all(|yy| grid[yy][x] < height)
                || (0..x).all(|xx| grid[y][xx] < height)
                || ((x + 1)..grid_width).all(|xx| grid[y][xx] < height)
            {
                part_1 += 1;
            }

            // Part 2
            let mut view_dist_top = 0;
            for yy in (0..y).rev() {
                view_dist_top += 1;
                if grid[yy][x] >= height {
                    break;
                }
            }
            let mut view_dist_bottom = 0;
            for row in grid.iter().take(grid_height).skip(y + 1) {
                view_dist_bottom += 1;
                if row[x] >= height {
                    break;
                }
            }
            let mut view_dist_left = 0;
            for xx in (0..x).rev() {
                view_dist_left += 1;
                if grid[y][xx] >= height {
                    break;
                }
            }
            let mut view_dist_right = 0;
            for xx in (x + 1)..grid_width {
                view_dist_right += 1;
                if grid[y][xx] >= height {
                    break;
                }
            }
            let scenic_score = view_dist_top * view_dist_bottom * view_dist_left * view_dist_right;
            part_2 = part_2.max(scenic_score);
        }
    }

    assert_eq!(1843, part_1);
    println!("Part 1: {}", part_1);

    assert_eq!(180000, part_2);
    println!("Part 2: {}", part_2);
}
