use io::Write;
use std::io;

fn main() {
    let stdout = io::stdout();
    let mut out = io::BufWriter::new(stdout.lock());

    let mut document = String::new();
    io::stdin().read_line(&mut document).unwrap();
    document = document.trim().to_string();

    let mut search = String::new();
    io::stdin().read_line(&mut search).unwrap();
    search = search.trim().to_string();

    let mut idx = 0;
    let mut ret = 0;

    while idx < document.len() {
        if document[idx..].starts_with(&search) {
            ret += 1;
            idx += search.len();
        } else {
            idx += 1;
        }
    }

    writeln!(out, "{ret}").unwrap();
}
