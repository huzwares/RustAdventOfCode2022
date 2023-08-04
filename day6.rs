use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let input = input.chars().collect::<Vec<char>>();

    // Part 1
    let iter = input.windows(4);
    let mut counter = 4;
    for sl in iter {
        let mut temp = sl.to_vec();
        temp.sort();
        temp.dedup();
        if temp.len() < 4 {
            counter += 1;
        }
        if temp.len() == 4 {
            break;
        }
    }
    println!("Part 1: {counter}");

    // Part 2
    let iter2 = input.windows(14);
    counter = 14;
    for sl in iter2 {
        let mut temp = sl.to_vec();
        temp.sort();
        temp.dedup();
        if temp.len() < 14 {
            counter += 1;
        }
        if temp.len() == 14 {
            break;
        }
    }
    println!("Part 2: {counter}");
}