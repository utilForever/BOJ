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

    let k = scan.token::<i64>();
    let val = (k as f64).cbrt() as i64 + 1;
    let mut area_min = i64::MAX;
    let mut ret = (0, 0, 0);

    for a in val - 3000..=val + 3000 {
        if a <= 0 || a > k {
            continue;
        }

        for b in a..=(k as f64 / a as f64).sqrt().ceil() as i64 {
            if b <= 0 || b > k {
                continue;
            }

            let mut c = k / a / b;

            if a * b * c < k {
                c += 1;
            }

            let area = 2 * (a * b + b * c + c * a);

            if area < area_min {
                area_min = area;
                ret = (a, b, c);
            } else if area == area_min {
                ret = ret.min((a, b, c));
            }
        }
    }

    writeln!(out, "{} {} {}", ret.0, ret.1, ret.2).unwrap();
}
