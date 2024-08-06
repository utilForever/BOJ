use io::Write;
use std::io;

fn main() {
    let stdout = io::stdout();
    let mut out = io::BufWriter::new(stdout.lock());

    let mut s = String::new();
    io::stdin().read_line(&mut s).unwrap();
    s = s.trim().to_string();

    let s = s.chars().collect::<Vec<_>>();
    let mut idx = 0;

    while idx < s.len() {
        if s[idx] == '<' {
            let mut word_piece = String::new();
            word_piece.push(s[idx]);
            idx += 1;

            while idx < s.len() && s[idx] != '>' {
                word_piece.push(s[idx]);
                idx += 1;
            }

            word_piece.push(s[idx]);
            idx += 1;

            write!(out, "{word_piece}").unwrap();
        } else if s[idx] == ' ' {
            write!(out, " ").unwrap();
            idx += 1;
        } else {
            let mut word_piece = String::new();
            word_piece.push(s[idx]);
            idx += 1;

            while idx < s.len() && (s[idx] != '<' && s[idx] != ' ') {
                word_piece.push(s[idx]);
                idx += 1;
            }

            let word_piece = word_piece.chars().rev().collect::<String>();
            write!(out, "{word_piece}").unwrap();
        }
    }

    writeln!(out).unwrap();
}
