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

    let (n, q) = (scan.token::<usize>(), scan.token::<usize>());
    let mut lines = vec![(0, 0); n];
    let mut queries = vec![(0, 0); q];

    for i in 0..n {
        lines[i] = (scan.token::<usize>(), scan.token::<i64>());
    }

    for i in 0..q {
        queries[i] = (scan.token::<usize>(), scan.token::<i64>());
    }

    lines.sort_by(|a, b| a.1.cmp(&b.1));

    let mut dp = vec![i64::MIN; 1_000_001];
    let mut ret = vec![false; q];

    dp[0] = 0;

    for (p, l) in lines {
        for i in (p..=1_000_000).rev() {
            dp[i] = dp[i].max(dp[i - p] + l);
        }

        for (idx, &(a, b)) in queries.iter().enumerate() {
            ret[idx] |= dp[a] >= b && l <= b;
        }
    }

    for val in ret {
        writeln!(out, "{}", if val { "YES" } else { "NO" }).unwrap();
    }
}
