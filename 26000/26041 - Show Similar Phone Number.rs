use io::Write;
use std::io;

fn main() {
    let stdout = io::stdout();
    let mut out = io::BufWriter::new(stdout.lock());

    let mut a = String::new();
    io::stdin().read_line(&mut a).unwrap();
    let a = a.trim().split_whitespace().collect::<Vec<_>>();

    let mut b = String::new();
    io::stdin().read_line(&mut b).unwrap();
    let b = b.trim().to_string();

    let mut ret = 0;

    for telephone in a.iter() {
        if telephone.len() > b.len() && telephone[0..b.len()] == b {
            ret += 1;
        }
    }

    writeln!(out, "{ret}").unwrap();
}
