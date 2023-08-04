use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    // Part 1
    let mut sum = 0;
    for line in input.clone().lines() {
        let pairs: Vec<&str> = line.split(',').collect();
        let p1: Vec<&str> = pairs[0].split('-').collect();
        let p2: Vec<&str> = pairs[1].split('-').collect();
        if (p1[0].parse::<i32>().unwrap() >= p2[0].parse::<i32>().unwrap()
            && p1[1].parse::<i32>().unwrap() <= p2[1].parse::<i32>().unwrap())
            || (p2[0].parse::<i32>().unwrap() >= p1[0].parse::<i32>().unwrap()
                && p2[1].parse::<i32>().unwrap() <= p1[1].parse::<i32>().unwrap())
        {
            sum += 1;
        }
    }
    println!("Part 1: {sum}");

    // Part 2
    //
    // This code snippet produces the same output as the code below.
    // sum = 0;
    // for line in input.lines() {
    //     let pairs: Vec<&str> = line.split(',').collect();
    //     let p1: Vec<&str> = pairs[0].split('-').collect();
    //     let p2: Vec<&str> = pairs[1].split('-').collect();
    //     if (p2[0].parse::<i32>().unwrap() >= p1[0].parse::<i32>().unwrap()
    //         && p2[0].parse::<i32>().unwrap() <= p1[1].parse::<i32>().unwrap())
    //         || (p1[0].parse::<i32>().unwrap() >= p2[0].parse::<i32>().unwrap()
    //             && p1[0].parse::<i32>().unwrap() <= p2[1].parse::<i32>().unwrap())
    //     {
    //         sum += 1;
    //     }
    // }
    //
    sum = input
        .lines()
        .map(|line| line.split(',').collect::<Vec<&str>>())
        .map(|pair| {
            (
                pair[0].split('-').collect::<Vec<&str>>(),
                pair[1].split('-').collect::<Vec<&str>>(),
            )
        })
        .filter(|(p1, p2)| {
            (p2[0].parse::<i32>().unwrap() >= p1[0].parse::<i32>().unwrap()
                && p2[0].parse::<i32>().unwrap() <= p1[1].parse::<i32>().unwrap())
                || (p1[0].parse::<i32>().unwrap() >= p2[0].parse::<i32>().unwrap()
                    && p1[0].parse::<i32>().unwrap() <= p2[1].parse::<i32>().unwrap())
        })
        .count();
    println!("Part 2: {sum}");
}