use io::Write;
use std::io;

fn main() {
    let stdout = io::stdout();
    let mut out = io::BufWriter::new(stdout.lock());

    for a in 6..=100 {
        for b in 2..=100 {
            for c in b + 1..=100 {
                for d in c + 1..=100 {
                    if a * a * a == b * b * b + c * c * c + d * d * d {
                        writeln!(out, "Cube = {}, Triple = ({},{},{})", a, b, c, d).unwrap();
                    }
                }
            }
        }
    }
}
