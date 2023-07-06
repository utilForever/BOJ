use io::Write;
use std::io;

fn main() {
    let stdout = io::stdout();
    let mut out = io::BufWriter::new(stdout.lock());

    loop {
        let mut s = String::new();
        io::stdin().read_line(&mut s).unwrap();
        s = s.trim().to_string();

        if s == "#" {
            break;
        }

        let words = s.split_whitespace().collect::<Vec<_>>();
        let mut ret = String::new();

        for word in words {
            let mut chars = word.chars().collect::<Vec<_>>();
            chars.reverse();

            let reversed = chars.into_iter().collect::<String>();
            ret.push_str(&reversed);
            ret.push(' ');
        }

        ret.pop();
        writeln!(out, "{ret}").unwrap();
    }
}
