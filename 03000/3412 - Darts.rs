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

    let t = scan.token::<i64>();
    let calculate_score = |x: i64, y: i64| -> i64 {
        let dist = x * x + y * y;

        if dist <= 400 {
            10
        } else if dist <= 1600 {
            9
        } else if dist <= 3600 {
            8
        } else if dist <= 6400 {
            7
        } else if dist <= 10000 {
            6
        } else if dist <= 14400 {
            5
        } else if dist <= 19600 {
            4
        } else if dist <= 25600 {
            3
        } else if dist <= 32400 {
            2
        } else if dist <= 40000 {
            1
        } else {
            0
        }
    };

    for _ in 0..t {
        let n = scan.token::<i64>();
        let mut ret = 0;

        for _ in 0..n {
            let (x, y) = (scan.token::<i64>(), scan.token::<i64>());
            ret += calculate_score(x, y);
        }

        writeln!(out, "{ret}").unwrap();
    }
}
