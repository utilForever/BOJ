use io::Write;
use std::{collections::BTreeMap, io};

fn main() {
    let stdout = io::stdout();
    let mut out = io::BufWriter::new(stdout.lock());

    let mut s = String::new();
    io::stdin().read_line(&mut s).unwrap();
    s = s.trim().to_string();

    let t = s.parse::<i64>().unwrap();
    let mut mapping = BTreeMap::new();

    mapping.insert('y', 'a');
    mapping.insert('n', 'b');
    mapping.insert('f', 'c');
    mapping.insert('i', 'd');
    mapping.insert('c', 'e');
    mapping.insert('w', 'f');
    mapping.insert('l', 'g');
    mapping.insert('b', 'h');
    mapping.insert('k', 'i');
    mapping.insert('u', 'j');
    mapping.insert('o', 'k');
    mapping.insert('m', 'l');
    mapping.insert('x', 'm');
    mapping.insert('s', 'n');
    mapping.insert('e', 'o');
    mapping.insert('v', 'p');
    mapping.insert('z', 'q');
    mapping.insert('p', 'r');
    mapping.insert('d', 's');
    mapping.insert('r', 't');
    mapping.insert('j', 'u');
    mapping.insert('g', 'v');
    mapping.insert('t', 'w');
    mapping.insert('h', 'x');
    mapping.insert('a', 'y');
    mapping.insert('q', 'z');
    mapping.insert(' ', ' ');

    for i in 1..=t {
        let mut s = String::new();
        io::stdin().read_line(&mut s).unwrap();
        s = s.trim().to_string();

        writeln!(
            out,
            "Case #{i}: {}",
            s.chars().map(|c| mapping[&c]).collect::<String>()
        )
        .unwrap();
    }
}
