use std::{cmp, io};

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

fn main() {
    let n = input_integers()[0] as usize;

    let mut houses = vec![vec![0; 3]; n];
    let mut min_costs = vec![vec![0; 3]; n];

    for i in 0..n {
        let nums = input_integers();
        houses[i][0] = nums[0];
        houses[i][1] = nums[1];
        houses[i][2] = nums[2];
    }

    min_costs[0][0] = houses[0][0];
    min_costs[0][1] = houses[0][1];
    min_costs[0][2] = houses[0][2];

    for i in 1..n {
        min_costs[i][0] = cmp::min(min_costs[i - 1][1], min_costs[i - 1][2]) + houses[i][0];
        min_costs[i][1] = cmp::min(min_costs[i - 1][0], min_costs[i - 1][2]) + houses[i][1];
        min_costs[i][2] = cmp::min(min_costs[i - 1][0], min_costs[i - 1][1]) + houses[i][2];
    }

    println!(
        "{}",
        vec![
            min_costs[n - 1][0],
            min_costs[n - 1][1],
            min_costs[n - 1][2]
        ]
        .iter()
        .min()
        .unwrap()
    );
}
