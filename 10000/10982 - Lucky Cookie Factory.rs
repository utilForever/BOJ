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
    let mut tasks = vec![(0, 0); n];

    for i in 0..n {
        tasks[i] = (scan.token::<usize>(), scan.token::<usize>());
    }

    let sum_a = tasks.iter().map(|(a, _)| *a).sum::<usize>();
    let sum_b = tasks.iter().map(|(_, b)| *b).sum::<usize>();
    let mut dp = vec![0; sum_a + 1];

    for i in 0..=sum_a {
        dp[i] = sum_b;
    }

    for i in 0..n {
        let (a, b) = tasks[i];

        for i in (a..=sum_a).rev() {
            dp[i] = dp[i].min(dp[i - a] - b);
        }
    }

    let mut ret = usize::MAX;

    for i in 1..=sum_a {
        ret = ret.min(dp[i].max(i));
    }

    writeln!(out, "{ret}").unwrap();
}
