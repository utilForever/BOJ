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

    let n = scan.token::<i64>();
    let mut top_left = (i64::MAX, i64::MAX);
    let mut bottom_right = (i64::MIN, i64::MIN);

    for _ in 0..n {
        let point = scan.token::<String>();
        let point = point.split(",").collect::<Vec<&str>>();
        let (x, y) = (
            point[0].parse::<i64>().unwrap(),
            point[1].parse::<i64>().unwrap(),
        );

        top_left.0 = top_left.0.min(x);
        top_left.1 = top_left.1.min(y);
        bottom_right.0 = bottom_right.0.max(x);
        bottom_right.1 = bottom_right.1.max(y);
    }

    writeln!(out, "{},{}", top_left.0 - 1, top_left.1 - 1).unwrap();
    writeln!(out, "{},{}", bottom_right.0 + 1, bottom_right.1 + 1).unwrap();
}
