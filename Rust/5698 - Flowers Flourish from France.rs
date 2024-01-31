use io::Write;
use std::io;

fn main() {
    let stdout = io::stdout();
    let mut out = io::BufWriter::new(stdout.lock());

    loop {
        let mut s = String::new();
        io::stdin().read_line(&mut s).unwrap();
        s = s.trim().to_string().to_uppercase();

        if s == "*" {
            break;
        }

        let letters = s
            .split_whitespace()
            .map(|s| s.chars().next().unwrap())
            .collect::<Vec<_>>();

        writeln!(
            out,
            "{}",
            if letters.iter().all(|&c| c == letters[0]) {
                "Y"
            } else {
                "N"
            }
        )
        .unwrap();
    }
}
