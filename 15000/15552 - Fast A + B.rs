use io::Write;
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
    let stdout = io::stdout();
    let mut out = io::BufWriter::new(stdout.lock());

    let t = input_integers()[0];

    for _ in 0..t {
        let nums = input_integers();

        writeln!(out, "{}", nums[0] + nums[1]).unwrap();
    }
}
