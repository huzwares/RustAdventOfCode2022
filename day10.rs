#[derive(Clone, Copy)]
enum Opration {
    Noop,
    Addx,
}

impl From<&str> for Opration {
    fn from(value: &str) -> Self {
        match value {
            "noop" => Opration::Noop,
            "addx" => Opration::Addx,
            _ => unreachable!(),
        }
    }
}
fn add_cycle(c: &mut i32, x: &mut i32, s: &mut Vec<i32>) {
    *c += 1;
    match c {
        20 | 60 | 100 | 140 | 180 | 220 => {
            s.push(*c * *x);
        }
        _ => (),
    }
}

fn set_pixel(c: &mut i32, x: &mut i32, screen: &mut [char]) {
    *c += 1;
    if ((*c - 1) % 40) >= *x - 1 && ((*c - 1) % 40) <= *x + 1 {
        screen[(*c - 1) as usize] = '#';
    }
}

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();

    // Part 1
    let mut x = 1;
    let mut cycle = 0;
    let mut signals = vec![];
    for line in input.lines() {
        let temp: Vec<&str> = line.split(' ').collect();
        let op = Opration::from(temp[0]);
        match op {
            Opration::Noop => {
                add_cycle(&mut cycle, &mut x, &mut signals);
            }
            Opration::Addx => {
                add_cycle(&mut cycle, &mut x, &mut signals);
                add_cycle(&mut cycle, &mut x, &mut signals);
                x += temp[1].parse::<i32>().unwrap();
            }
        }
    }
    println!("Part 1: {}", signals.iter().sum::<i32>());

    // Part 2
    x = 1;
    cycle = 0;
    let mut screen = ['.'; 40 * 6];
    for line in input.lines() {
        let temp: Vec<&str> = line.split(' ').collect();
        let op = Opration::from(temp[0]);
        match op {
            Opration::Noop => {
                set_pixel(&mut cycle, &mut x, &mut screen);
            }
            Opration::Addx => {
                set_pixel(&mut cycle, &mut x, &mut screen);
                set_pixel(&mut cycle, &mut x, &mut screen);
                x += temp[1].parse::<i32>().unwrap();
            }
        }
    }
    println!(
        "Part 2:\n{}",
        screen
            .chunks(40)
            .map(|row| row.iter().collect())
            .collect::<Vec<String>>()
            .join("\n")
    );
    Ok(())
}
