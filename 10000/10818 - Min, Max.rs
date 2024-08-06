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

    let mut min = i32::MAX;
    let mut max = i32::MIN;

    let nums = input_integers();

    for i in 0..n as usize {
        if nums[i] < min {
            min = nums[i];
        }

        if nums[i] > max {
            max = nums[i];
        }
    }

    println!("{} {}", min, max);
}
