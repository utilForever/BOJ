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
    let mut ret = 0;

    loop {
        let mut s = String::new();
        io::stdin().read_line(&mut s).unwrap();

        let str = strip_trailing_newline(&s);

        if str.is_empty() {
            break;
        }

        ret += 1;
    }

    writeln!(out, "{ret}").unwrap();
}
