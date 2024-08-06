use io::Write;
use std::io;

fn main() {
    let stdout = io::stdout();
    let mut out = io::BufWriter::new(stdout.lock());
    let mut ret = 0;

    loop {
        let mut s = String::new();
        io::stdin().read_line(&mut s).unwrap();
        s = s.trim().to_string();

        if s.is_empty() {
            break;
        }

        let words = s.split_whitespace().collect::<Vec<_>>();
        let step = words[1].parse::<i64>().unwrap();

        ret += if words[0] == "Es" {
            21 * step
        } else {
            17 * step
        };
    }

    writeln!(out, "{ret}").unwrap();
}
