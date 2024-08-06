use io::Write;
use std::io;

fn main() {
    let stdout = io::stdout();
    let mut out = io::BufWriter::new(stdout.lock());

    let mut a = String::new();
    io::stdin().read_line(&mut a).unwrap();
    let mut a = a.chars().collect::<Vec<_>>();

    let mut b = String::new();
    io::stdin().read_line(&mut b).unwrap();

    let mut alphabets = [0; 26];

    for ch in b.split_whitespace() {
        let val = ch.parse::<char>().unwrap();
        alphabets[(val as u8 - 'A' as u8) as usize] += 1;
    }

    for ch in a.iter_mut() {
        let idx = (ch.to_ascii_uppercase() as u8 - 'A' as u8) as usize;

        if ch.is_uppercase() && alphabets[idx] > 0 {
            *ch = ch.to_ascii_lowercase();
        }
    }

    write!(out, "{}", a.iter().collect::<String>()).unwrap();
}
