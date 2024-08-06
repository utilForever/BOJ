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

    let (k, l) = (scan.token::<usize>(), scan.token::<i64>());
    let mut map = HashMap::new();

    for i in 0..l {
        let num = scan.token::<String>();

        if map.contains_key(&num) {
            map.entry(num).and_modify(|e| *e = i);
        } else {
            map.insert(num, i);
        }
    }

    let mut ret = map.iter().collect::<Vec<_>>();
    ret.sort_by(|a, b| a.1.cmp(b.1));

    for (num, _) in ret.iter().take(k) {
        writeln!(out, "{num}").unwrap();
    }
}
