use io::Write;
use std::io;

fn main() {
    let stdout = io::stdout();
    let mut out = io::BufWriter::new(stdout.lock());

    let mut s = String::new();
    io::stdin().read_line(&mut s).unwrap();
    s = s.trim().to_string();

    let n = s.parse::<i64>().unwrap();

    for _ in 0..n {
        s.clear();
        io::stdin().read_line(&mut s).unwrap();
        s = s.trim().to_string();

        if s.starts_with("Simon says") {
            writeln!(out, "{}", &s[10..]).unwrap();
        }
    }
}
