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

    let (x, y, w, s) = (
        scan.token::<i64>(),
        scan.token::<i64>(),
        scan.token::<i64>(),
        scan.token::<i64>(),
    );

    if 2 * w < s {
        writeln!(out, "{}", (x + y) * w).unwrap();
    } else {
        let len_diagonal = x.min(y);
        let mut len_straight = x.max(y) - len_diagonal;
        let mut ret = len_diagonal * s;

        if len_straight >= 2 && w >= s {
            ret += (len_straight / 2) * 2 * s;
            len_straight %= 2;
        }

        ret += len_straight * w;

        writeln!(out, "{ret}").unwrap();
    }
}
