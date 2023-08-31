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

// Reference: https://en.wikipedia.org/wiki/Eight_queens_puzzle#Explicit_solutions
fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let t = scan.token::<i64>();

    for _ in 0..t {
        let n = scan.token::<i64>();
        let mut ret = Vec::new();

        if n % 6 != 2 && n % 6 != 3 {
            for i in (2..=n).step_by(2) {
                ret.push(i);
            }

            for i in (1..=n).step_by(2) {
                ret.push(i);
            }
        } else if n % 6 == 2 {
            for i in (2..=n).step_by(2) {
                ret.push(i);
            }

            ret.push(3);
            ret.push(1);

            for i in (7..=n).step_by(2) {
                ret.push(i);
            }

            ret.push(5);
        } else if n % 6 == 3 {
            for i in (4..=n).step_by(2) {
                ret.push(i);
            }

            ret.push(2);

            for i in (5..=n).step_by(2) {
                ret.push(i);
            }

            ret.push(1);
            ret.push(3);
        }

        writeln!(out, "{n}").unwrap();

        for val in ret.iter() {
            write!(out, " {}", val - 1).unwrap();
        }

        writeln!(out).unwrap();
    }
}
