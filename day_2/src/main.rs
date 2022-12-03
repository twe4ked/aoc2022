// https://adventofcode.com/2022/day/2

const ROCK_SCORE: u32 = 1;
const PAPER_SCORE: u32 = 2;
const SCISSORS_SCORE: u32 = 3;

const LOSS_SCORE: u32 = 0;
const DRAW_SCORE: u32 = 3;
const WIN_SCORE: u32 = 6;

fn main() {
    // A for Rock, B for Paper, and C for Scissors
    // X for Rock, Y for Paper, and Z for Scissors

    // Follow the strategy and calculate the score.
    let part_1 = |row| match row {
        "A X" => ROCK_SCORE + DRAW_SCORE,
        "B X" => ROCK_SCORE + LOSS_SCORE,
        "C X" => ROCK_SCORE + WIN_SCORE,
        "A Y" => PAPER_SCORE + WIN_SCORE,
        "B Y" => PAPER_SCORE + DRAW_SCORE,
        "C Y" => PAPER_SCORE + LOSS_SCORE,
        "A Z" => SCISSORS_SCORE + LOSS_SCORE,
        "B Z" => SCISSORS_SCORE + WIN_SCORE,
        "C Z" => SCISSORS_SCORE + DRAW_SCORE,
        _ => unreachable!(),
    };

    // The second column says how the round needs to end: X means you need to lose, Y means you
    // need to end the round in a draw, and Z means you need to win.
    let part_2 = |row| match row {
        "A X" => SCISSORS_SCORE + LOSS_SCORE,
        "B X" => ROCK_SCORE + LOSS_SCORE,
        "C X" => PAPER_SCORE + LOSS_SCORE,
        "A Y" => ROCK_SCORE + DRAW_SCORE,
        "B Y" => PAPER_SCORE + DRAW_SCORE,
        "C Y" => SCISSORS_SCORE + DRAW_SCORE,
        "A Z" => PAPER_SCORE + WIN_SCORE,
        "B Z" => SCISSORS_SCORE + WIN_SCORE,
        "C Z" => ROCK_SCORE + WIN_SCORE,
        _ => unreachable!(),
    };

    let (part_1, part_2) = include_str!("../input").lines().fold((0, 0), |acc, row| {
        (acc.0 + part_1(row), acc.1 + part_2(row))
    });

    assert_eq!(12855, part_1);
    println!("Part 1: {}", part_1);

    assert_eq!(13726, part_2);
    println!("Part 2: {}", part_2);
}
