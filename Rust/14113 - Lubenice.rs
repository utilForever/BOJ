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

    let (e, d, mut c, mut b, mut a) = (
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

    ret += c;

    if b < c {
        c -= b;
        b = 0;

        if a > 0 {
            a -= 2 * c;
        }
    } else {
        b -= c;
    }

    if b > 1 && a > 0 {
        let val = b / 2;
        ret += val;
        b -= 2 * val;
        a -= val;    
    }

    if b > 0 && a > 0 {
        let val = (if a % 3 == 0 { a / 3 } else { a / 3 + 1 }).min(b);
        ret += val;
        a -= 3 * val;
        b -= val;    
    }

    if b > 0 {
        ret += if b % 2 == 0 { b / 2 } else { b / 2 + 1 };
    }

    if a > 0 {
        ret += if a % 5 == 0 { a / 5 } else { a / 5 + 1 };
    }

    writeln!(out, "{ret}").unwrap();
}
