use io::Write;
use std::io;

fn main() {
    let stdout = io::stdout();
    let mut out = io::BufWriter::new(stdout.lock());

    let mut s = String::new();
    io::stdin().read_line(&mut s).unwrap();
    let n = s.trim().parse::<i64>().unwrap();

    for _ in 0..n {
        let mut s = String::new();
        io::stdin().read_line(&mut s).unwrap();
        s = s.trim().to_string();

        let s = s.chars().collect::<Vec<_>>();
        let mut alphabet = vec![false; 26];

        for c in s {
            if !c.is_alphabetic() {
                continue;
            }

            let i = c.to_ascii_lowercase() as usize - 'a' as usize;
            alphabet[i] = true;
        }

        if alphabet.iter().all(|&x| x) {
            writeln!(out, "pangram").unwrap();
        } else {
            write!(out, "missing ").unwrap();

            for (i, &val) in alphabet.iter().enumerate() {
                if !val {
                    write!(out, "{}", (i as u8 + 'a' as u8) as char).unwrap();
                }
            }

            writeln!(out).unwrap();
        }
    }
}
