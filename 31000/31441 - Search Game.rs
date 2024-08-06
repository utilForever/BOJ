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
    let mut nums = vec![0; n + 1];
    let mut prefix_sum = vec![0; n + 1];

    for i in 1..=n {
        nums[i] = scan.token::<i64>();
        prefix_sum[i] = prefix_sum[i - 1] + nums[i];
    }

    let mut choice = vec![vec![vec![0; k + 1]; n.max(k) + 1]; n.max(k) + 2];
    let mut expectation = vec![vec![vec![0; k + 1]; n.max(k) + 1]; n.max(k) + 2];

    let get_val =
        |expectation: &Vec<Vec<Vec<i64>>>, a: usize, b: usize, c: usize, k: usize| -> i64 {
            if c == 0 {
                expectation[a][b][k] * (k + 1) as i64
            } else {
                expectation[a][b][c]
            }
        };

    for i in 1..=n {
        for j in 1..=k {
            choice[i][i][j] = i;
            expectation[i][i][j] = nums[i];
        }
    }

    for l in 1..n {
        for i in 1..=n - l {
            for j in 1..=k {
                if j > l + 1 {
                    choice[i][i + l][j] = i;
                    expectation[i][i + l][j] = prefix_sum[i + l] - prefix_sum[i - 1];
                } else {
                    let mut choice_min = 0;
                    let mut val_min = i64::MAX;

                    for m in choice[i][i + l - 1][j]..=choice[i + 1][i + l][j] {
                        let val = get_val(&expectation, i, m - 1, 0, k)
                            + get_val(&expectation, m + 1, i + l, j - 1, k)
                            + nums[m];

                        if val < val_min {
                            val_min = val;
                            choice_min = m;
                        }
                    }

                    choice[i][i + l][j] = choice_min;
                    expectation[i][i + l][j] = val_min;
                }
            }
        }
    }

    writeln!(out, "{}", expectation[1][n][k]).unwrap();
}
