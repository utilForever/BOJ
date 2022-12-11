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

fn gcd(first: i64, second: i64) -> i64 {
    let mut max = first;
    let mut min = second;

    if min == 0 && max == 0 {
        return 0;
    } else if min == 0 {
        return max;
    } else if max == 0 {
        return min;
    }

    if min > max {
        let val = max;

        max = min;
        min = val;
    }

    loop {
        let res = max % min;

        if res == 0 {
            return min;
        }

        max = min;
        min = res;
    }
}

fn main() {
    let stdout = io::stdout();
    let mut out = io::BufWriter::new(stdout.lock());

    loop {
        let nums = input_integers();

        if nums.is_empty() {
            break;
        }

        let n = nums[0];
        let mut a = 0;
        let mut b = 1;

        for i in 1..=n {
            b = b * i / gcd(b, i);
        }

        for i in 1..=n {
            a += b / i;
        }

        a *= n;

        if a % b == 0 {
            writeln!(out, "{}", a / b).unwrap();
        } else {
            let q = a / b;
            let mut r = a - q * b;

            let val_gcd = gcd(r, b);
            r = r / val_gcd;
            b = b / val_gcd;

            for _ in 0..=q.to_string().len() {
                write!(out, " ").unwrap();
            }
            writeln!(out, "{r}").unwrap();

            write!(out, "{q} ").unwrap();
            for _ in 0..b.to_string().len() {
                write!(out, "-").unwrap();
            }
            writeln!(out).unwrap();

            for _ in 0..=q.to_string().len() {
                write!(out, " ").unwrap();
            }
            writeln!(out, "{b}").unwrap();
        }
    }
}
