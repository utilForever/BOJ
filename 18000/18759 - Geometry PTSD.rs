use io::Write;
use std::io;

fn main() {
    let stdout = io::stdout();
    let mut out = io::BufWriter::new(stdout.lock());

    writeln!(out, "999998 999998 1").unwrap();
    writeln!(out, "1 -999999 1000000").unwrap();
    writeln!(out, "-999999 0 -1000000").unwrap();
}
