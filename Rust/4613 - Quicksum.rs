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

        let s = s.chars().collect::<Vec<_>>();
        let mut ret = 0;

        for (idx, c) in s.iter().enumerate() {
            if c.is_whitespace() {
                continue;
            }

            ret += (idx + 1) * (*c as usize - 'A' as usize + 1);
        }

        writeln!(out, "{ret}").unwrap();
    }
}
