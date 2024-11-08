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
    let mut curr = (scan.token::<i64>(), scan.token::<i64>());
    let mut points = vec![(0, 0); n - 1];

    for i in 0..n - 1 {
        points[i] = (scan.token::<i64>(), scan.token::<i64>());
    }

    let mut dist_min = i64::MAX;
    let mut idx = 0;

    for i in 0..n - 1 {
        let dist = (curr.0 - points[i].0).pow(2) + (curr.1 - points[i].1).pow(2);

        if dist < dist_min {
            dist_min = dist;
            idx = i;
        }
    }

    writeln!(out, "{} {}", curr.0, curr.1).unwrap();
    writeln!(out, "{} {}", points[idx].0, points[idx].1).unwrap();
    writeln!(out, "{:.2}", (dist_min as f64).sqrt()).unwrap();
}
