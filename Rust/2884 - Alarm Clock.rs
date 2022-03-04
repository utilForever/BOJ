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
    let (mut h, m) = (nums[0], nums[1]);

    h = if h == 0 { 24 } else { h };
    let total_min = (h * 60 + m) - 45;

    let mut new_h = total_min / 60;
    new_h = if new_h == 24 { 0 } else { new_h };
    let new_m = total_min % 60;

    println!("{} {}", new_h, new_m);
}
