use io::Write;
use std::{cmp, io, str};

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
    let (a, b, c) = (
        scan.token::<usize>(),
        scan.token::<usize>(),
        scan.token::<usize>(),
    );
    let mut suitabilities = vec![(0, 0, 0); n];
    let mut max_suitabilities = vec![vec![vec![-10000; n + 1]; n + 1]; 2];

    for i in 0..n {
        suitabilities[i] = (
            scan.token::<i32>(),
            scan.token::<i32>(),
            scan.token::<i32>(),
        );
    }

    suitabilities.sort_by(|a, b| a.0.cmp(&b.0).reverse());

    max_suitabilities[0][0][0] = 0;

    for l in 0..n {
        for i in (0..=1).rev() {
            for j in (0..=b).rev() {
                for k in (0..=c).rev() {
                    if (l as i32) < (a + j + k) as i32 {
                        max_suitabilities[1][j][k] = cmp::max(
                            max_suitabilities[1][j][k],
                            max_suitabilities[i][j][k] + suitabilities[l].0,
                        );
                    }

                    if j > 0 {
                        max_suitabilities[i][j][k] = cmp::max(
                            max_suitabilities[i][j][k],
                            max_suitabilities[i][j - 1][k] + suitabilities[l].1,
                        );
                    }

                    if k > 0 {
                        max_suitabilities[i][j][k] = cmp::max(
                            max_suitabilities[i][j][k],
                            max_suitabilities[i][j][k - 1] + suitabilities[l].2,
                        );
                    }
                }
            }
        }
    }

    let mut ans = i32::MIN;

    for i in 1..=b {
        for j in 1..=c {
            ans = cmp::max(ans, max_suitabilities[1][i][j]);
        }
    }

    writeln!(out, "{}", ans).unwrap();
}
