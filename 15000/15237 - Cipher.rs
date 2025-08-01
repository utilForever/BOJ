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

    pub fn all(&mut self) -> String {
        let mut input = String::new();
        self.reader.read_to_string(&mut input).expect("Failed read");
        input
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

    let (n, _) = (scan.token::<i64>(), scan.token::<i64>());
    let mut map: HashMap<i64, (i64, i64)> = HashMap::new();

    for i in 0..n {
        let key = scan.token::<i64>();
        map.entry(key).and_modify(|e| e.1 += 1).or_insert((i, 1));
    }

    let mut vals = map.into_iter().collect::<Vec<_>>();
    vals.sort_by(|a, b| {
        if a.1 .1 == b.1 .1 {
            a.1 .0.cmp(&b.1 .0)
        } else {
            b.1 .1.cmp(&a.1 .1)
        }
    });

    for (k, v) in vals {
        for _ in 0..v.1 {
            write!(out, "{k} ").unwrap();
        }
    }

    writeln!(out).unwrap();
}
