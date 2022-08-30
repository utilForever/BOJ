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

    let (c, d, mut m) = (
        scan.token::<usize>(),
        scan.token::<usize>(),
        scan.token::<i64>(),
    );
    let mut cost = vec![vec![0; d]; c];

    for i in 0..c {
        for j in 0..d {
            cost[i][j] = scan.token::<i64>();
        }
    }

    for i in 1..d {
        let mut ret = vec![0; 500_001];

        for j in 0..c {
            for k in 1..=m {
                if k - cost[j][i - 1] < 0 {
                    continue;
                }

                ret[k as usize] = ret[k as usize]
                    .max(ret[(k - cost[j][i - 1]) as usize] + cost[j][i] - cost[j][i - 1]);
            }
        }

        m += ret[m as usize];
    }

    writeln!(out, "{m}").unwrap();
}
