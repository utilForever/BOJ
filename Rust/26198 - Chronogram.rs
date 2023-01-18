use io::Write;
use std::io;

fn main() {
    let stdout = io::stdout();
    let mut out = io::BufWriter::new(stdout.lock());

    let mut s = String::new();
    io::stdin().read_line(&mut s).unwrap();
    let n = s.trim().parse::<i64>().unwrap();

    for _ in 0..n {
        s.clear();
        io::stdin().read_line(&mut s).unwrap();
        let words = s.trim().split_whitespace().collect::<Vec<_>>();
        let mut ret = 0;

        for word in words.iter() {
            let strs = word.chars().collect::<Vec<_>>();

            for c in strs.iter() {
                match c {
                    'I' => ret += 1,
                    'V' => ret += 5,
                    'X' => ret += 10,
                    'L' => ret += 50,
                    'C' => ret += 100,
                    'D' => ret += 500,
                    'M' => ret += 1000,
                    _ => {}
                }
            }
        }

        writeln!(out, "{ret}").unwrap();
    }
}
