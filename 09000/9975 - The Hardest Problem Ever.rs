use io::Write;
use std::io;

fn main() {
    let stdout = io::stdout();
    let mut out = io::BufWriter::new(stdout.lock());

    loop {
        let mut s = String::new();
        io::stdin().read_line(&mut s).unwrap();
        s = s.trim().to_string();

        if s == "ENDOFINPUT" {
            break;
        }

        s.clear();
        io::stdin().read_line(&mut s).unwrap();
        s = s.trim().to_string();

        for c in s.chars() {
            if c.is_ascii_alphabetic() {
                let c = (c as u8 - b'A' + 21) % 26 + b'A';
                write!(out, "{}", c as char).unwrap();
            } else {
                write!(out, "{c}").unwrap();
            }
        }

        writeln!(out).unwrap();

        s.clear();
        io::stdin().read_line(&mut s).unwrap();
    }
}
