use io::Write;
use std::io;

fn main() {
    let stdout = io::stdout();
    let mut out = io::BufWriter::new(stdout.lock());

    loop {
        let mut s = String::new();
        io::stdin().read_line(&mut s).unwrap();
        s = s.trim().to_string();

        if s.is_empty() {
            break;
        }

        let mut iter = s.split("+");
        let a = iter.next().unwrap();
        let mut iter = iter.next().unwrap().split("=");
        let b = iter.next().unwrap();
        let c = iter.next().unwrap();

        let a = a.chars().rev().collect::<String>().parse::<i64>().unwrap();
        let b = b.chars().rev().collect::<String>().parse::<i64>().unwrap();
        let c = c.chars().rev().collect::<String>().parse::<i64>().unwrap();

        writeln!(out, "{}", if a + b == c { "True" } else { "False" }).unwrap();
    }
}
