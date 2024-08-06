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
    let nums = input_integers();

    let mut ans = 0;

    for i in 1..(n - 1) {
        ans = cmp::max(ans, nums[i] + cmp::min(nums[i - 1], nums[i + 1]));
    }

    ans = vec![nums[0], nums[n - 1], ans].into_iter().max().unwrap();

    println!("{}", ans);
}
