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
    let (mut q1, mut q2, mut q3, mut q4, mut axis) = (0, 0, 0, 0, 0);

    for _ in 0..n {
        let (x, y) = (scan.token::<i32>(), scan.token::<i32>());

        if x == 0 || y == 0 {
            axis += 1;
        } else if x > 0 && y > 0 {
            q1 += 1;
        } else if x < 0 && y > 0 {
            q2 += 1;
        } else if x < 0 && y < 0 {
            q3 += 1;
        } else if x > 0 && y < 0 {
            q4 += 1;
        }
    }

    writeln!(out, "Q1: {q1}").unwrap();
    writeln!(out, "Q2: {q2}").unwrap();
    writeln!(out, "Q3: {q3}").unwrap();
    writeln!(out, "Q4: {q4}").unwrap();
    writeln!(out, "AXIS: {axis}").unwrap();
}
