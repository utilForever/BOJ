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

    let (xa, ya) = (scan.token::<i64>(), scan.token::<i64>());
    let (xb, yb) = (scan.token::<i64>(), scan.token::<i64>());
    let dist = (xa - xb).abs() + (ya - yb).abs();

    if dist % 2 == 1 {
        writeln!(out, "-1").unwrap();
    } else {
        let diff_x = (xa - xb).abs();
        let sign_x = if xa < xb {
            1
        } else if xa > xb {
            -1
        } else {
            0
        };
        let sign_y = if ya < yb {
            1
        } else if ya > yb {
            -1
        } else {
            0
        };
        let val_x = diff_x.min(dist / 2);
        let val_y = dist / 2 - val_x;

        writeln!(out, "{} {}", xa + sign_x * val_x, ya + sign_y * val_y).unwrap();
    }
}
