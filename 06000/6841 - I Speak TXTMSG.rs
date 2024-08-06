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

        match str {
            "CU" => writeln!(out, "see you").unwrap(),
            ":-)" => writeln!(out, "I’m happy").unwrap(),
            ":-(" => writeln!(out, "I’m unhappy").unwrap(),
            ";-)" => writeln!(out, "wink").unwrap(),
            ":-P" => writeln!(out, "stick out my tongue").unwrap(),
            "(~.~)" => writeln!(out, "sleepy").unwrap(),
            "TA" => writeln!(out, "totally awesome").unwrap(),
            "CCC" => writeln!(out, "Canadian Computing Competition").unwrap(),
            "CUZ" => writeln!(out, "because").unwrap(),
            "TY" => writeln!(out, "thank-you").unwrap(),
            "YW" => writeln!(out, "you’re welcome").unwrap(),
            "TTYL" => writeln!(out, "talk to you later").unwrap(),
            _ => writeln!(out, "{str}").unwrap(),
        }

        if str == "TTYL" {
            break;
        }
    }
}
