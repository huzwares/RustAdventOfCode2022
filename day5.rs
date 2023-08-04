use std::fs;

const COLUMNS: usize = 9;

fn parse_stack(input: &str, stack: &mut [Vec<char>]) {
    for line in input.lines().take(8) {
        let mut line = line.chars();
        let mut index = 0;
        while line.next().is_some() {
            if let Some(ch) = line.next() {
                if ch.is_ascii_uppercase() {
                    stack[index].push(ch);
                }
            }
            line.next();
            line.next();
            index += 1;
        }
    }
}

fn move_p1(input: &str, stack: &mut [Vec<char>]) {
    let (number, from, to) = parse_move(input);
    for _ in 0..number {
        let temp = stack[from].pop().unwrap();
        stack[to].push(temp);
    }
}

fn move_p2(input: &str, stack: &mut [Vec<char>]) {
    let (number, from, to) = parse_move(input);
    let mut temp: Vec<char> = vec![];
    for _ in 0..number {
        temp.push(stack[from].pop().unwrap());
    }
    temp.reverse();
    stack[to].append(&mut temp);
}

fn parse_move(input: &str) -> (usize, usize, usize) {
    let input: Vec<&str> = input
        .split(' ')
        .filter(|&ch| ch.parse::<i32>().is_ok())
        .collect();
    let number = input[0].parse::<usize>().unwrap();
    let from = input[1].parse::<usize>().unwrap() - 1;
    let to = input[2].parse::<usize>().unwrap() - 1;
    (number, from, to)
}

fn main() {
    let mut input = fs::read_to_string("input.txt").unwrap();
    let mut stack: Vec<Vec<char>> = vec![vec![]; COLUMNS];
    parse_stack(&input, &mut stack);
    for item in stack.iter_mut().take(COLUMNS) {
        item.reverse();
    }
    // We can calculate Part 2 in the same loop, but I choose to clone `stack` for use in Part 2.
    let mut stack_p2 = stack.clone();

    // Part 1
    for line in input.lines().skip(10) {
        move_p1(line, &mut stack);
    }
    let mut result = String::new();
    for mut col in stack {
        result.push(col.pop().unwrap());
    }
    println!("Part 1: {result}");

    // Part 2
    for line in input.lines().skip(10) {
        move_p2(line, &mut stack_p2);
    }
    let mut result_p2 = String::new();
    for mut col in stack_p2 {
        result_p2.push(col.pop().unwrap());
    }
    println!("Part 2: {result_p2}");
}