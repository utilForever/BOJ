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
    let mut healths = vec![0; n];
    let mut happiness = vec![0; n];

    for i in 0..n {
        healths[i] = scan.token::<i64>();
    }

    for i in 0..n {
        happiness[i] = scan.token::<i64>();
    }

    let mut dp = vec![(0, 0); 1 << n];

    for mask in 1..1 << n {
        let mut h = 0;
        let mut s = 0;

        for i in 0..n {
            if mask & 1 << i != 0 {
                h += healths[i];
                s += happiness[i];
            }
        }

        dp[mask] = (h, s);
    }

    let mut ret = 0;

    for mask in 1..1 << n {
        if dp[mask].0 >= 100 {
            continue;
        }

        ret = ret.max(dp[mask].1);
    }

    writeln!(out, "{ret}").unwrap();
}
