use io::Write;
use std::io;

fn main() {
    let stdout = io::stdout();
    let mut out = io::BufWriter::new(stdout.lock());

    let buffer = io::read_to_string(io::stdin()).unwrap();

    for s in buffer.lines() {
        writeln!(
            out,
            "{}",
            if s.to_lowercase().contains("problem") {
                "yes"
            } else {
                "no"
            }
        )
        .unwrap();
    }
}
