use io::Write;
use std::io;

fn main() {
    let stdout = io::stdout();
    let mut out = io::BufWriter::new(stdout.lock());

    let mut s = String::new();
    io::stdin().read_line(&mut s).unwrap();
    let n = (s.chars().next().unwrap() as u8 - '0' as u8) as i32;

    for i in 1..=n {
        let mut s = String::new();
        io::stdin().read_line(&mut s).unwrap();

        write!(out, "{}. {}", i, s).unwrap();
    }
}
