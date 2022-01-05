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

    let mut nums = vec![0; n];

    for i in 0..n {
        nums[i] = input_integers()[0];
    }

    nums.sort();

    for num in nums {
        writeln!(out, "{}", num).unwrap();
    }
}
