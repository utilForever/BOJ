use io::Write;
use std::io;

fn main() {
    let stdout = io::stdout();
    let mut out = io::BufWriter::new(stdout.lock());

    loop {
        let mut s = String::new();
        io::stdin().read_line(&mut s).unwrap();
        s = s.trim().to_string();

        if s == "EOI" {
            break;
        }

        let s = s.chars().collect::<Vec<_>>();
        let mut ret = false;

        s.windows(4)
            .map(|w| w.iter().map(|c| c.to_ascii_uppercase()).collect::<String>())
            .filter(|w| w == "NEMO")
            .for_each(|_| ret = true);

        writeln!(out, "{}", if ret { "Found" } else { "Missing" }).unwrap();
    }
}
