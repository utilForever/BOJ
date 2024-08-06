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
    let apple1 = nums[0];
    let orange1 = nums[1];

    let nums = input_integers();
    let apple2 = nums[0];
    let orange2 = nums[1];

    println!("{}", cmp::min(apple1 + orange2, apple2 + orange1));
}
