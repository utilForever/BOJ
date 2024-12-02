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
    let s = scan.token::<String>().chars().collect::<Vec<_>>();
    let patterns = [['R', 'S', 'P'], ['P', 'R', 'S'], ['S', 'P', 'R']];
    let mut dp = [0; 3];

    for i in 0..n {
        for j in 0..3 {
            if s[i] == patterns[j][dp[j] % 3] {
                dp[j] += 1;
            }
        }
    }

    let mut ret = usize::MAX;

    for i in 0..3 {
        ret = ret.min(n - (dp[i] - dp[i] % 3));
    }

    writeln!(out, "{ret}").unwrap();
}
