use io::Write;
use std::{collections::HashMap, io, str};

pub struct UnsafeScanner<R> {
    reader: R,
    buf_str: Vec<u8>,
    buf_iter: str::SplitAsciiWhitespace<'static>,
}

impl<R: io::BufRead> UnsafeScanner<R> {
    pub fn new(reader: R) -> Self {
        Self {
            reader,
            buf_str: vec![],
            buf_iter: "".split_ascii_whitespace(),
        }
    }

    pub fn token<T: str::FromStr>(&mut self) -> T {
        loop {
            if let Some(token) = self.buf_iter.next() {
                return token.parse().ok().expect("Failed parse");
            }
            self.buf_str.clear();
            self.reader
                .read_until(b'\n', &mut self.buf_str)
                .expect("Failed read");
            self.buf_iter = unsafe {
                let slice = str::from_utf8_unchecked(&self.buf_str);
                std::mem::transmute(slice.split_ascii_whitespace())
            }
        }
    }

    pub fn line(&mut self) -> String {
        let mut input = String::new();
        self.reader.read_line(&mut input).expect("Failed read");
        input
    }
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let mut translation = HashMap::new();
    translation.insert('a', "@");
    translation.insert('b', "8");
    translation.insert('c', "(");
    translation.insert('d', "|)");
    translation.insert('e', "3");
    translation.insert('f', "#");
    translation.insert('g', "6");
    translation.insert('h', "[-]");
    translation.insert('i', "|");
    translation.insert('j', "_|");
    translation.insert('k', "|<");
    translation.insert('l', "1");
    translation.insert('m', "[]\\/[]");
    translation.insert('n', "[]\\[]");
    translation.insert('o', "0");
    translation.insert('p', "|D");
    translation.insert('q', "(,)");
    translation.insert('r', "|Z");
    translation.insert('s', "$");
    translation.insert('t', "\'][\'");
    translation.insert('u', "|_|");
    translation.insert('v', "\\/");
    translation.insert('w', "\\/\\/");
    translation.insert('x', "}{");
    translation.insert('y', "`/");
    translation.insert('z', "2");

    let mut text = scan.line().to_string();
    text = text.to_ascii_lowercase();

    let mut ret = String::new();

    for c in text.chars() {
        if c == '\r' || c == '\n' {
            continue;
        }

        if translation.contains_key(&c) {
            ret.push_str(translation.get(&c).unwrap());
        } else {
            ret.push(c);
        }
    }

    writeln!(out, "{ret}").unwrap();
}
