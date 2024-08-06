use io::Write;
use std::io;

fn main() {
    let stdout = io::stdout();
    let mut out = io::BufWriter::new(stdout.lock());

    for i in 100..1000 {
        for j in 100..1000 {
            if i % 111 == 0 && j % 111 == 0 {
                continue;
            }

            if ((i / 10) * j == i * (j % 100)) && (i % 10 == j / 100) {
                writeln!(out, "{} / {} = {} / {}", i, j, i / 10, j % 100).unwrap();
            }
        }
    }
}
