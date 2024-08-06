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
    let (n, mut k) = (nums[0], nums[1]);

    let mut values = vec![0; n as usize];

    for i in 0..n as usize {
        values[i] = input_integers()[0];
    }

    let mut count = 0;

    for value in values.iter().rev() {
        count += k / value;
        k %= value;
    }

    println!("{}", count);
}
