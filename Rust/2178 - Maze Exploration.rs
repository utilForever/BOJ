use std::{collections::VecDeque, io};

fn input_integers() -> Vec<i32> {
    let mut s = String::new();

    io::stdin().read_line(&mut s).unwrap();

    let values: Vec<i32> = s
        .as_mut_str()
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    values
}

fn explore(maze: &Vec<Vec<char>>, n: usize, m: usize) -> usize {
    let mut queue = VecDeque::new();
    let mut visited = vec![vec![0; m]; n];

    queue.push_back((0, 0));
    visited[0][0] = 1;

    while !queue.is_empty() {
        let (x, y) = queue.pop_front().unwrap();

        if y == n - 1 && x == m - 1 {
            break;
        }

        for i in 0..4 {
            let (dx, dy) = match i {
                0 => (0, -1),
                1 => (1, 0),
                2 => (0, 1),
                3 => (-1, 0),
                _ => (0, 0),
            };

            let (next_x, next_y) = (x as i32 + dx, y as i32 + dy);

            if next_x < 0 || next_x >= m as i32 || next_y < 0 || next_y >= n as i32 {
                continue;
            }

            let next_x = next_x as usize;
            let next_y = next_y as usize;

            if visited[next_y][next_x] == 0 && maze[next_y][next_x] == '1' {
                visited[next_y][next_x] = visited[y][x] + 1;
                queue.push_back((next_x, next_y));
            }
        }
    }

    visited[n - 1][m - 1]
}

fn main() {
    let nums = input_integers();
    let (n, m) = (nums[0] as usize, nums[1] as usize);
    let mut maze = vec![vec!['0'; m]; n];

    for i in 0..n {
        let mut s = String::new();
        io::stdin().read_line(&mut s).unwrap();
        s = s.trim().to_string();
        let mut chars = s.chars();

        for j in 0..m {
            maze[i][j] = chars.next().unwrap();
        }
    }

    println!("{}", explore(&maze, n, m));
}
