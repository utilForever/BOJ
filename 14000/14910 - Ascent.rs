use io::Write;
use std::io;

fn main() {
    let stdout = io::stdout();
    let mut out = io::BufWriter::new(stdout.lock());

    let mut s = String::new();
    io::stdin().read_line(&mut s).unwrap();
    let nums = s
        .split_whitespace()
        .map(|x| x.parse::<i64>().unwrap())
        .collect::<Vec<_>>();
    let ret = nums.windows(2).all(|x| x[0] <= x[1]);

    writeln!(out, "{}", if ret { "Good" } else { "Bad" }).unwrap();
}
