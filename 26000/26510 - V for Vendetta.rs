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

    for num in nums {
        for i in 0..num {
            for _ in 0..i {
                write!(out, " ").unwrap();
            }

            if i != num - 1 {
                write!(out, "*").unwrap();
            }

            for _ in 0..2 * (num - i) - 3 {
                write!(out, " ").unwrap();
            }

            writeln!(out, "*").unwrap();
        }
    }
}
