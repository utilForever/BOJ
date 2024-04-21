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

    let (n, m) = (scan.token::<usize>(), scan.token::<usize>());
    let mut items = vec![(0, 0); n];

    for i in 0..n {
        items[i].0 = scan.token::<usize>();
    }

    for i in 0..n {
        items[i].1 = scan.token::<usize>();
    }

    let sum = items.iter().map(|x| x.1).sum::<usize>();
    let mut dp = vec![0; sum + 1];
    let mut ret = usize::MAX;

    for i in 0..n {
        for j in (items[i].1..=sum).rev() {
            dp[j] = dp[j].max(dp[j - items[i].1] + items[i].0);

            if dp[j] >= m {
                ret = ret.min(j);
            }
        }
    }

    writeln!(out, "{ret}").unwrap();
}
