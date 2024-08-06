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

        let s = s.split_whitespace().collect::<Vec<&str>>();
        let (mut n, k) = (s[0].parse::<i64>().unwrap(), s[1].parse::<i64>().unwrap());
        let mut ret = n;

        while n >= k {
            let r = n % k;
            n = n / k;
            ret += n;
            n += r;
        }

        writeln!(out, "{ret}").unwrap();
    }
}
