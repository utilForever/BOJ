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

    let n = scan.token::<usize>();
    let mut points = vec![(0, 0); n];

    for i in 0..n {
        points[i] = (i as u64 + 1, scan.token::<i64>());
    }

    let mut slope_max = u64::MIN;

    for i in 0..n - 1 {
        slope_max = slope_max.max(points[i].1.abs_diff(points[i + 1].1));
    }

    points.sort_unstable_by(|a, b| a.1.cmp(&b.1));

    let mut slope_min = u64::MIN;

    for i in 0..n - 1 {
        slope_min = slope_min.max(points[i].0.abs_diff(points[i + 1].0));
    }

    writeln!(out, "{slope_max}/1").unwrap();
    writeln!(out, "1/{slope_min}").unwrap();
}
