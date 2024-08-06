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

    let n = scan.token::<i64>();
    let mut ret = 0;

    for _ in 0..n {
        let (time, duration) = (scan.token::<String>(), scan.token::<i64>());
        let (h, m) = (time[..2].parse::<i64>().unwrap(), time[3..].parse::<i64>().unwrap());
        let start = h * 60 + m;
        let end = start + duration;

        ret += if end <= 420 {
            duration * 5
        } else if start >= 1140 {
            duration * 5
        } else if start >= 420 && end <= 1140 {
            duration * 10
        } else if start <= 420 && end > 420 {
            (420 - start) * 5 + (end - 420) * 10
        } else {
            (1140 - start) * 10 + (end - 1140) * 5
        }
    }

    writeln!(out, "{ret}").unwrap();
}
