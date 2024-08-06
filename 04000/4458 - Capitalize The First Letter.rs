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

    let n = input_integers()[0];

    for _ in 0..n {
        let mut s = String::new();
        io::stdin().read_line(&mut s).unwrap();

        let first_letter = s.remove(0).to_uppercase().to_string();
        s.insert(0, first_letter.as_str().chars().next().unwrap());

        write!(out, "{}", s).unwrap();
    }
}
