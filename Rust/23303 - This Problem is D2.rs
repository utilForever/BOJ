use io::Write;
use std::io;

fn main() {
    let stdout = io::stdout();
    let mut out = io::BufWriter::new(stdout.lock());

    let mut s = String::new();
    io::stdin().read_line(&mut s).unwrap();

    writeln!(
        out,
        "{}",
        if s.find("d2").is_some() || s.find("D2").is_some() {
            "D2"
        } else {
            "unrated"
        }
    )
    .unwrap();
}
