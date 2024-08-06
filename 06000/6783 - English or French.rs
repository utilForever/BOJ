use io::Write;
use std::io;

fn main() {
    let stdout = io::stdout();
    let mut out = io::BufWriter::new(stdout.lock());

    let mut s = String::new();
    io::stdin().read_line(&mut s).unwrap();
    s = s.trim().to_string();

    let n = s.parse::<i64>().unwrap();
    let mut cnt_t = 0;
    let mut cnt_s = 0;

    for _ in 0..n {
        s.clear();
        io::stdin().read_line(&mut s).unwrap();
        s = s.trim().to_string();

        for c in s.chars() {
            if c == 't' || c == 'T' {
                cnt_t += 1;
            } else if c == 's' || c == 'S' {
                cnt_s += 1;
            }
        }
    }

    writeln!(out, "{}", if cnt_t > cnt_s { "English" } else { "French" }).unwrap();
}
