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

    pub fn all(&mut self) -> String {
        let mut input = String::new();
        self.reader.read_to_string(&mut input).expect("Failed read");
        input
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

    let (x, y, d, t) = (
        scan.token::<i64>(),
        scan.token::<i64>(),
        scan.token::<i64>(),
        scan.token::<i64>(),
    );
    let dist = ((x * x + y * y) as f64).sqrt();

    // Case 1: Only walk
    let time1 = dist;

    // Case 2: Jump once and walk rest
    let time2 = t as f64 + (dist - d as f64).abs();

    // Case 3: Jump exactly
    let q3 = ((dist / d as f64).ceil() as i64).max(2);
    let time3 = (q3 * t) as f64;

    // Case 4: Jump and walk rest
    let q4 = (dist / d as f64).floor() as i64;
    let time4 = if q4 >= 2 {
        (q4 * t) as f64 + (dist - q4 as f64 * d as f64)
    } else {
        f64::MAX
    };

    writeln!(out, "{:.12}", time1.min(time2).min(time3).min(time4)).unwrap();
}
