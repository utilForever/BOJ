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

    let buttons = input_integers();
    let mut ret = 5000;

    for &button in buttons.iter() {
        match button {
            1 => ret -= 500,
            2 => ret -= 800,
            3 => ret -= 1000,
            _ => (),
        }
    }

    writeln!(out, "{ret}").unwrap();
}
