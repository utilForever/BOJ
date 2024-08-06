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
    let num_stairs = input_integers()[0];

    let mut stairs = vec![0; num_stairs as usize + 1];

    for i in 1..=num_stairs as usize {
        stairs[i] = input_integers()[0];
    }

    let mut scores = vec![vec![0; 3]; num_stairs as usize + 1];

    scores[1][1] = stairs[1];

    for i in 2..=num_stairs as usize {
        scores[i][1] = cmp::max(scores[i - 2][1], scores[i - 2][2]) + stairs[i];
        scores[i][2] = scores[i - 1][1] + stairs[i];
    }

    println!(
        "{}",
        cmp::max(
            scores[num_stairs as usize][1],
            scores[num_stairs as usize][2]
        )
    );
}
