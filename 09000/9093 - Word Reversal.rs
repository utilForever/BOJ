use io::Write;
use std::{io, str};

fn main() {
    let stdout = io::stdout();
    let mut out = io::BufWriter::new(stdout.lock());

    let mut s = String::new();
    io::stdin().read_line(&mut s).unwrap();
    s = s.trim().to_string();
    
    let n = s.parse::<i64>().unwrap();

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
