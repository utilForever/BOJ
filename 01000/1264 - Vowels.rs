use io::Write;
use std::io;

fn main() {
    let stdout = io::stdout();
    let mut out = io::BufWriter::new(stdout.lock());

    loop {
        let mut s = String::new();
        io::stdin().read_line(&mut s).unwrap();

        if s.starts_with('#') {
            break;
        }

        let mut ret = 0;

        for c in s.chars() {
            match c {
                'A' | 'E' | 'I' | 'O' | 'U' | 'a' | 'e' | 'i' | 'o' | 'u' => ret += 1,
                _ => {}
            }
        }

        writeln!(out, "{}", ret).unwrap();
    }
}
