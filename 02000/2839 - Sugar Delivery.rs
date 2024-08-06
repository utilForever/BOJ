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

    let mut bag = vec![1_000_000_000; 5001];
    bag[3] = 1;
    bag[5] = 1;
    bag[6] = 2;

    for i in 8..=n {
        bag[i] = cmp::min(bag[i - 3] + 1, bag[i - 5] + 1);
    }

    println!("{}", if bag[n] != 1_000_000_000 { bag[n] } else { -1 });
}
