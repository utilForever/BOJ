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

        let mut s = s.chars().collect::<Vec<_>>();
        let mut s_sorted = s.clone();

        s_sorted.sort();

        let mut ret = 0;

        for i in 0..s.len() {
            if s[i] != s_sorted[i] {
                let pos = s.iter().position(|&x| x == s_sorted[i]).unwrap();
                s.swap(i, pos);
                ret += 1;
            }
        }

        writeln!(out, "{ret}").unwrap();
    }
}
