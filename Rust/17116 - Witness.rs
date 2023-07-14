use io::Write;
use std::io;

fn main() {
    let stdout = io::stdout();
    let mut out = io::BufWriter::new(stdout.lock());

    let mut s = String::new();
    io::stdin().read_line(&mut s).unwrap();
    s = s.trim().to_string();

    let mut t = String::new();
    io::stdin().read_line(&mut t).unwrap();

    let words = s.split_whitespace().collect::<Vec<_>>();

    writeln!(
        out,
        "{}",
        match words[0] {
            "KEY" => "BABA IS WIN",
            "BABA" => "BABA IS NOT WIN",
            "LONELY" => "BABA IS WIN",
            "TEXT" => "BABA IS NOT WIN",
            _ => unreachable!(),
        }
    )
    .unwrap();
}
