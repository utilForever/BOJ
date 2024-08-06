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

    let (n, r2) = (scan.token::<usize>(), scan.token::<i64>());
    let mut values = vec![(0.0, 0.0); n];

    for i in 0..n {
        let (x, y) = (scan.token::<i64>(), scan.token::<i64>());
        let sqrt_x = if x >= 0 {
            (x as f64).sqrt()
        } else {
            -((-x as f64).sqrt())
        };
        let sqrt_y = if y >= 0 {
            (y as f64).sqrt()
        } else {
            -((-y as f64).sqrt())
        };

        values[i] = (sqrt_x, sqrt_y);
    }

    let mut ret = 0.0;

    for i in 0..n {
        for j in 0..i {
            let (x1, y1) = values[i];
            let (x2, y2) = values[j];

            ret += ((x1 - x2).powi(2) + (y1 - y2).powi(2)).ln() / 2.0;
        }
    }

    writeln!(
        out,
        "{:.4}",
        (n - 2) as f64 * ret - ((n * (n - 1) * (n - 2)) / 6) as f64 * (2.0 * r2 as f64).ln()
    )
    .unwrap();
}
