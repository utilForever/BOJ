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
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let t = scan.token::<i64>();
    let mut table = HashMap::new();

    table.insert('A', 2);
    table.insert('B', 2);
    table.insert('C', 2);
    table.insert('D', 3);
    table.insert('E', 3);
    table.insert('F', 3);
    table.insert('G', 4);
    table.insert('H', 4);
    table.insert('I', 4);
    table.insert('J', 5);
    table.insert('K', 5);
    table.insert('L', 5);
    table.insert('M', 6);
    table.insert('N', 6);
    table.insert('O', 6);
    table.insert('P', 7);
    table.insert('Q', 7);
    table.insert('R', 7);
    table.insert('S', 7);
    table.insert('T', 8);
    table.insert('U', 8);
    table.insert('V', 8);
    table.insert('W', 9);
    table.insert('X', 9);
    table.insert('Y', 9);
    table.insert('Z', 9);

    for _ in 0..t {
        let mut phone_number = scan.token::<String>();
        phone_number.retain(|c| c != '-');
        phone_number.truncate(10);

        let mut ret = String::new();

        for c in phone_number.chars() {
            if c.is_numeric() {
                ret.push(c);
            } else {
                ret.push_str(&table[&c].to_string());
            }
        }

        ret.insert(3, '-');
        ret.insert(7, '-');

        writeln!(out, "{ret}").unwrap();
    }
}
