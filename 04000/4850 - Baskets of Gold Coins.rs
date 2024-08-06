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
        let values = input_integers();

        if values.is_empty() {
            break;
        }

        let (n, w, d, sum) = (values[0], values[1], values[2], values[3]);
        let total = (w * n * (n - 1)) / 2;
        let idx = (total - sum) / d;

        writeln!(out, "{}", if idx == 0 { n } else { idx }).unwrap();
    }
}
