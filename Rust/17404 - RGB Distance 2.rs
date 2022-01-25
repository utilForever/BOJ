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

fn calculate_cost(min_costs: &mut Vec<Vec<i64>>, houses: &mut Vec<Vec<i64>>, n: usize) {
    for i in 2..n {
        min_costs[i][0] = cmp::min(min_costs[i - 1][1], min_costs[i - 1][2]) + houses[i][0];
        min_costs[i][1] = cmp::min(min_costs[i - 1][0], min_costs[i - 1][2]) + houses[i][1];
        min_costs[i][2] = cmp::min(min_costs[i - 1][0], min_costs[i - 1][1]) + houses[i][2];
    }
}

fn main() {
    let n = input_integers()[0] as usize;

    let mut houses = vec![vec![0; 3]; n];
    let mut min_costs = vec![vec![0; 3]; n];
    let mut ans = i64::MAX;

    for i in 0..n {
        let nums = input_integers();
        houses[i][0] = nums[0];
        houses[i][1] = nums[1];
        houses[i][2] = nums[2];
    }

    // Red start
    min_costs[1][0] = i64::MAX;
    min_costs[1][1] = houses[0][0] + houses[1][1];
    min_costs[1][2] = houses[0][0] + houses[1][2];

    calculate_cost(&mut min_costs, &mut houses, n);
    ans = cmp::min(ans, cmp::min(min_costs[n - 1][1], min_costs[n - 1][2]));

    // Green start
    min_costs[1][0] = houses[0][1] + houses[1][0];
    min_costs[1][1] = i64::MAX;
    min_costs[1][2] = houses[0][1] + houses[1][2];

    calculate_cost(&mut min_costs, &mut houses, n);
    ans = cmp::min(ans, cmp::min(min_costs[n - 1][0], min_costs[n - 1][2]));

    // Blue start
    min_costs[1][0] = houses[0][2] + houses[1][0];
    min_costs[1][1] = houses[0][2] + houses[1][1];
    min_costs[1][2] = i64::MAX;

    calculate_cost(&mut min_costs, &mut houses, n);
    ans = cmp::min(ans, cmp::min(min_costs[n - 1][0], min_costs[n - 1][1]));

    println!("{}", ans);
}
