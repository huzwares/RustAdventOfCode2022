fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let width = input.lines().next().unwrap().chars().count();
    let height = input.lines().count();
    let _jungle: Vec<Vec<u32>> = input
        .lines()
        .map(|l| {
            l.chars()
                .filter(|ch| ch.is_ascii_digit())
                .map(|ch| ch.to_digit(10).unwrap())
                .collect::<Vec<u32>>()
        })
        .collect();

    // Part 1
    let mut sum = 0;
    for row in 0..width {
        for col in 0..height {
            if row == 0 || row == 98 || col == 0 || col == 98 {
                sum += 1;
                continue;
            }
            if is_visible(&_jungle, row, col) {
                sum += 1;
            }
        }
    }
    println!("Part 1: {}", sum);

    // part 2
    let mut max = 0;
    for row in 0..width {
        for col in 0..height {
            if row == 0 || row == 98 || col == 0 || col == 98 {
                continue;
            }
            let temp = calc_distance(&_jungle, row, col);
            if temp > max {
                max = temp;
            }
        }
    }
    println!("Part 2: {}", max);
    Ok(())
}

fn is_visible(jungle: &[Vec<u32>], row: usize, col: usize) -> bool {
    let tree = jungle[row][col];
    if up_visible(jungle, tree, row, col)
        || left_visible(jungle, tree, row, col)
        || right_visible(jungle, tree, row, col)
        || down_visible(jungle, tree, row, col)
    {
        return true;
    }
    false

    // I implemented the same functionality in a different way. Please see the commented function below.
    // if go_up(j, tree, row, col)
    //     || go_right(j, tree, row, col)
    //     || go_down(j, tree, row, col)
    //     || go_left(j, tree, row, col)
    // {
    //     return true;
    // }
    // false
}

fn up_visible(jungle: &[Vec<u32>], tree: u32, row: usize, col: usize) -> bool {
    (1..=row).all(|n| tree > jungle[row - n][col])
}

fn right_visible(jungle: &[Vec<u32>], tree: u32, row: usize, col: usize) -> bool {
    (1..=(98 - col)).all(|n| tree > jungle[row][col + n])
}

fn down_visible(jungle: &[Vec<u32>], tree: u32, row: usize, col: usize) -> bool {
    (1..=(98 - row)).all(|n| tree > jungle[row + n][col])
}

fn left_visible(jungle: &[Vec<u32>], tree: u32, row: usize, col: usize) -> bool {
    (1..=col).all(|n| tree > jungle[row][col - n])
}

fn calc_distance(jungle: &[Vec<u32>], row: usize, col: usize) -> usize {
    let tree = jungle[row][col];
    [
        match (1..=row)
            .take_while(|&n| tree > jungle[row - n][col])
            .count()
        {
            x if up_visible(jungle, tree, row, col) => x,
            x => x + 1,
        },
        match (1..=(98 - col))
            .take_while(|&n| tree > jungle[row][col + n])
            .count()
        {
            x if right_visible(jungle, tree, row, col) => x,
            x => x + 1,
        },
        match (1..=(98 - row))
            .take_while(|&n| tree > jungle[row + n][col])
            .count()
        {
            x if down_visible(jungle, tree, row, col) => x,
            x => x + 1,
        },
        match (1..=col)
            .take_while(|&n| tree > jungle[row][col - n])
            .count()
        {
            x if left_visible(jungle, tree, row, col) => x,
            x => x + 1,
        },
    ]
    .iter()
    .product()
    // let up = {
    //     let mut row = row;
    //     let mut counter = 0;
    //     while row > 0 {
    //         row -= 1;
    //         counter += 1;
    //         if tree <= j[row][col] {
    //             break;
    //         }
    //     }
    //     counter
    // };
    // let left = {
    //     let mut col = col;
    //     let mut counter = 0;
    //     while col > 0 {
    //         col -= 1;
    //         counter += 1;
    //         if tree <= j[row][col] {
    //             break;
    //         }
    //     }
    //     counter
    // };
    // let right = {
    //     let mut col = col;
    //     let mut counter = 0;
    //     while col < 98 {
    //         col += 1;
    //         counter += 1;
    //         if tree <= j[row][col] {
    //             break;
    //         }
    //     }
    //     counter
    // };
    // let down = {
    //     let mut row = row;
    //     let mut counter = 0;
    //     while row < 98 {
    //         row += 1;
    //         counter += 1;
    //         if tree <= j[row][col] {
    //             break;
    //         }
    //     }
    //     counter
    // };
    // up * right * left * down
}

// fn go_up(j: &[Vec<u32>], tree: u32, row: usize, col: usize) -> bool {
//     let mut row: usize = row;
//     while row > 0 {
//         row -= 1;
//         if tree <= j[row][col] {
//             return false;
//         }
//     }
// }

// fn go_left(j: &[Vec<u32>], tree: u32, row: usize, col: usize) -> bool {
//     let mut col = col;
//     while col > 0 {
//         col -= 1;
//         if tree <= j[row][col] {
//             return false;
//         }
//     }
//     true
// }

// fn go_right(j: &[Vec<u32>], tree: u32, row: usize, col: usize) -> bool {
//     let mut col = col;
//     while col < 98 {
//         col += 1;
//         if tree <= j[row][col] {
//             return false;
//         }
//     }
//     true
// }

// fn go_down(j: &[Vec<u32>], tree: u32, row: usize, col: usize) -> bool {
//     let mut row = row;
//     while row < 98 {
//         row += 1;
//         if tree <= j[row][col] {
//             return false;
//         }
//     }
//     true
// }
