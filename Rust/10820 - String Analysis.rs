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

        if str.is_empty() {
            break;
        }

        let mut cnt_lowercase = 0;
        let mut cnt_uppercase = 0;
        let mut cnt_number = 0;
        let mut cnt_whitespace = 0;

        for c in str.chars() {
            if c.is_lowercase() {
                cnt_lowercase += 1;
            } else if c.is_uppercase() {
                cnt_uppercase += 1;
            } else if c.is_numeric() {
                cnt_number += 1;
            } else if c.is_whitespace() {
                cnt_whitespace += 1;
            }
        }

        writeln!(
            out,
            "{cnt_lowercase} {cnt_uppercase} {cnt_number} {cnt_whitespace}"
        )
        .unwrap();
    }
}
