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
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let n = scan.token::<usize>();
    let mut data = vec![(0, 0); n];

    for i in 0..n {
        data[i] = (scan.token::<i64>(), scan.token::<i64>());
    }

    let mut rss_global = i64::MAX;
    let mut ret = (0, 0);

    for a in 1..=100 {
        for b in 1..=100 {
            let rss_local = data
                .iter()
                .fold(0, |acc, (x, y)| acc + (y - (a * x + b)).pow(2));

            if rss_local < rss_global {
                rss_global = rss_local;
                ret = (a, b);
            }
        }
    }

    writeln!(out, "{} {}", ret.0, ret.1).unwrap();
}
