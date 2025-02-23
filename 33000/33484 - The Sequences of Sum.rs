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

fn check(val: i128, m: i128) -> i128 {
    if val == 0 {
        return m * (m + 1) / 2;
    }

    let p = m * m + 1;
    let a = val / m;
    let b = val % m;
    let mut k = check(a, m);

    let mut left = a * p + 1 + b * m;
    let mut right = a * p + 1 + b * m + m - 1;

    if k < left {
        left += 1;
        right += 1;
    }

    if left <= k && right >= k {
        right += 1;
    } else {
        k = 0;
    }

    (left + right) * (right - left + 1) / 2 - k
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let (m, q) = (scan.token::<i128>(), scan.token::<i64>());

    for _ in 0..q {
        let n = scan.token::<i128>();
        let temp = m * (m + 1) / 2;

        if m == 1 {
            writeln!(out, "1").unwrap();
        } else if temp > 10i128.pow(18) {
            writeln!(out, "0").unwrap();
        } else {
            let p = m * m + 1;
            writeln!(
                out,
                "{}",
                if check((n - 1) / p, m) == n { "1" } else { "0" }
            )
            .unwrap();
        }
    }
}
