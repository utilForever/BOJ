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
    let num_days = [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
    let res = ["SUN", "MON", "TUE", "WED", "THU", "FRI", "SAT"];

    let nums = input_integers();

    let x = nums[0];
    let y = nums[1];

    let mut spent_days = 0;

    for i in 0..(x - 1) as usize {
        spent_days += num_days[i];
    }
    spent_days += y;

    println!("{}", res[(spent_days % 7) as usize]);
}
