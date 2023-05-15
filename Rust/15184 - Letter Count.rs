use io::Write;
use std::io;

fn main() {
    let stdout = io::stdout();
    let mut out = io::BufWriter::new(stdout.lock());

    let mut s = String::new();
    io::stdin().read_line(&mut s).unwrap();
    s = s.trim().to_string().to_ascii_uppercase();

    let mut ret = [0; 26];

    for c in s.chars() {
        if c.is_alphabetic() {
            ret[c as usize - 'A' as usize] += 1;
        }
    }

    for i in 0..26 {
        write!(out, "{} |", (i + 'A' as usize) as u8 as char).unwrap();

        if ret[i] > 0 {
            write!(out, " ").unwrap();
        }

        for _ in 0..ret[i] {
            write!(out, "*").unwrap();
        }

        writeln!(out).unwrap();
    }
}
