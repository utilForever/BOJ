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

    let (n, m) = (scan.token::<usize>(), scan.token::<usize>());
    let mut items = Vec::new();

    for _ in 0..n {
        let (v, c, mut k) = (
            scan.token::<usize>(),
            scan.token::<usize>(),
            scan.token::<i64>(),
        );

        // This is a key idea of the solution
        // We can always split the items into 2^x items such as bit representation of k
        // For example, if k = 13, we can split it into 1 + 2 + 4 + 6 items
        // We can select 1 ~ 13 items by selecting 1, 2, 4, 6 items
        let mut idx = 1;

        while k > 0 {
            let cnt = idx.min(k as usize);

            items.push((v * cnt, c * cnt));

            k -= cnt as i64;
            idx *= 2;
        }
    }

    let n = items.len();
    let mut ret = vec![vec![0; m + 1]; n + 1];

    for i in 0..n {
        for j in 0..=m {
            ret[i][j] = if i == 0 {
                if items[i].0 <= j {
                    items[i].1
                } else {
                    ret[i][j]
                }
            } else {
                if items[i].0 <= j {
                    ret[i - 1][j].max(ret[i - 1][j - items[i].0] + items[i].1)
                } else {
                    ret[i - 1][j]
                }
            }
        }
    }

    writeln!(out, "{}", ret[n - 1][m]).unwrap();
}
