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

        let n = s.parse::<i64>().unwrap();
        let mut val = 1;
        let mut ret = 1;

        loop {
            if val % n == 0 {
                writeln!(out, "{ret}").unwrap();
                break;
            }

            val %= n;
            val = (val * 10 + 1) % n;
            ret += 1;
        }
    }
}
