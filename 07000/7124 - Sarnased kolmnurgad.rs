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

const EPS: f64 = 1e-9;

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let mut triangle1 = [(0, 0); 3];
    let mut triangle2 = [(0, 0); 3];

    for i in 0..3 {
        triangle1[i] = (scan.token::<i64>(), scan.token::<i64>());
    }

    for i in 0..3 {
        triangle2[i] = (scan.token::<i64>(), scan.token::<i64>());
    }

    let mut lengths1 = [0; 3];
    let mut lengths2 = [0; 3];

    for i in 0..3 {
        let (x1, y1) = triangle1[i];
        let (x2, y2) = triangle1[(i + 1) % 3];

        lengths1[i] = (x2 - x1).pow(2) + (y2 - y1).pow(2);

        let (x1, y1) = triangle2[i];
        let (x2, y2) = triangle2[(i + 1) % 3];

        lengths2[i] = (x2 - x1).pow(2) + (y2 - y1).pow(2);
    }

    lengths1.sort_unstable();
    lengths2.sort_unstable();

    let ratio1 = lengths1[0] as f64 / lengths2[0] as f64;
    let ratio2 = lengths1[1] as f64 / lengths2[1] as f64;
    let ratio3 = lengths1[2] as f64 / lengths2[2] as f64;

    if (ratio1 - ratio2).abs() < EPS && (ratio2 - ratio3).abs() < EPS {
        writeln!(out, "{:.9}", ratio1.sqrt()).unwrap();
    } else {
        writeln!(out, "-1").unwrap();
    }
}
