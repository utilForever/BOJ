use io::Write;
use std::io;

fn main() {
    let stdout = io::stdout();
    let mut out = io::BufWriter::new(stdout.lock());

    let mut s = String::new();
    io::stdin().read_line(&mut s).unwrap();

    let mut ret = String::new();
    let mut prev = '_';

    for s in s.chars() {
        if s != prev {
            ret.push(s);
            prev = s;
        }
    }

    write!(out, "{ret}").unwrap();
}
