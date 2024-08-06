use io::Write;
use std::io::{self, BufWriter, StdoutLock};

fn input_integers() -> Vec<usize> {
    let mut s = String::new();

    io::stdin().read_line(&mut s).unwrap();

    let values: Vec<usize> = s
        .as_mut_str()
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    values
}

fn check(out: &mut BufWriter<StdoutLock>, arr: &mut [i32; 9], num: usize, n: usize, m: usize) {
    if num == m + 1 {
        for i in 1..=m {
            write!(out, "{} ", arr[i]).unwrap();
        }

        writeln!(out).unwrap();
    } else {
        for i in 1..=n {
            arr[num] = i as i32;
            check(out, arr, num + 1, n, m);
            arr[num] = 0;
        }
    }
}

fn main() {
    let stdout = io::stdout();
    let mut out = io::BufWriter::new(stdout.lock());

    let nums = input_integers();
    let (n, m) = (nums[0], nums[1]);

    let mut arr = [0; 9];

    check(&mut out, &mut arr, 1, n, m);
}
