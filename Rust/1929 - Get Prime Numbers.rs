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

    let nums = input_integers();
    let n = nums[0] as usize;
    let m = nums[1] as usize;

    let mut prime_number = vec![0; m + 1];

    for i in 2..=m {
        prime_number[i] = i;
    }

    for i in 2..=(m as f64).sqrt() as usize {
        if prime_number[i] == 0 {
            continue;
        }

        for j in (i * i..=m).step_by(i) {
            prime_number[j] = 0;
        }
    }

    for i in n..=m {
        if prime_number[i] != 0 {
            writeln!(out, "{}", prime_number[i]).unwrap();
        }
    }
}
