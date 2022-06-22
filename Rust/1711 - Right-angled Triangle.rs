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

fn get_dist(p1: (i64, i64), p2: (i64, i64)) -> i64 {
    (p1.0 - p2.0).pow(2) + (p1.1 - p2.1).pow(2)
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let n = scan.token::<usize>();
    let mut points = vec![(0, 0); n];

    for i in 0..n {
        points[i] = (scan.token::<i64>(), scan.token::<i64>());
    }

    let mut ret = 0;

    for i in 0..n - 2 {
        for j in i + 1..n - 1 {
            for k in j + 1..n {
                let a = get_dist(points[i], points[j]);
                let b = get_dist(points[j], points[k]);
                let c = get_dist(points[i], points[k]);

                if (a > c && a > b && b + c == a)
                    || (b > c && b > a && a + c == b)
                    || (c > a && c > b && a + b == c)
                {
                    ret += 1;
                }
            }
        }
    }

    writeln!(out, "{}", ret).unwrap();
}
