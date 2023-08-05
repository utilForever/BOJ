use io::Write;
use std::io;

fn main() {
    let stdout = io::stdout();
    let mut out = io::BufWriter::new(stdout.lock());

    writeln!(out, "{} {}", 0, -1_000_000_000).unwrap();
    writeln!(out, "{} {}", 1_000_000_000, 1).unwrap();
    writeln!(out, "{} {}", -1_000_000_000, 0).unwrap();
    writeln!(out, "{} {}", -1, 1_000_000_000).unwrap();
}
