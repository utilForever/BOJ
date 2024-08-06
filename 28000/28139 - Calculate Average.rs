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

    let n = scan.token::<usize>();
    let mut positions = vec![(0, 0); n];

    for i in 0..n {
        positions[i] = (scan.token::<i64>(), scan.token::<i64>());
    }

    // The sum of distances between all pairs of points
    let ret = positions
        .iter()
        .enumerate()
        .map(|(i, &p1)| {
            positions
                .iter()
                .enumerate()
                .map(|(j, &p2)| {
                    if i <= j {
                        0.0
                    } else {
                        ((p1.0 - p2.0) as f64).hypot((p1.1 - p2.1) as f64)
                    }
                })
                .sum::<f64>()
        })
        .sum::<f64>();

    writeln!(out, "{:.6}", ret / n as f64 * 2.0).unwrap();
}
