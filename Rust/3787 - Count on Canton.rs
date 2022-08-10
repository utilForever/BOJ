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

        let mut term = nums[0];
        let mut idx = 1;

        while term > idx {
            term -= idx;
            idx += 1;
        }

        if idx % 2 == 0 {
            writeln!(out, "TERM {} IS {}/{}", nums[0], term, idx - term + 1).unwrap();
        } else {
            writeln!(out, "TERM {} IS {}/{}", nums[0], idx - term + 1, term).unwrap();
        }
    }
}
