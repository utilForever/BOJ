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

    let (a2, a1, a0) = (
        scan.token::<i64>(),
        scan.token::<i64>(),
        scan.token::<i64>(),
    );
    let c = scan.token::<i64>();
    let n0 = scan.token::<i64>();

    // c * g(x) - f(x)
    let (a, b, c) = (c - a2, -a1, -a0);

    match a {
        1.. => {
            writeln!(out, "0").unwrap();
        }
        0 => {
            writeln!(out, "{}", if b <= 0 && b * n0 + c <= 0 { 1 } else { 0 }).unwrap();
        }
        _ => {
            if -2 * a * n0 < b {
                writeln!(
                    out,
                    "{}",
                    if a * b * b - 2 * a * b * b + 4 * a * a * c <= 0 {
                        1
                    } else {
                        0
                    }
                )
                .unwrap();
            } else {
                writeln!(out, "{}", if a * n0 * n0 + b * n0 + c <= 0 { 1 } else { 0 }).unwrap();
            }
        }
    }
}
