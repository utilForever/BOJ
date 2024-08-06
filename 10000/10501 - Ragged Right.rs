use io::Write;
use std::io;

fn main() {
    let stdout = io::stdout();
    let mut out = io::BufWriter::new(stdout.lock());

    let mut lines = Vec::new();

    loop {
        let mut s = String::new();
        io::stdin().read_line(&mut s).unwrap();
        s = s.trim().to_string();

        if s.is_empty() {
            break;
        }

        lines.push(s);
    }

    let n = lines.iter().map(|x| x.len()).max().unwrap();
    let mut ret = 0;

    for i in 0..lines.len() - 1 {
        let m = lines[i].len();
        ret += (n - m) * (n - m);
    }

    writeln!(out, "{ret}").unwrap();
}
