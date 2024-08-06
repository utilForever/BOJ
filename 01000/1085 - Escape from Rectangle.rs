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
    let (x, y, w, h) = (nums[0], nums[1], nums[2], nums[3]);

    println!("{}", vec![x, y, w - x, h - y].iter().min().unwrap());
}
