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

    pub fn all(&mut self) -> String {
        let mut input = String::new();
        self.reader.read_to_string(&mut input).expect("Failed read");
        input
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

    let n = scan.token::<usize>();
    let mut mapping = vec![0; 101];

    for _ in 0..n {
        let (a, b) = (scan.token::<usize>(), scan.token::<usize>());

        mapping[a] = b;
        mapping[b] = a;
    }

    let mut dp = vec![vec![0; 101]; 101];

    for len in 1..100 {
        for i in 1..=100 - len {
            let j = len + i;
            let mut val_max = dp[i][j - 1];

            if mapping[j] != 0 && mapping[j] >= i && mapping[j] < j {
                let val1 = if mapping[j] > i {
                    dp[i][mapping[j] - 1]
                } else {
                    0
                };
                let val2 = if mapping[j] + 1 <= j - 1 {
                    dp[mapping[j] + 1][j - 1]
                } else {
                    0
                };

                val_max = val_max.max(val1 + val2 + 1);
            }

            dp[i][j] = val_max;
        }
    }

    writeln!(out, "{}", dp[1][100]).unwrap();
}
