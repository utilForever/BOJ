use io::Write;
use std::io;

fn main() {
    let stdout = io::stdout();
    let mut out = io::BufWriter::new(stdout.lock());
    let buf = io::read_to_string(io::stdin()).unwrap();

    let mut alphabets = [0; 26];

    for c in buf.chars() {
        if c.is_alphabetic() {
            alphabets[c as usize - 97] += 1;
        }
    }

    let ret = *alphabets.iter().max().unwrap();

    for (i, &val) in alphabets.iter().enumerate() {
        if val == ret {
            write!(out, "{}", (i + 97) as u8 as char).unwrap();
        }
    }

    writeln!(out).unwrap();
}
