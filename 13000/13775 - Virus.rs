use io::Write;
use std::io;

fn main() {
    let stdout = io::stdout();
    let mut out = io::BufWriter::new(stdout.lock());

    let mut s = String::new();
    io::stdin().read_line(&mut s).unwrap();
    s = s.trim().to_string();

    let n = s.parse::<usize>().unwrap();

    s.clear();
    io::stdin().read_line(&mut s).unwrap();
    s = s.trim().to_string();

    let mut ret = String::new();
    let mut offset = 26 - n;

    for c in s.chars() {
        if !c.is_alphabetic() {
            ret.push(c);
            continue;
        }

        let converted = (c as u8 - 'a' as u8 + offset as u8) % 26 + 'a' as u8;
        ret.push(converted as char);

        offset -= 1;

        if offset == 0 {
            offset = 25;
        }
    }

    writeln!(out, "{ret}").unwrap();
}
