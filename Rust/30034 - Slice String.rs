use io::Write;
use std::io;

fn main() {
    let stdout = io::stdout();
    let mut out = io::BufWriter::new(stdout.lock());

    let mut delimeter: Vec<char> = Vec::new();

    let mut s = String::new();
    io::stdin().read_line(&mut s).unwrap();

    let mut s = String::new();
    io::stdin().read_line(&mut s).unwrap();

    for ch in s.split_whitespace() {
        delimeter.push(ch.parse::<char>().unwrap());
    }

    let mut s = String::new();
    io::stdin().read_line(&mut s).unwrap();

    let mut s = String::new();
    io::stdin().read_line(&mut s).unwrap();

    for ch in s.split_whitespace() {
        delimeter.push(ch.parse::<char>().unwrap());
    }

    let mut s = String::new();
    io::stdin().read_line(&mut s).unwrap();

    let mut s = String::new();
    io::stdin().read_line(&mut s).unwrap();

    for ch in s.split_whitespace() {
        delimeter.retain(|&x| x != ch.parse::<char>().unwrap());
    }

    let mut s = String::new();
    io::stdin().read_line(&mut s).unwrap();

    let mut s = String::new();
    io::stdin().read_line(&mut s).unwrap();

    for ch in delimeter {
        s = s.replace(ch, " ");
    }

    for ch in s.split_whitespace() {
        writeln!(out, "{ch}").unwrap();
    }
}
