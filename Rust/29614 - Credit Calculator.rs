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

    let s = scan.token::<String>();
    let s = s.chars().collect::<Vec<_>>();
    let mut idx = 0;
    let mut cnt = 0;
    let mut ret = 0.0;

    while idx < s.len() {
        let is_plus = idx + 1 < s.len() && s[idx + 1] == '+';

        ret += match s[idx] {
            'A' => {
                if is_plus {
                    4.5
                } else {
                    4.0
                }
            }
            'B' => {
                if is_plus {
                    3.5
                } else {
                    3.0
                }
            }
            'C' => {
                if is_plus {
                    2.5
                } else {
                    2.0
                }
            }
            'D' => {
                if is_plus {
                    1.5
                } else {
                    1.0
                }
            }
            'F' => 0.0,
            _ => unreachable!(),
        };

        idx += if is_plus { 2 } else { 1 };
        cnt += 1;
    }

    writeln!(out, "{:.4}", ret / cnt as f64).unwrap();
}
