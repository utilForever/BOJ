use io::Write;
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
    let stdout = io::stdout();
    let mut out = io::BufWriter::new(stdout.lock());

    let n = input_integers()[0] as usize;

    let mut nums = vec![0; 10001];

    for _ in 0..n {
        nums[input_integers()[0] as usize] += 1;
    }

    for i in 1..=10000 as usize {
        for _ in 0..nums[i] {
            writeln!(out, "{}", i).unwrap();
        }
    }
}
