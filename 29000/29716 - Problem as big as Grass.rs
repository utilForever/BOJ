use io::Write;
use std::io;

fn main() {
    let stdout = io::stdout();
    let mut out = io::BufWriter::new(stdout.lock());

    let mut s = String::new();
    io::stdin().read_line(&mut s).unwrap();

    let (j, n) = {
        let mut ws = s.split_whitespace();
        let j = ws.next().unwrap().parse::<i64>().unwrap();
        let n = ws.next().unwrap().parse::<i64>().unwrap();

        (j, n)
    };
    let mut ret = 0;

    for _ in 0..n {
        let mut s = String::new();
        io::stdin().read_line(&mut s).unwrap();

        let mut cnt = 0;

        for c in s.chars() {
            if c >= 'A' && c <= 'Z' {
                cnt += 4;
            } else if c >= 'a' && c <= 'z' {
                cnt += 2;
            } else if c >= '0' && c <= '9' {
                cnt += 2;
            } else if c == ' ' {
                cnt += 1;
            }
        }

        if cnt <= j {
            ret += 1;
        }
    }

    writeln!(out, "{ret}").unwrap();
}
