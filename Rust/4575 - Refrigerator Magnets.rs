use io::Write;
use std::io;

fn main() {
    let stdout = io::stdout();
    let mut out = io::BufWriter::new(stdout.lock());

    loop {
        let mut s = String::new();
        io::stdin().read_line(&mut s).unwrap();
        s = s.trim().to_string();

        if s == "END" {
            break;
        }

        let mut alphabet = [0; 26];

        for c in s.chars() {
            if c == ' ' {
                continue;
            }

            let index = c as usize - 'A' as usize;
            alphabet[index] += 1;
        }

        if alphabet.iter().any(|&x| x > 1) {
            continue;
        }

        writeln!(out, "{s}").unwrap();
    }
}
