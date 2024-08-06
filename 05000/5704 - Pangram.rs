use io::Write;
use std::io;

fn main() {
    let stdout = io::stdout();
    let mut out = io::BufWriter::new(stdout.lock());

    loop {
        let mut s = String::new();
        io::stdin().read_line(&mut s).unwrap();
        s = s.trim().to_string();

        if s == "*" {
            break;
        }

        let s = s.chars().collect::<Vec<_>>();
        let mut alphabet = vec![false; 26];

        for c in s {
            if c == ' ' {
                continue;
            }

            let i = c.to_ascii_lowercase() as usize - 'a' as usize;
            alphabet[i] = true;
        }

        writeln!(
            out,
            "{}",
            if alphabet.iter().all(|&x| x) {
                "Y"
            } else {
                "N"
            }
        )
        .unwrap();
    }
}
