use std::{collections::HashMap, io};

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

fn sum(tree: &Vec<i64>, x: usize) -> i64 {
    let mut sum = 0;
    let mut idx = x as i64;

    while idx > 0 {
        sum += tree[idx as usize];
        idx -= idx & -idx;
    }

    sum
}

fn sum_section(tree: &Vec<i64>, x: usize, y: usize) -> i64 {
    sum(tree, y) - sum(tree, x - 1)
}

fn update(tree: &mut Vec<i64>, x: usize, diff: i64) {
    let mut idx = x as i64;

    while (idx as usize) < tree.len() {
        tree[idx as usize] += diff;
        idx += idx & -idx;
    }
}

fn main() {
    let n = input_integers()[0] as usize;

    let machines = input_integers();
    let mut position = HashMap::new();

    let nums = input_integers();
    for i in 0..nums.len() {
        position.insert(nums[i], i + 1);
    }

    let mut num_cable = 0;
    let mut tree = vec![0; n + 1];

    for i in 0..n {
        if position[&machines[i]] + 1 <= n {
            num_cable += sum_section(&mut tree, position[&machines[i]] + 1, n);
        }

        update(&mut tree, position[&machines[i]], 1);
    }

    println!("{}", num_cable);
}
