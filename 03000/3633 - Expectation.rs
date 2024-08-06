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

    let k = scan.token::<i64>();

    let count_ones = |n: i64, filter: i64| -> i64 {
        let val = n / (2 * filter) * filter;

        if n - 2 * val < filter {
            val
        } else {
            n - val - filter
        }
    };

    for _ in 0..k {
        let n = scan.token::<i64>();
        let mut ret = 0.0;

        for i in 0..=30 {
            let filter = 1 << i;
            let p = count_ones(n, filter) as f64 / n as f64;
            ret += 2.0 * p * (1.0 - p) * filter as f64;
        }

        writeln!(out, "{:.2}", ret).unwrap();
    }
}
