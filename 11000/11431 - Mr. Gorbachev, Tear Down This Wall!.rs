use io::Write;
use std::{io, str};

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

    let k = scan.token::<i64>();

    for i in 1..=k {
        let (n, s, p) = (
            scan.token::<usize>(),
            scan.token::<i64>(),
            scan.token::<i64>(),
        );
        let mut points = vec![(0, 0); n + 1];

        for j in 0..=n {
            points[j] = (scan.token::<i64>(), scan.token::<i64>());
        }

        let mut sum = 0;

        for j in 1..=n {
            sum += (points[j].0 - points[j - 1].0).abs() + (points[j].1 - points[j - 1].1).abs();
        }

        let ret = (sum * s + p - 1) / p;

        writeln!(out, "Data Set {i}:").unwrap();
        writeln!(out, "{ret}").unwrap();

        if i != k {
            writeln!(out).unwrap();
        }
    }
}
