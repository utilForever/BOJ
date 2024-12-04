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

const MOD: usize = 1_000_000_007;

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let n = scan.token::<usize>();
    let mut computers = vec![0; n];

    for i in 0..n {
        let (a, m) = (scan.token::<usize>(), scan.token::<usize>());
        let cost = (a * m) / 625;

        computers[i] = cost / 6;
    }

    let (mut c1, mut c2) = (scan.token::<usize>(), scan.token::<usize>());
    c1 = (c1 + 5) / 6;
    c2 = c2 / 6;

    let computers = computers
        .into_iter()
        .filter(|&x| x <= c2)
        .collect::<Vec<_>>();
    let mut dp = vec![0; c2 + 1];

    dp[0] = 1;

    for computer in computers {
        for cost in (computer..=c2).rev() {
            dp[cost] = (dp[cost] + dp[cost - computer]) % MOD;
        }
    }

    let mut ret = 0;

    for val in c1..=c2 {
        ret = (ret + dp[val]) % MOD;
    }

    writeln!(out, "{ret}").unwrap();
}
