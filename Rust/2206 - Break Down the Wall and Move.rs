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

fn explore(maze: &Vec<Vec<char>>, visited: &mut Vec<Vec<Vec<i32>>>, n: i32, m: i32) -> i32 {
    let mut queue = VecDeque::new();
    queue.push_back((0, 0, 0));
    visited[0][0][0] = 1;

    while !queue.is_empty() {
        let (cur_x, cur_y, num_break) = queue.pop_front().unwrap();

        if cur_y == n - 1 && cur_x == m - 1 {
            return visited[cur_y as usize][cur_x as usize][num_break];
        }

        for i in 0..4 {
            let next_x = cur_x
                + if i == 0 {
                    -1
                } else if i == 1 {
                    1
                } else {
                    0
                };
            let next_y = cur_y
                + if i == 2 {
                    -1
                } else if i == 3 {
                    1
                } else {
                    0
                };

            if next_x < 0 || next_x >= m || next_y < 0 || next_y >= n {
                continue;
            }

            if visited[next_y as usize][next_x as usize][num_break] == 0 {
                if maze[next_y as usize][next_x as usize] == '0' {
                    visited[next_y as usize][next_x as usize][num_break] =
                        visited[cur_y as usize][cur_x as usize][num_break] + 1;
                    queue.push_back((next_x, next_y, num_break));
                } else if maze[next_y as usize][next_x as usize] == '1' && num_break == 0 {
                    visited[next_y as usize][next_x as usize][num_break + 1] =
                        visited[cur_y as usize][cur_x as usize][num_break] + 1;
                    queue.push_back((next_x, next_y, num_break + 1));
                }
            }
        }
    }

    -1
}

fn main() {
    let nums = input_integers();
    let (n, m) = (nums[0] as usize, nums[1] as usize);

    let mut maze = vec![vec![' '; m]; n];
    let mut visited = vec![vec![vec![0; 2]; m]; n];

    for i in 0..n {
        let mut s = String::new();
        io::stdin().read_line(&mut s).unwrap();
        s = s.trim().to_string();
        let mut chars = s.chars();

        for j in 0..m {
            maze[i][j] = chars.next().unwrap();
        }
    }

    println!("{}", explore(&maze, &mut visited, n as i32, m as i32));
}
