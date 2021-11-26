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

fn main() {
    let n = input_integers()[0];

    let nums = input_integers();
    let mut sum_score = 0;
    let mut max_score = 0;

    for i in 0..n as usize {
        sum_score += nums[i];

        if nums[i] > max_score {
            max_score = nums[i];
        }
    }

    println!(
        "{}",
        (sum_score as f64 / max_score as f64 * 100.0) / n as f64
    );
}
