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

    let (t, p) = (scan.token::<usize>(), scan.token::<i64>());
    let mut factorial = vec![0_i64; 401];
    let mut dp = vec![vec![0_i64; 401]; 401];
    let mut order = vec![0_i64; 401];
    let mut ret = vec![0_i64; 401];

    factorial[0] = 1;
    for i in 1..=400 {
        factorial[i] = (factorial[i - 1] * i as i64) % p;
    }

    dp[0][0] = 1;
    for i in 1..=400 {
        for j in 1..=i {
            for k in 1..=i {
                dp[i][j] = (dp[i][j] + dp[i - k][j - 1] * factorial[k]) % p;
            }
        }
    }

    for i in 1..=400 {
        order[i] = factorial[i];

        for j in 1..=i - 1 {
            order[i] = (order[i] - order[j] * factorial[i - j]) % p;
        }

        order[i] = (order[i] % p + p) % p;
    }

    for i in 1..=400 {
        ret[i] = factorial[i];

        for j in 1..=i - 1 {
            if i <= 2 {
                continue;
            }

            ret[i] = (ret[i] - order[j] * factorial[i - j] * 2) % p;
        }

        for j in 3..=i - 1 {
            ret[i] = (ret[i] - dp[i][j] * ret[j]) % p;
        }

        ret[i] = (ret[i] % p + p) % p;
    }

    for _ in 0..t {
        let n = scan.token::<usize>();
        writeln!(out, "{}", ret[n]).unwrap();
    }
}
