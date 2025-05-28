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

    let mut stations = [(0, 0); 3];

    for i in 0..3 {
        stations[i] = (scan.token::<i64>(), scan.token::<i64>());
    }

    let q = scan.token::<i64>();

    for _ in 0..q {
        let (x, y) = (scan.token::<i64>(), scan.token::<i64>());
        let mut periods = [0; 3];

        for i in 0..3 {
            periods[i] = scan.token::<i64>();
        }

        let mut ret = i64::MAX;

        for i in 0..3 {
            let mut time = (stations[i].0 - x).abs() + (stations[i].1 - y).abs();
            time += if time % periods[i] != 0 {
                periods[i] - time % periods[i]
            } else {
                0
            };

            ret = ret.min(time);
        }

        writeln!(out, "{ret}").unwrap();
    }
}
