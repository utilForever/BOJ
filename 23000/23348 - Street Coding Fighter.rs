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

    let diff_a = nums[0];
    let diff_b = nums[1];
    let diff_c = nums[2];

    let n = input_integers()[0];

    let mut max_score = 0;

    for _ in 0..n {
        let mut total_score = 0;

        for _ in 0..3 {
            let nums = input_integers();

            let a = nums[0];
            let b = nums[1];
            let c = nums[2];

            let score = diff_a * a + diff_b * b + diff_c * c;
            total_score += score;
        }

        max_score = cmp::max(max_score, total_score);
    }

    println!("{}", max_score);
}
