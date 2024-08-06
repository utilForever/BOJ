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

    loop {
        let nums = input_integers();

        if nums.is_empty() {
            break;
        }

        writeln!(out, "{}", (nums[1] - nums[0]).max(nums[2] - nums[1]) - 1).unwrap();
    }
}
