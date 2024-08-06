use io::Write;
use std::io;

fn main() {
    let stdout = io::stdout();
    let mut out = io::BufWriter::new(stdout.lock());

    loop {
        let mut s = String::new();
        io::stdin().read_line(&mut s).unwrap();
        s = s.trim().to_string();

        if s == "#" {
            break;
        }

        let mut alphabet = [false; 26];

        for c in s.chars() {
            if c.is_ascii_alphabetic() {
                alphabet[c.to_ascii_lowercase() as usize - 97] = true;
            }
        }

        writeln!(out, "{}", alphabet.iter().filter(|&&x| x).count()).unwrap();
    }
}
