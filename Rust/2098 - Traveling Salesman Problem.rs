use std::io;

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

fn process_tsp(
    matrix: &Vec<Vec<usize>>,
    cost: &mut Vec<Vec<usize>>,
    n: usize,
    cur: usize,
    visited: usize,
) -> usize {
    if cost[cur][visited] != 0 {
        return cost[cur][visited];
    }

    if visited == (1 << n) - 1 {
        if matrix[cur][0] != 0 {
            return matrix[cur][0];
        } else {
            return 1_000_000_000;
        }
    }

    let mut min_cost = 1_000_000_000;

    for i in 0..n {
        if visited & (1 << i) == 0 && matrix[cur][i] != 0 {
            let cost = process_tsp(matrix, cost, n, i, visited + (1 << i));
            min_cost = std::cmp::min(min_cost, cost + matrix[cur][i]);
        }
    }

    cost[cur][visited] = min_cost;
    min_cost
}

fn main() {
    let n = input_integers()[0] as usize;

    let mut matrix = vec![vec![0; n]; n];
    let mut cost = vec![vec![0; 1 << n]; n];

    for i in 0..n {
        let nums = input_integers();

        for j in 0..n {
            matrix[i][j] = nums[j] as usize;
        }
    }

    println!("{}", process_tsp(&matrix, &mut cost, n, 0, 1));
}
