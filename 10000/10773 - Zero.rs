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

fn main() {
    let n = input_integers()[0];

    let mut nums = VecDeque::new();

    for _ in 0..n {
        let num = input_integers()[0];

        if num == 0 {
            nums.pop_back();
        } else {
            nums.push_back(num);
        }
    }

    let mut sum = 0;

    while !nums.is_empty() {
        sum += nums.back().unwrap();
        nums.pop_back();
    }

    println!("{}", sum);
}
