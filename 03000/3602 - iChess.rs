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

    let (b, w) = (scan.token::<i64>(), scan.token::<i64>());

    if b == 0 && w == 0 {
        writeln!(out, "Impossible").unwrap();
        return;
    }

    let mut ret = 1;

    loop {
        let val = ret * ret / 2;

        if ret % 2 == 0 {
            if b >= val && w >= val {
                ret += 1;
            } else {
                ret -= 1;
                break;
            }
        } else {
            if b >= val + 1 && w >= val {
                ret += 1;
            } else if b >= val && w >= val + 1 {
                ret += 1;
            } else {
                ret -= 1;
                break;
            }
        }
    }

    writeln!(out, "{ret}").unwrap();
}
