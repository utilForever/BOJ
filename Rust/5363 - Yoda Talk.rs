use io::Write;
use std::io;

fn main() {
    let stdout = io::stdout();
    let mut out = io::BufWriter::new(stdout.lock());

    let mut s = String::new();
    io::stdin().read_line(&mut s).unwrap();
    let n = s.trim().parse::<i64>().unwrap();

    for _ in 0..n {
        let mut s = String::new();
        io::stdin().read_line(&mut s).unwrap();
        s = s.trim().to_string();

        let s = s.split_whitespace().collect::<Vec<_>>();

        for i in 2..s.len() {
            write!(out, "{} ", s[i]).unwrap();
        }

        writeln!(out, "{} {}", s[0], s[1]).unwrap();
    }
}
