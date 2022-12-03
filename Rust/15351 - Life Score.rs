use io::Write;
use std::io;

fn main() {
    let stdout = io::stdout();
    let mut out = io::BufWriter::new(stdout.lock());

    let mut n = String::new();
    io::stdin().read_line(&mut n).unwrap();
    let n = n.trim().parse::<i64>().unwrap();

    for _ in 0..n {
        let mut s = String::new();
        io::stdin().read_line(&mut s).unwrap();
        s = s.trim().to_string();

        let s = s.chars().collect::<Vec<_>>();
        let mut ret = 0;

        for c in s.iter() {
            if *c == ' ' {
                continue;
            }

            ret += *c as i64 - 'A' as i64 + 1;
        }

        if ret == 100 {
            writeln!(out, "PERFECT LIFE").unwrap();
        } else {
            writeln!(out, "{ret}").unwrap();
        }
    }
}
