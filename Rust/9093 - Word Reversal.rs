use io::Write;
use std::{io, str};

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

    let n = input_integers()[0];

    for _ in 0..n {
        let mut s = String::new();
        io::stdin().read_line(&mut s).unwrap();
        s = s.trim().to_string();

        let s = s.split(' ').collect::<Vec<&str>>();

        for word in s.iter() {
            write!(out, "{} ", word.chars().rev().collect::<String>()).unwrap();
        }

        writeln!(out).unwrap();
    }
}
