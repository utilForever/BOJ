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

        let mut str = str.chars().collect::<Vec<_>>();

        for c in str.iter_mut() {
            if *c == 'i' {
                *c = 'e';
            } else if *c == 'e' {
                *c = 'i';
            } else if *c == 'I' {
                *c = 'E';
            } else if *c == 'E' {
                *c = 'I';
            }
        }

        writeln!(out, "{}", str.iter().collect::<String>()).unwrap();
    }
}
