use io::Write;
use std::io;

fn main() {
    let stdout = io::stdout();
    let mut out = io::BufWriter::new(stdout.lock());

    loop {
        let mut s = String::new();
        io::stdin().read_line(&mut s).unwrap();
        s = s.trim().to_string();

        if s.is_empty() {
            break;
        }

        let strs = s.split_whitespace().collect::<Vec<&str>>();
        let r = strs[0].parse::<f64>().unwrap();
        let s = strs[1].parse::<f64>().unwrap();

        writeln!(out, "{}", ((r * (s + 0.16)) / 0.067).sqrt().round() as i64).unwrap();
    }
}
