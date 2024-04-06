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

    let (h1, m1) = (scan.token::<i64>(), scan.token::<i64>());
    let (h2, m2) = (scan.token::<i64>(), scan.token::<i64>());
    let n = scan.token::<i64>();

    let start = h1 * 60 + m1;
    let end = h2 * 60 + m2;
    let mut ret = 0;

    for i in start..=end {
        let (h, m) = (i / 60, i % 60);

        if h / 10 == n || h % 10 == n || m / 10 == n || m % 10 == n {
            ret += 1;
        }
    }

    writeln!(out, "{ret}").unwrap();
}
