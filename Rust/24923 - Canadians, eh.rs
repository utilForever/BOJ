use io::Write;
use std::io;

fn main() {
    let stdout = io::stdout();
    let mut out = io::BufWriter::new(stdout.lock());

    let mut s = String::new();
    io::stdin().read_line(&mut s).unwrap();
    s = s.trim().to_string();

    writeln!(
        out,
        "{}",
        if &s[s.len() - 3..s.len()] == "eh?" {
            "Canadian!"
        } else {
            "Imposter!"
        }
    )
    .unwrap();
}
