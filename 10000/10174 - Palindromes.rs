use io::Write;
use std::io;

fn main() {
    let stdout = io::stdout();
    let mut out = io::BufWriter::new(stdout.lock());

    let mut s = String::new();
    io::stdin().read_line(&mut s).unwrap();
    let n = s.trim().parse::<i64>().unwrap();

    for _ in 0..n {
        let mut s = String::new();
        io::stdin().read_line(&mut s).unwrap();
        s = s.trim().to_string();
        let s = s.chars().collect::<Vec<_>>();
        let mut ret = true;

        for i in 0..s.len() / 2 {
            if s[i].to_lowercase().to_string() != s[s.len() - i - 1].to_lowercase().to_string() {
                ret = false;
                break;
            }
        }

        writeln!(out, "{}", if ret { "Yes" } else { "No" }).unwrap();
    }
}
