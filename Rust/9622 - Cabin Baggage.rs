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

    let t = scan.token::<i64>();
    let mut ret = 0;

    for _ in 0..t {
        let (length, width, depth, weight) = (
            scan.token::<f64>(),
            scan.token::<f64>(),
            scan.token::<f64>(),
            scan.token::<f64>(),
        );

        let is_allowed = (length <= 56.0 && width <= 45.0 && depth <= 25.0
            || (length + width + depth) <= 125.0)
            && weight <= 7.0;

        if is_allowed {
            ret += 1;
        }

        writeln!(out, "{}", if is_allowed { "1" } else { "0" }).unwrap();
    }

    writeln!(out, "{ret}").unwrap();
}
