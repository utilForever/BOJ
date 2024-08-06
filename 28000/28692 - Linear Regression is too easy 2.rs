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
    let mut points = vec![(0, 0); n];
    let mut sx = 0;
    let mut sy = 0;
    let mut sxx = 0;
    let mut sxy = 0;

    for i in 0..n {
        points[i] = (scan.token::<i64>(), scan.token::<i64>());
        sx += points[i].0;
        sy += points[i].1;
        sxx += points[i].0 * points[i].0;
        sxy += points[i].0 * points[i].1;
    }

    if sx * sx == n as i64 * sxx {
        writeln!(out, "EZPZ").unwrap();
        return;
    }

    let a = (n as i64 * sxy - sx * sy) as f64 / (n as i64 * sxx - sx * sx) as f64;
    let b = (sy as f64 - a * sx as f64) / n as f64;

    writeln!(out, "{:.6} {:.6}", a, b).unwrap();
}
