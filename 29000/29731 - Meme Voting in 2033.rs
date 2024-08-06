use io::Write;
use std::io;

fn main() {
    let stdout = io::stdout();
    let mut out = io::BufWriter::new(stdout.lock());

    let sentences = [
        "Never gonna give you up",
        "Never gonna let you down",
        "Never gonna run around and desert you",
        "Never gonna make you cry",
        "Never gonna say goodbye",
        "Never gonna tell a lie and hurt you",
        "Never gonna stop",
    ];

    let mut s = String::new();
    io::stdin().read_line(&mut s).unwrap();
    let n = s.trim().parse::<i64>().unwrap();
    let mut ret = true;

    for _ in 0..n {
        let mut sentence = String::new();
        io::stdin().read_line(&mut sentence).unwrap();
        let sentence = sentence.trim();

        if !sentences.contains(&sentence) {
            ret = false;
        }
    }

    writeln!(out, "{}", if ret { "No" } else { "Yes" }).unwrap();
}
