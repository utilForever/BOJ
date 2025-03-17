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
    let mut coordinates = vec![(0.0, 0.0); n];

    for i in 0..n {
        coordinates[i] = (scan.token::<f64>(), scan.token::<f64>());
    }

    let m = scan.token::<i64>();

    for _ in 0..m {
        let p = scan.token::<usize>();
        let mut points = vec![0; p];

        for i in 0..p {
            points[i] = scan.token::<usize>();
        }

        let mut ret = 0.0;

        for i in 0..p - 1 {
            let (x1, y1) = coordinates[points[i]];
            let (x2, y2) = coordinates[points[i + 1]];

            ret += ((x1 - x2).powi(2) + (y1 - y2).powi(2)).sqrt();
        }

        writeln!(out, "{}", ret.round() as i64).unwrap();
    }
}
