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

fn process_bfs(tomatoes: &mut Vec<Vec<Vec<i64>>>, m: usize, n: usize, h: usize) {
    let mut queue = VecDeque::new();

    for i in 0..h {
        for j in 0..n {
            for k in 0..m {
                if tomatoes[i][j][k] == 1 {
                    queue.push_back((k as i64, j as i64, i as i64));
                }
            }
        }
    }

    while !queue.is_empty() {
        let (cur_x, cur_y, cur_z) = queue.pop_front().unwrap();

        for i in 0..6 {
            let next_x = cur_x
                + if i == 2 {
                    -1
                } else if i == 3 {
                    1
                } else {
                    0
                };
            let next_y = cur_y
                + if i == 4 {
                    -1
                } else if i == 5 {
                    1
                } else {
                    0
                };
            let next_z = cur_z
                + if i == 0 {
                    -1
                } else if i == 1 {
                    1
                } else {
                    0
                };

            if next_x < 0
                || next_x >= m as i64
                || next_y < 0
                || next_y >= n as i64
                || next_z < 0
                || next_z >= h as i64
            {
                continue;
            }

            if tomatoes[next_z as usize][next_y as usize][next_x as usize] == -1 {
                continue;
            }

            if tomatoes[next_z as usize][next_y as usize][next_x as usize] == 0 {
                tomatoes[next_z as usize][next_y as usize][next_x as usize] =
                    tomatoes[cur_z as usize][cur_y as usize][cur_x as usize] + 1;
                queue.push_back((next_x, next_y, next_z));
            }
        }
    }
}

fn main() {
    let nums = input_integers();
    let (m, n, h) = (nums[0] as usize, nums[1] as usize, nums[2] as usize);

    let mut tomatoes = vec![vec![vec![0; m]; n]; h];

    for i in 0..h {
        for j in 0..n {
            let nums = input_integers();

            for k in 0..m {
                tomatoes[i][j][k] = nums[k];
            }
        }
    }

    process_bfs(&mut tomatoes, m, n, h);

    let mut max = 0;

    for i in 0..h {
        for j in 0..n {
            for k in 0..m {
                if tomatoes[i][j][k] == 0 {
                    println!("-1");
                    return;
                }

                if tomatoes[i][j][k] > max {
                    max = tomatoes[i][j][k];
                }
            }
        }
    }

    println!("{}", max - 1);
}
