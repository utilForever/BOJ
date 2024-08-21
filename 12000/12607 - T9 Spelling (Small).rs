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

    let t = scan.token::<i64>();
    let mut map = HashMap::new();

    map.insert('a', 2);
    map.insert('b', 22);
    map.insert('c', 222);
    map.insert('d', 3);
    map.insert('e', 33);
    map.insert('f', 333);
    map.insert('g', 4);
    map.insert('h', 44);
    map.insert('i', 444);
    map.insert('j', 5);
    map.insert('k', 55);
    map.insert('l', 555);
    map.insert('m', 6);
    map.insert('n', 66);
    map.insert('o', 666);
    map.insert('p', 7);
    map.insert('q', 77);
    map.insert('r', 777);
    map.insert('s', 7777);
    map.insert('t', 8);
    map.insert('u', 88);
    map.insert('v', 888);
    map.insert('w', 9);
    map.insert('x', 99);
    map.insert('y', 999);
    map.insert('z', 9999);
    map.insert(' ', 0);
    map.insert('A', -1);

    for i in 1..=t {
        let s = scan.line().to_string();
        let mut prev = 'A';
        let mut ret = String::new();

        for c in s.chars() {
            if c == '\r' || c == '\n' {
                break;
            }

            if map[&c] % 10 == map[&prev] % 10  {
                ret.push_str(" ");
            }

            ret.push_str(&map[&c].to_string());
            prev = c;
        }

        writeln!(out, "Case #{i}: {ret}").unwrap();
    }
}
