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

    let (n, m, b) = (
        scan.token::<usize>(),
        scan.token::<usize>(),
        scan.token::<i64>(),
    );
    let mut blocks = vec![vec![0; m]; n];

    for i in 0..n {
        for j in 0..m {
            blocks[i][j] = scan.token::<i64>();
        }
    }

    let mut min_time = i64::MAX;
    let mut height = 0;

    for i in 0..=256 {
        let mut req_block = 0;
        let mut req_time = 0;

        for j in 0..n {
            for k in 0..m {
                let diff = i as i64 - blocks[j][k] as i64;

                if diff > 0 {
                    req_block += diff;
                    req_time += diff;
                } else if diff < 0 {
                    req_block -= diff.abs();
                    req_time += 2 * diff.abs();
                }
            }
        }

        if req_block > b {
            continue;
        }

        if req_time < min_time {
            min_time = req_time;
            height = i;
        } else if req_time == min_time && height < i {
            height = i;
        }
    }

    writeln!(out, "{} {}", min_time, height).unwrap();
}
