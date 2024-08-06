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

        for i in 0..nums.len() {
            write!(
                out,
                "{} ",
                if i == 0 {
                    nums[i] * nums[i + 1]
                } else if i == nums.len() - 1 {
                    nums[i] * nums[i - 1]
                } else {
                    nums[i - 1] * nums[i] * nums[i + 1]
                }
            )
            .unwrap();
        }

        writeln!(out).unwrap();
    }
}
