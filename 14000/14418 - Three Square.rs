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

    let mut rectangles = vec![(0, 0); 3];

    for i in 0..3 {
        let (length, height) = (scan.token::<i64>(), scan.token::<i64>());
        rectangles[i] = (length.max(height), length.min(height));
    }

    rectangles.sort_by(|a, b| b.0.cmp(&a.0));

    let cond1 = rectangles[0].0 == rectangles[1].0
        && rectangles[1].0 == rectangles[2].0
        && rectangles[0].1 + rectangles[1].1 + rectangles[2].1 == rectangles[0].0;
    let cond2 = rectangles[0].1 == rectangles[1].1
        && rectangles[1].1 == rectangles[2].1
        && rectangles[0].0 + rectangles[1].0 + rectangles[2].0 == rectangles[0].1;
    let cond3 = rectangles[1].0 + rectangles[2].0 == rectangles[0].0
        && rectangles[1].1 == rectangles[2].1
        && rectangles[1].1 + rectangles[0].1 == rectangles[0].0;
    let cond4 = rectangles[1].0 + rectangles[2].1 == rectangles[0].0
        && rectangles[1].1 == rectangles[2].0
        && rectangles[1].1 + rectangles[0].1 == rectangles[0].0;
    let cond5 = rectangles[1].1 + rectangles[2].0 == rectangles[0].0
        && rectangles[1].0 == rectangles[2].1
        && rectangles[1].0 + rectangles[0].1 == rectangles[0].0;
    let cond6 = rectangles[1].1 + rectangles[2].1 == rectangles[0].0
        && rectangles[1].0 == rectangles[2].0
        && rectangles[1].0 + rectangles[0].1 == rectangles[0].0;
    let ret = cond1 || cond2 || cond3 || cond4 || cond5 || cond6;

    writeln!(out, "{}", if ret { "YES" } else { "NO" }).unwrap();
}
