use io::Write;
use std::io;

fn main() {
    let stdout = io::stdout();
    let mut out = io::BufWriter::new(stdout.lock());

    let mut s = String::new();
    io::stdin().read_line(&mut s).unwrap();

    let q = s.trim().parse::<usize>().unwrap();
    let mut quotes = vec![String::new(); q + 1];

    for i in 1..=q {
        s.clear();
        io::stdin().read_line(&mut s).unwrap();
        quotes[i] = s.trim().to_string();
    }

    s.clear();
    io::stdin().read_line(&mut s).unwrap();

    let r = s.trim().parse::<usize>().unwrap();

    for _ in 0..r {
        s.clear();
        io::stdin().read_line(&mut s).unwrap();

        let num = s.trim().parse::<i64>().unwrap();

        write!(out, "Rule {num}: ").unwrap();

        if num >= 1 && num <= q as i64 {
            writeln!(out, "{}", quotes[num as usize]).unwrap();
        } else {
            writeln!(out, "No such rule").unwrap();
        }
    }
}
