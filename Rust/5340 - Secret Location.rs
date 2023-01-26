use io::Write;
use std::io;

fn main() {
    let stdout = io::stdout();
    let mut out = io::BufWriter::new(stdout.lock());

    write!(out, "Latitude ").unwrap();

    let mut s = String::new();
    io::stdin().read_line(&mut s).unwrap();
    let len1 = s.trim().len();

    s.clear();
    io::stdin().read_line(&mut s).unwrap();
    let len2 = s.trim().len();

    s.clear();
    io::stdin().read_line(&mut s).unwrap();
    let len3 = s.trim().len();

    writeln!(out, "{len1}:{len2}:{len3}").unwrap();

    write!(out, "Longitude ").unwrap();

    let mut s = String::new();
    io::stdin().read_line(&mut s).unwrap();
    let len1 = s.trim().len();

    s.clear();
    io::stdin().read_line(&mut s).unwrap();
    let len2 = s.trim().len();

    s.clear();
    io::stdin().read_line(&mut s).unwrap();
    let len3 = s.trim().len();

    writeln!(out, "{len1}:{len2}:{len3}").unwrap();
}
