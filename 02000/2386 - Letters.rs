use io::Write;
use std::io;

fn main() {
    let stdout = io::stdout();
    let mut out = io::BufWriter::new(stdout.lock());

    loop {
        let mut s = String::new();
        io::stdin().read_line(&mut s).unwrap();
        s = s.trim().to_string();

        if s == "#" {
            break;
        }

        let s = s.split_whitespace().collect::<Vec<_>>();
        let alphabet = s[0].chars().nth(0).unwrap();
        let mut ret = 0;

        for i in 1..s.len() {
            for c in s[i].chars() {
                if c.to_lowercase().next().unwrap() == alphabet {
                    ret += 1;
                }
            }
        }

        writeln!(out, "{alphabet} {ret}").unwrap();
    }
}
