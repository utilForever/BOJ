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
    let mut heights = vec![0; n];

    for i in 0..n {
        heights[i] = scan.token::<usize>();
    }

    let sum = heights.iter().sum::<usize>();
    let mut dp = vec![-1; sum + 1];

    dp[0] = 0;

    for h in heights {
        let mut dp_new = dp.clone();

        for val in 0..=sum {
            if dp[val] < 0 {
                continue;
            }

            let val_high = val + h;

            if val_high <= sum {
                dp_new[val_high] = dp_new[val_high].max(dp[val]);
            }

            let val_low = if val >= h { val - h } else { h - val };

            dp_new[val_low] = dp_new[val_low].max(dp[val] + val.min(h) as i64);
        }

        dp = dp_new;
    }

    writeln!(out, "{}", if dp[0] > 0 { dp[0] as i64 } else { -1 }).unwrap();
}
