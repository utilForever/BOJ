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

    let (n, m) = (scan.token::<i64>(), scan.token::<i64>());
    let (a, mut d) = (scan.token::<i64>(), scan.token::<i64>());
    let (s_r, s_c) = (scan.token::<i64>(), scan.token::<i64>());

    let mut pos = (1, a);
    let mut ret = true;

    while pos.0 != n || pos.1 != m {
        if d == 0 {
            if pos.1 > 1 {
                pos.1 -= 1;
            }

            if pos == (s_r, s_c) {
                ret = false;
                break;
            }

            if pos.1 == 1 {
                pos.0 += 1;
                d = 1;

                if pos == (s_r, s_c) {
                    ret = false;
                    break;
                }
            }
        } else {
            if pos.1 < m {
                pos.1 += 1;
            }

            if pos == (s_r, s_c) {
                ret = false;
                break;
            }

            if pos.1 == m {
                pos.0 += 1;
                d = 0;

                if pos == (s_r, s_c) {
                    ret = false;
                    break;
                }
            }
        }
    }

    writeln!(out, "{}", if ret { "YES!" } else { "NO..." }).unwrap();
}
