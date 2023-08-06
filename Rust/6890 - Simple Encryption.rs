use io::Write;
use std::io;

fn main() {
    let stdout = io::stdout();
    let mut out = io::BufWriter::new(stdout.lock());

    let mut s = String::new();
    io::stdin().read_line(&mut s).unwrap();
    let s = s.trim().to_string();
    let s = s.chars().collect::<Vec<_>>();

    let mut t = String::new();
    io::stdin().read_line(&mut t).unwrap();
    let t = t.trim().to_string();

    let mut idx = 0;
    let mut ret = String::new();

    for ch in t.chars() {
        if !ch.is_alphabetic() {
            continue;
        }

        let ch_converted = (ch as u8 - b'A' + s[idx] as u8 - b'A') % 26;
        ret.push((ch_converted + b'A') as char);

        idx = (idx + 1) % s.len();
    }

    writeln!(out, "{ret}").unwrap();
}
