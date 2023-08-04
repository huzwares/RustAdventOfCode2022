use std::{collections::HashSet, fs};

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
struct Position {
    x: i32,
    y: i32,
}

#[derive(Hash, PartialEq, Eq, Clone, Copy)]
struct Knot {
    position: Position,
}

#[derive(Clone, Copy)]
enum Dir {
    Up,
    Right,
    Down,
    Left,
}

impl From<&str> for Dir {
    fn from(value: &str) -> Self {
        match value {
            "U" => Dir::Up,
            "R" => Dir::Right,
            "D" => Dir::Down,
            "L" => Dir::Left,
            _ => {
                eprintln!("Move not found");
                std::process::exit(1);
            }
        }
    }
}

impl Knot {
    fn new() -> Self {
        Self {
            position: Position { x: 0, y: 0 },
        }
    }
    fn move_knot(&mut self, dir: Dir) {
        match dir {
            Dir::Down => {
                self.position.y -= 1;
            }
            Dir::Left => {
                self.position.x -= 1;
            }
            Dir::Right => {
                self.position.x += 1;
            }
            Dir::Up => self.position.y += 1,
        }
    }
    fn calculate_distance(&self, other: &Knot) -> (i32, i32) {
        (
            self.position.x - other.position.x,
            self.position.y - other.position.y,
        )
    }
}

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    // Part 1
    let mut visited: HashSet<Knot> = HashSet::new();
    let mut head = Knot::new();
    let mut tail = Knot::new();
    visited.insert(tail);
    for line in input.lines() {
        let line: Vec<&str> = line.split(' ').collect();
        let (dir, step) = (Dir::from(line[0]), line[1].parse::<i32>().unwrap());
        for _ in 0..step {
            head.move_knot(dir);
            let (dist_x, dist_y) = head.calculate_distance(&tail);
            if dist_x.abs() > 1 || dist_y.abs() > 1 {
                tail.position.x += dist_x.signum();
                tail.position.y += dist_y.signum();
                visited.insert(tail);
            }
        }
    }
    println!("Part 1: {}", visited.len());

    // Part 2
    let mut rope = vec![Knot::new(); 10];
    visited = HashSet::new();
    visited.insert(rope[0]);
    for line in input.lines() {
        let line: Vec<&str> = line.split(' ').collect();
        let (dir, step) = (Dir::from(line[0]), line[1].parse::<i32>().unwrap());
        for _ in 0..step {
            rope[0].move_knot(dir);
            for (h_id, t_id) in (0..10)
                .collect::<Vec<usize>>()
                .windows(2)
                .map(|pair| (pair[0], pair[1]))
            {
                let (dist_x, dist_y) = rope[h_id].calculate_distance(&rope[t_id]);
                if dist_x.abs() > 1 || dist_y.abs() > 1 {
                    rope[t_id].position.x += dist_x.signum();
                    rope[t_id].position.y += dist_y.signum();
                    if t_id == 9 {
                        visited.insert(rope[t_id]);
                    }
                }
            }
        }
    }
    println!("Part 2: {}", visited.len());
}