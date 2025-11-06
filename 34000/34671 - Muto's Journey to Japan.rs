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

    let (_, m, q) = (
        scan.token::<usize>(),
        scan.token::<usize>(),
        scan.token::<i32>(),
    );
    let mut edges = HashMap::with_capacity(m * 2);

    for _ in 0..m {
        let (a, b, c) = (
            scan.token::<u64>(),
            scan.token::<u64>(),
            scan.token::<i64>(),
        );
        let (a, b) = (a.min(b), a.max(b));
        
        edges
            .entry((a << 32) | b)
            .and_modify(|x| {
                if *x > c {
                    *x = c
                }
            })
            .or_insert(c);
    }

    for _ in 0..q {
        let (s, e) = (scan.token::<u64>(), scan.token::<u64>());
        let (s, e) = (s.min(e), s.max(e));

        if let Some(&cost) = edges.get(&((s << 32) | e)) {
            writeln!(out, "{cost}").unwrap();
        } else {
            writeln!(out, "-1").unwrap();
        }
    }
}
