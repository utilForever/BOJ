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
        let (s, t) = (
            s[0].chars().collect::<Vec<_>>(),
            s[1].chars().collect::<Vec<_>>(),
        );

        let mut idx_s = 0;
        let mut idx_t = 0;

        while idx_s < s.len() && idx_t < t.len() {
            if s[idx_s] == t[idx_t] {
                idx_s += 1;
            }

            idx_t += 1;
        }

        writeln!(out, "{}", if idx_s == s.len() { "Yes" } else { "No" }).unwrap();
    }
}
