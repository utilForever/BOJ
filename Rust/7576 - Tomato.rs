use std::{collections::VecDeque, io};

fn input_integers() -> Vec<i64> {
    let mut s = String::new();

    io::stdin().read_line(&mut s).unwrap();

    let values: Vec<i64> = s
        .as_mut_str()
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    values
}

fn process_bfs(tomatoes: &mut Vec<Vec<i64>>, m: usize, n: usize) {
    let mut queue = VecDeque::new();

    for i in 0..n {
        for j in 0..m {
            if tomatoes[i][j] == 1 {
                queue.push_back((j as i64, i as i64));
            }
        }
    }

    while !queue.is_empty() {
        let (cur_x, cur_y) = queue.pop_front().unwrap();

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

            if next_x < 0 || next_x >= m as i64 || next_y < 0 || next_y >= n as i64 {
                continue;
            }

            if tomatoes[next_y as usize][next_x as usize] == -1 {
                continue;
            }

            if tomatoes[next_y as usize][next_x as usize] == 0 {
                tomatoes[next_y as usize][next_x as usize] =
                    tomatoes[cur_y as usize][cur_x as usize] + 1;
                queue.push_back((next_x, next_y));
            }
        }
    }
}

fn main() {
    let nums = input_integers();
    let (m, n) = (nums[0] as usize, nums[1] as usize);

    let mut tomatoes = vec![vec![0; m]; n];

    for i in 0..n {
        let nums = input_integers();

        for j in 0..m {
            tomatoes[i][j] = nums[j];
        }
    }

    process_bfs(&mut tomatoes, m, n);

    let mut max = 0;

    for i in 0..n {
        for j in 0..m {
            if tomatoes[i][j] == 0 {
                println!("-1");
                return;
            }

            if tomatoes[i][j] > max {
                max = tomatoes[i][j];
            }
        }
    }

    println!("{}", max - 1);
}
