use std::{cmp, io};

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

fn main() {
    let mut gems = vec![vec![0.0; 501]; 501];
    let mut num_cases = vec![vec![0.0; 501]; 501];
    let mut total_values = vec![vec![0.0; 501]; 501];

    let nums = input_integers();

    let n = nums[0];
    let d = nums[1];
    let r = nums[2];

    for i in 0..=cmp::max(n, d) as usize {
        gems[i as usize][0] = 1.0;

        for j in 1..=i as usize {
            gems[i][j] = gems[i - 1][j - 1] + gems[i - 1][j];
        }
    }

    num_cases[0][0] = 1.0;

    for i in 1..=n as usize {
        for j in 0..=d as usize {
            for k in 0..=cmp::min(i, j) {
                num_cases[i][j] += gems[i][k] * num_cases[k][j - k];
                total_values[i][j] += gems[i][k]
                    * (total_values[k][j - k]
                        + cmp::min(k, r as usize) as f64 * num_cases[k][j - k]);
            }
        }
    }

    println!(
        "{}",
        total_values[n as usize][d as usize] / num_cases[n as usize][d as usize] + r as f64
    );
}
