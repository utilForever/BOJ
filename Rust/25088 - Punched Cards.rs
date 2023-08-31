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

    for i in 1..=t {
        let (r, c) = (scan.token::<i64>(), scan.token::<i64>());

        writeln!(out, "Case #{i}:").unwrap();

        for j in 0..r * 2 + 1 {
            for k in 0..c * 2 + 1 {
                if (j == 0 && k == 0) || (j == 0 && k == 1) || (j == 1 && k == 0) {
                    write!(out, ".").unwrap();
                } else if j % 2 == 1 && k % 2 == 1 {
                    write!(out, ".").unwrap();
                } else if j % 2 == 1 && k % 2 == 0 {
                    write!(out, "|").unwrap();
                } else if j % 2 == 0 && k % 2 == 1 {
                    write!(out, "-").unwrap();
                } else if j % 2 == 0 && k % 2 == 0 {
                    write!(out, "+").unwrap();
                }
            }

            writeln!(out).unwrap();
        }
    }
}
