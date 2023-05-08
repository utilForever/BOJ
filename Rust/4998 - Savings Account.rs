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

        let mut iter = s.split_whitespace();
        let (n, b, m) = (
            iter.next().unwrap().parse::<f64>().unwrap(),
            iter.next().unwrap().parse::<f64>().unwrap(),
            iter.next().unwrap().parse::<f64>().unwrap(),
        );

        let mut amount = n;
        let mut ret = 0;

        while amount <= m {
            amount += amount * (b / 100.0);
            ret += 1;
        }

        writeln!(out, "{ret}").unwrap();
    }
}
