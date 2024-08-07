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

    let d = scan.token::<i64>();

    for i in 1..=d {
        let (h1, m1, s1, u1) = (
            scan.token::<i64>(),
            scan.token::<i64>(),
            scan.token::<i64>(),
            scan.token::<i64>(),
        );
        let (h2, m2, s2, u2) = (
            scan.token::<i64>(),
            scan.token::<i64>(),
            scan.token::<i64>(),
            scan.token::<i64>(),
        );
        let r = scan.token::<f64>();

        let angle1 = h1 as f64 * 30.0
            + m1 as f64 * 0.5
            + s1 as f64 * 0.5 / 60.0
            + u1 as f64 * 0.5 / 60.0 / 100.0;
        let angle2 = h2 as f64 * 30.0
            + m2 as f64 * 0.5
            + s2 as f64 * 0.5 / 60.0
            + u2 as f64 * 0.5 / 60.0 / 100.0;
        let angle_diff = angle2 - angle1;

        writeln!(out, "{i}. {:.3}", r * r * angle_diff.to_radians() / 2.0).unwrap();
    }
}
