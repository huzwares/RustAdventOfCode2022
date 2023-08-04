use std::fs;

fn priority(ch: char) -> i32 {
    if ch.is_ascii_lowercase() {
        return ch as i32 - 96;
    }
    ch as i32 - 38
}

fn main() {
    let mut input = fs::read_to_string("input.txt").unwrap();
    let mut sum = 0;
    // Part 1
    for line in input.clone().lines() {
        let (cmp1, cmp2) = line.split_at(line.len() / 2);
        let mut cmp1 = cmp1.chars().collect::<Vec<char>>();
        cmp1.sort();
        cmp1.dedup();
        for ch in cmp1 {
            if cmp2.find(ch).is_some() {
                sum += priority(ch);
            }
        }
    }
    println!("Part 1: {sum}");

    // Part 2
    sum = 0;
    let mut lines = input.lines();
    while let (Some(line1), Some(line2), Some(line3)) = (lines.next(), lines.next(), lines.next()) {
        let mut line1 = line1.chars().collect::<Vec<char>>();
        line1.sort();
        line1.dedup();
        for ch in line1 {
            if line2.find(ch).is_some() && line3.find(ch).is_some() {
                sum += priority(ch);
            }
        }
    }
    println!("Part 2: {sum}");
}
