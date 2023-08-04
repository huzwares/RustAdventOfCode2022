use std::fs;

#[derive(Clone, Copy)]
enum Shape {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

#[derive(Clone, Copy)]
enum Round {
    Win = 6,
    Draw = 3,
    Lose = 0,
}

fn decode(hand: char) -> Shape {
    match hand {
        'A' | 'X' => Shape::Rock,
        'B' | 'Y' => Shape::Paper,
        'C' | 'Z' => Shape::Scissors,
        _ => {
            eprintln!("input error. hand shape not recognised.");
            std::process::exit(1);
        }
    }
}

fn score(hand_played: Shape, res: Round) -> i32 {
    hand_played as i32 + res as i32
}

// Determine if player 2 wins, loses, or draws the game
fn strategy_1(p1: Shape, p2: Shape) -> Round {
    match p1 {
        Shape::Rock => match p2 {
            Shape::Rock => Round::Draw,
            Shape::Paper => Round::Win,
            Shape::Scissors => Round::Lose,
        },
        Shape::Paper => match p2 {
            Shape::Rock => Round::Lose,
            Shape::Paper => Round::Draw,
            Shape::Scissors => Round::Win,
        },
        Shape::Scissors => match p2 {
            Shape::Rock => Round::Win,
            Shape::Paper => Round::Lose,
            Shape::Scissors => Round::Draw,
        },
    }
}

fn st2_decode(st: char) -> Round {
    match st {
        'X' => Round::Lose,
        'Y' => Round::Draw,
        'Z' => Round::Win,
        _ => {
            eprintln!("input error. hand shape not recognised.");
            std::process::exit(1);
        }
    }
}

fn strategy_2(p1: Shape, p2: Round) -> Shape {
    match p1 {
        Shape::Rock => match p2 {
            Round::Win => Shape::Paper,
            Round::Draw => Shape::Rock,
            Round::Lose => Shape::Scissors,
        },
        Shape::Paper => match p2 {
            Round::Win => Shape::Scissors,
            Round::Draw => Shape::Paper,
            Round::Lose => Shape::Rock,
        },
        Shape::Scissors => match p2 {
            Round::Win => Shape::Rock,
            Round::Draw => Shape::Scissors,
            Round::Lose => Shape::Paper,
        },
    }
}

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let mut sum = 0;
    // Part 1
    for line in input.clone().lines() {
        let players = line.chars().filter(|&c| c != ' ').collect::<Vec<char>>();
        let (p1, p2) = (decode(players[0]), decode(players[1]));
        sum += score(p2, strategy_1(p1, p2));
    }
    println!("Part 1: {sum}");

    // Part 2
    sum = 0;
    for line in input.lines() {
        let players = line.chars().filter(|&c| c != ' ').collect::<Vec<char>>();
        let (p1, p2) = (decode(players[0]), st2_decode(players[1]));
        sum += score(strategy_2(p1, p2), p2);
    }
    println!("Part 2: {sum}");
}