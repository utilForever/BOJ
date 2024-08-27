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

    pub fn line(&mut self) -> String {
        let mut input = String::new();
        self.reader.read_line(&mut input).expect("Failed read");
        input
    }
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let (n, r) = (scan.token::<usize>(), scan.token::<i64>());
    let mut points = vec![(0.0, 0.0, 0.0); n + 1];

    for i in 0..n {
        points[i] = (
            scan.token::<f64>(),
            scan.token::<f64>(),
            scan.token::<f64>(),
        );
    }

    points[n] = points[0];

    let mut ret = (n as f64) * 4.0 * std::f64::consts::PI * (r as f64).powi(3) / 3.0;

    for i in 1..=n {
        let d = ((points[i].0 - points[i - 1].0).powi(2)
            + (points[i].1 - points[i - 1].1).powi(2)
            + (points[i].2 - points[i - 1].2).powi(2))
        .sqrt();
        let v = 2.0 / 3.0
            * std::f64::consts::PI
            * (r as f64 - d / 2.0).powi(2)
            * (2.0 * r as f64 + d / 2.0);

        ret -= v;
    }

    writeln!(out, "{:.10}", ret).unwrap();
}
