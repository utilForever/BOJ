use io::Write;
use std::io;

fn main() {
    let stdout = io::stdout();
    let mut out = io::BufWriter::new(stdout.lock());

    let lowercase = "abcdefghijklmnopqrstuvwxyz".chars().collect::<Vec<_>>();
    let uppercase = "ABCDEFGHIJKLMNOPQRSTUVWXYZ".chars().collect::<Vec<_>>();

    let mut s = String::new();
    io::stdin().read_line(&mut s).unwrap();

    for c in s.chars() {
        if c.is_lowercase() {
            let i = lowercase.iter().position(|&x| x == c).unwrap();
            write!(out, "{}", lowercase[(i + 13) % 26]).unwrap();
        } else if c.is_uppercase() {
            let i = uppercase.iter().position(|&x| x == c).unwrap();
            write!(out, "{}", uppercase[(i + 13) % 26]).unwrap();
        } else {
            write!(out, "{c}").unwrap();
        }
    }
}
