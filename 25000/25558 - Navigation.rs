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

    let n = scan.token::<i64>();
    let (sx, sy, ex, ey) = (
        scan.token::<i64>(),
        scan.token::<i64>(),
        scan.token::<i64>(),
        scan.token::<i64>(),
    );
    let dist_sum = (ex - sx).abs() + (ey - sy).abs();
    let mut ret_dist = i64::MAX;
    let mut ret_idx = 0;

    for i in 1..=n {
        let m = scan.token::<i64>();
        let mut dist = 0;

        let (mut x_old, mut y_old) = (sx, sy);

        for _ in 0..m {
            let (x, y) = (scan.token::<i64>(), scan.token::<i64>());
            dist += (x - x_old).abs() + (y - y_old).abs();

            x_old = x;
            y_old = y;
        }

        dist += (ex - x_old).abs() + (ey - y_old).abs();

        if (ret_dist - dist_sum).abs() > (dist - dist_sum).abs() {
            ret_dist = dist;
            ret_idx = i;
        }
    }

    writeln!(out, "{ret_idx}").unwrap();
}
