use io::Write;
use std::io;

fn main() {
    let stdout = io::stdout();
    let mut out = io::BufWriter::new(stdout.lock());

    let mut s = String::new();
    io::stdin().read_line(&mut s).unwrap();
    let n = s.trim().parse::<i64>().unwrap();

    for i in 1..=n {
        let mut s = String::new();
        io::stdin().read_line(&mut s).unwrap();
        s = s.trim().to_string();

        let strs = s.split_whitespace().collect::<Vec<&str>>();

        write!(out, "Case #{i}: ").unwrap();

        for j in (0..strs.len()).rev() {
            write!(out, "{} ", strs[j]).unwrap();
        }

        writeln!(out).unwrap();
    }
}
