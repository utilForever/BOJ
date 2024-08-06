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

    for _ in 0..3 {
        let (start, end) = (scan.token::<String>(), scan.token::<String>());
        let (start_h, start_m, start_s) = (
            start[0..2].parse::<i64>().unwrap(),
            start[3..5].parse::<i64>().unwrap(),
            start[6..8].parse::<i64>().unwrap(),
        );
        let (end_h, end_m, end_s) = (
            end[0..2].parse::<i64>().unwrap(),
            end[3..5].parse::<i64>().unwrap(),
            end[6..8].parse::<i64>().unwrap(),
        );
        let (mut h, mut m, mut s) = (start_h, start_m, start_s);
        let mut ret = 0;

        loop {
            if (h * 100000 + m * 1000 + s) % 3 == 0 {
                ret += 1;
            }

            if h == end_h && m == end_m && s == end_s {
                break;
            }

            s += 1;

            if s == 60 {
                s = 0;
                m += 1;
            }

            if m == 60 {
                m = 0;
                h += 1;
            }

            if h == 24 {
                h = 0;
            }
        }

        writeln!(out, "{ret}").unwrap();
    }
}
