use io::Write;
use std::io;

fn main() {
    let stdout = io::stdout();
    let mut out = io::BufWriter::new(stdout.lock());

    let mut cnt = 1;

    loop {
        let mut s = String::new();
        io::stdin().read_line(&mut s).unwrap();
        s = s.trim().to_string();

        if s == "Was it a cat I saw?" {
            break;
        }

        let s = s.chars().collect::<Vec<_>>();

        let mut idx = 0;

        while idx < s.len() {
            write!(out, "{}", s[idx]).unwrap();
            idx += cnt + 1;
        }

        writeln!(out).unwrap();

        cnt += 1;
    }
}
