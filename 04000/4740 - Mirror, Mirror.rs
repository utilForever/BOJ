use io::Write;
use std::io;

fn strip_trailing_newline(input: &str) -> &str {
    input
        .strip_suffix("\r\n")
        .or(input.strip_suffix("\n"))
        .unwrap_or(input)
}

fn main() {
    let stdout = io::stdout();
    let mut out = io::BufWriter::new(stdout.lock());

    loop {
        let mut s = String::new();
        io::stdin().read_line(&mut s).unwrap();

        let str = strip_trailing_newline(&s);

        if str == "***" {
            break;
        }

        let ret = str.chars().rev().collect::<String>();
        writeln!(out, "{ret}").unwrap();
    }
}
