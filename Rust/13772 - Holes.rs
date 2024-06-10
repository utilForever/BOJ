use io::Write;
use std::io;

fn main() {
    let stdout = io::stdout();
    let mut out = io::BufWriter::new(stdout.lock());

    let mut s = String::new();
    io::stdin().read_line(&mut s).unwrap();
    s = s.trim().to_string();

    let n = s.parse::<i64>().unwrap();
    let holes = [
        1, 2, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0,
    ];

    for _ in 0..n {
        s.clear();
        io::stdin().read_line(&mut s).unwrap();
        s = s.trim().to_string();

        let mut ret = 0;

        for c in s.chars() {
            if c == ' ' {
                continue;
            }

            let idx = c as usize - 'A' as usize;
            ret += holes[idx];
        }

        writeln!(out, "{ret}").unwrap();
    }
}
