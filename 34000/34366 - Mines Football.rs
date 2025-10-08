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

    let n = scan.token::<i64>();
    let mut point_max = 0;
    let mut point_min = i64::MAX;
    let mut sum_max = 0;
    let mut sum_min = i64::MAX;

    for _ in 0..n {
        let m = scan.token::<i64>();
        let mut sum = 0;

        for _ in 0..m {
            let point = scan.token::<i64>();

            sum += point;
            point_max = point_max.max(point);
            point_min = point_min.min(point);
        }

        sum_max = sum_max.max(sum);
        sum_min = sum_min.min(sum);
    }

    writeln!(out, "{point_max}").unwrap();
    writeln!(out, "{point_min}").unwrap();
    writeln!(out, "{sum_max}").unwrap();
    writeln!(out, "{sum_min}").unwrap();
}
