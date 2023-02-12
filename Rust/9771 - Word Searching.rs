use io::Write;
use std::io;

fn main() {
    let stdout = io::stdout();
    let mut out = io::BufWriter::new(stdout.lock());

    let strings = io::read_to_string(io::stdin()).unwrap();
    let word = strings.lines().next().unwrap();

    writeln!(out, "{}", strings.matches(word).count() - 1).unwrap();
}
