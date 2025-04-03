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

    loop {
        let (a, b, c, d) = (
            scan.token::<i64>(),
            scan.token::<i64>(),
            scan.token::<i64>(),
            scan.token::<i64>(),
        );

        if a == -1 && b == -1 && c == -1 && d == -1 {
            break;
        }

        let mut ret = -1;

        if a == -1 {
            // Check Arthmetic
            let diff1 = c - b;
            let diff2 = d - c;

            if diff1 == diff2 {
                ret = b - diff1;
            }

            // Check Geometric
            if c % b == 0 && d % c == 0 {
                let ratio1 = c / b;
                let ratio2 = d / c;

                if ratio1 == ratio2 && b % ratio1 == 0 {
                    ret = b / ratio1;
                }
            }
        } else if b == -1 {
            // Check Arthmetic
            let diff1 = c - a;
            let diff2 = d - c;

            if diff1 == diff2 * 2 {
                ret = a + diff2;
            }

            // Check Geometric
            if c % a == 0 && d % c == 0 {
                let ratio1 = c / a;
                let ratio2 = d / c;

                if ratio1 == ratio2 * ratio2 {
                    ret = a * ratio2;
                }
            }
        } else if c == -1 {
            // Check Arithmetic
            let diff1 = b - a;
            let diff2 = d - b;

            if diff1 * 2 == diff2 {
                ret = b + diff1;
            }

            // Check Geometric
            if b % a == 0 && d % b == 0 {
                let ratio1 = b / a;
                let ratio2 = d / b;

                if ratio1 * ratio1 == ratio2 {
                    ret = b * ratio1;
                }
            }
        } else {
            // Check Arithmetic
            let diff1 = b - a;
            let diff2 = c - b;

            if diff1 == diff2 {
                ret = c + diff1;
            }

            // Check Geometric
            if b % a == 0 && c % b == 0 {
                let ratio1 = b / a;
                let ratio2 = c / b;

                if ratio1 == ratio2 {
                    ret = c * ratio1;
                }
            }
        }

        writeln!(out, "{}", if ret < 1 || ret > 10000 { -1 } else { ret }).unwrap();
    }
}
