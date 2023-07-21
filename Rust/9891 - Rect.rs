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
    let mut rectangles = vec![(0, 0); n];

    for i in 0..n {
        let (x1, y1, x2, y2) = (
            scan.token::<i64>(),
            scan.token::<i64>(),
            scan.token::<i64>(),
            scan.token::<i64>(),
        );
        rectangles[i] = (
            (x2 - x1).abs().min((y2 - y1).abs()),
            (x2 - x1).abs().max((y2 - y1).abs()),
        );
    }

    rectangles.sort();

    let mut ret = 0;

    for i in 0..n - 1 {
        for j in i + 1..n {
            if !((rectangles[i].0 > rectangles[j].0 && rectangles[i].1 > rectangles[j].1)
                || (rectangles[i].0 <= rectangles[j].0 && rectangles[i].1 <= rectangles[j].1))
            {
                ret += 1;
            }
        }
    }

    writeln!(out, "{ret}").unwrap();
}
