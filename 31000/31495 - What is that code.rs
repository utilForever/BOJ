use io::Write;
use std::io;

fn main() {
    let stdout = io::stdout();
    let mut out = io::BufWriter::new(stdout.lock());

    let mut s = String::new();
    io::stdin().read_line(&mut s).unwrap();
    s = s.trim().to_string();

    if s == "\"" || s == "\"\"" {
        writeln!(out, "CE").unwrap();
        return;
    }

    writeln!(
        out,
        "{}",
        if s.starts_with("\"") && s.ends_with("\"") {
            s[1..s.len() - 1].to_string()
        } else {
            "CE".to_string()
        }
    )
    .unwrap();
}
