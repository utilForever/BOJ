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

    let (n, k) = (scan.token::<usize>(), scan.token::<usize>());
    let mut items = vec![(0, 0); n];

    for i in 0..n {
        items[i] = (scan.token::<i64>(), scan.token::<i64>());
    }

    let s_max = 1 + items.iter().map(|x| x.1).max().unwrap() as usize * k;
    let mut dp = vec![vec![i64::MIN; s_max + 1]; k + 1];
    
    dp[0][1] = 0;

    for i in 0..k {
        for j in 1..=s_max {
            if dp[i][j] < 0 {
                continue;
            }

            dp[i + 1][j] = dp[i + 1][j].max(dp[i][j] + j as i64);

            for l in 0..n {
                dp[i + 1][j + items[l].1 as usize] =
                    dp[i + 1][j + items[l].1 as usize].max(dp[i][j] - items[l].0);
            }
        }
    }

    writeln!(out, "{}", dp[k].iter().max().unwrap()).unwrap();
}
