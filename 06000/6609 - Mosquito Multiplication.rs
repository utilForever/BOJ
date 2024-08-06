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

        let s = s.split_whitespace().collect::<Vec<_>>();
        let (mut m, mut p, mut l, e, r, s, n) = (
            s[0].parse::<i64>().unwrap(),
            s[1].parse::<i64>().unwrap(),
            s[2].parse::<i64>().unwrap(),
            s[3].parse::<i64>().unwrap(),
            s[4].parse::<i64>().unwrap(),
            s[5].parse::<i64>().unwrap(),
            s[6].parse::<i64>().unwrap(),
        );

        for _ in 0..n {
            let new_l = m * e;
            let new_p = l / r;
            let new_m = p / s;

            m = new_m;
            p = new_p;
            l = new_l;
        }

        writeln!(out, "{m}").unwrap();
    }
}
