use std::io;

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
    let nums = input_integers();
    let (a, b, v) = (nums[0], nums[1], nums[2]);

    println!("{}", ((v - b) as f64 / (a - b) as f64).ceil() as i64);
}
