use io::Write;
use std::io;

fn main() {
    let stdout = io::stdout();
    let mut out = io::BufWriter::new(stdout.lock());

    writeln!(out, "10101010 20202020 30303030 40404040 50505050 60606060 70707070 80808080 90909090 95959595 99999999").unwrap();
}
