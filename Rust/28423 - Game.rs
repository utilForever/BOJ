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

fn calculate(dp: &mut Vec<i64>, n: usize) -> i64 {
    if n > 100000 {
        return -1;
    }

    if dp[n] == -2 {
        dp[n] = 0;
        return 0;
    }

    if dp[n] != -3 {
        return dp[n];
    }

    dp[n] = -2;

    let mut a = 0;
    let mut b = 1;
    let s = n.to_string().chars().collect::<Vec<_>>();

    for c in s {
        let d = c.to_digit(10).unwrap() as i64;
        a += d;
        b *= d;
    }

    let next = (a.to_string() + &b.to_string()).parse::<usize>().unwrap();
    dp[n] = if next == n {
        1
    } else {
        calculate(dp, next)
    };

    dp[n]
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let (l, r) = (scan.token::<usize>(), scan.token::<usize>());
    let mut dp = vec![-3; 100001];
    let mut ret = 0;

    for i in l..=r {
        ret += calculate(&mut dp, i);
    }

    writeln!(out, "{ret}").unwrap();
}
