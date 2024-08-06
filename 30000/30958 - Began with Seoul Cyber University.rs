use io::Write;
use std::io;

fn main() {
    let stdout = io::stdout();
    let mut out = io::BufWriter::new(stdout.lock());

    let mut s = String::new();
    io::stdin().read_line(&mut s).unwrap();
    let _ = s.trim().parse::<i64>().unwrap();
    let mut alphabets = [0; 26];

    let mut s = String::new();
    io::stdin().read_line(&mut s).unwrap();

    for ch in s.trim().chars() {
        if !ch.is_alphabetic() {
            continue;
        }

        alphabets[(ch as u8 - 'a' as u8) as usize] += 1;
    }

    writeln!(out, "{}", alphabets.iter().max().unwrap()).unwrap();
}
