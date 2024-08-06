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

    let (mut a, mut b, mut c, d, e) = (
        scan.token::<i64>(),
        scan.token::<i64>(),
        scan.token::<i64>(),
        scan.token::<i64>(),
        scan.token::<i64>(),
    );
    let mut ret = 0;

    ret += e;
    ret += d;
    a -= d;

    while c > 0 {
        ret += 1;

        if b > 0 {
            b -= 1;
            c -= 1;
        } else if a > 0 {
            a -= 2;
            c -= 1;
        } else {
            c -= 1;
        }
    }

    while b > 0 {
        ret += 1;

        if b > 1 && a > 0 {
            b -= 2;
            a -= 1;
        } else if b > 0 && a > 0 {
            b -= 1;
            a -= 3;
        } else {
            b -= 2;
        }
    }

    if a > 0 {
        ret += if a % 5 == 0 { a / 5 } else { a / 5 + 1 };
    }

    writeln!(out, "{ret}").unwrap();
}
