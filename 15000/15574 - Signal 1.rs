use io::Write;
use std::{collections::BTreeMap, io, str};

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
    let mut points = BTreeMap::new();

    for _ in 0..n {
        let (x, y) = (scan.token::<i64>(), scan.token::<i64>());
        points.entry(x).or_insert(vec![]).push(y);
    }

    let mut upper = 0.0;
    let mut lower = 0.0;
    let mut x1 = *points.iter().next().unwrap().0;
    let mut y_upper1 = *points.iter().next().unwrap().1.iter().max().unwrap();
    let mut y_lower1 = *points.iter().next().unwrap().1.iter().min().unwrap();

    for (x2, y) in points.iter().skip(1) {
        let y_upper2 = *y.iter().max().unwrap();
        let y_lower2 = *y.iter().min().unwrap();
        let upper_temp = upper;
        let lower_temp = lower;

        upper = (upper_temp + ((x2 - x1) as f64).hypot((y_upper2 - y_upper1) as f64))
            .max(lower_temp + ((x2 - x1) as f64).hypot((y_upper2 - y_lower1) as f64));
        lower = (lower_temp + ((x2 - x1) as f64).hypot((y_lower2 - y_lower1) as f64))
            .max(upper_temp + ((x2 - x1) as f64).hypot((y_lower2 - y_upper1) as f64));

        x1 = *x2;
        y_upper1 = y_upper2;
        y_lower1 = y_lower2;
    }

    writeln!(out, "{:.6}", upper.max(lower)).unwrap();
}
