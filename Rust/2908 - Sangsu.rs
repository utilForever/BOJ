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
    let nums = input_integers();

    let a = nums[0];
    let b = nums[1];

    let new_a = (a / 100) + ((a % 100) / 10) * 10 + (a % 10) * 100;
    let new_b = (b / 100) + ((b % 100) / 10) * 10 + (b % 10) * 100;

    println!("{}", cmp::max(new_a, new_b));
}
