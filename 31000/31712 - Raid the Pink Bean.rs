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

    let (cu, du) = (scan.token::<i64>(), scan.token::<i64>());
    let (cd, dd) = (scan.token::<i64>(), scan.token::<i64>());
    let (cp, dp) = (scan.token::<i64>(), scan.token::<i64>());
    let h = scan.token::<i64>();

    if du + dd + dp >= h {
        writeln!(out, "0").unwrap();
        return;
    }

    let mut remain = h - (du + dd + dp);
    let (mut tu, mut td, mut tp) = (1, 1, 1);
    let mut ret = 0;

    while remain > 0 {
        ret += 1;

        if tu >= cu {
            tu = 0;
            remain -= du;
        }

        if td >= cd {
            td = 0;
            remain -= dd;
        }

        if tp >= cp {
            tp = 0;
            remain -= dp;
        }

        tu += 1;
        td += 1;
        tp += 1;
    }

    writeln!(out, "{ret}").unwrap();
}
