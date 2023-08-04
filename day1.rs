use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();

    // Part 1
    let max = input
        .clone()
        .split("\n\n")
        .collect::<Vec<&str>>()
        .iter()
        .map(|elf| {
            elf.lines()
                .map(|line| line.parse::<i32>().unwrap())
                .sum::<i32>()
        })
        .max()
        .unwrap();
    println!("Part 1: {max}");

    // This code snippet produces the same output as Part 1.
    //
    // let elfs: Vec<&str> = input.split("\n\n").collect();
    // let mut max = 0;
    // for elf in elfs {
    //     let mut sum = 0;
    //     for line in elf.lines() {
    //         sum += line.parse::<i32>().unwrap();
    //     }
    //     if sum > max {
    //         max = sum
    //     }
    // }
    // println!("{max}");

    // Part 2
    let mut elfs = input
        .split("\n\n")
        .collect::<Vec<&str>>()
        .iter()
        .map(|elf| {
            elf.lines()
                .map(|line| line.parse::<i32>().unwrap())
                .sum::<i32>()
        })
        .collect::<Vec<i32>>();
    elfs.sort();
    println!(
        "Part 2: {}",
        elfs.pop().unwrap() + elfs.pop().unwrap() + elfs.pop().unwrap()
    );
}