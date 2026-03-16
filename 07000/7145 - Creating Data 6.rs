use io::Write;
use std::io;

fn main() {
    let stdout = io::stdout();
    let mut out = io::BufWriter::new(stdout.lock());

    writeln!(out, "33").unwrap();

    for i in 0..=15 {
        let a = 2 * i;
        let (b, c) = (a + 1, a + 2);
        let w = 1i64 << (15 - i);

        writeln!(out, "2 {c} 0 {b} {w}").unwrap();
        writeln!(out, "1 {c} {}", -2 * w).unwrap();
    }

    writeln!(out, "0").unwrap();

    writeln!(out, "6").unwrap();
    writeln!(out, "0 32").unwrap();
    writeln!(out, "0 32").unwrap();
    writeln!(out, "0 32").unwrap();
    writeln!(out, "0 32").unwrap();
    writeln!(out, "0 32").unwrap();
    writeln!(out, "0 32").unwrap();
}
