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

    for _ in 0..t {
        let (from, to) = (scan.token::<i64>(), scan.token::<i64>());
        let (a, b, c) = (
            scan.token::<f64>(),
            scan.token::<f64>(),
            scan.token::<f64>(),
        );

        if from == 1 {
            if to == 2 {
                writeln!(
                    out,
                    "{} {} {}",
                    a.hypot(b),
                    if b.atan2(a) >= 0.0 {
                        b.atan2(a)
                    } else {
                        b.atan2(a) + std::f64::consts::TAU
                    },
                    c
                )
                .unwrap();
            } else {
                writeln!(
                    out,
                    "{} {} {}",
                    a.hypot(b).hypot(c),
                    if a.hypot(b).atan2(c) >= 0.0 {
                        a.hypot(b).atan2(c)
                    } else {
                        a.hypot(b).atan2(c) + std::f64::consts::TAU
                    },
                    if b.atan2(a) >= 0.0 {
                        b.atan2(a)
                    } else {
                        b.atan2(a) + std::f64::consts::TAU
                    }
                )
                .unwrap();
            }
        } else if from == 2 {
            if to == 1 {
                writeln!(out, "{} {} {}", a * b.cos(), a * b.sin(), c).unwrap();
            } else {
                writeln!(
                    out,
                    "{} {} {}",
                    a.hypot(c),
                    if a.atan2(c) >= 0.0 {
                        a.atan2(c)
                    } else {
                        a.atan2(c) + std::f64::consts::TAU
                    },
                    b
                )
                .unwrap();
            }
        } else {
            if to == 1 {
                writeln!(
                    out,
                    "{} {} {}",
                    a * b.sin() * c.cos(),
                    a * b.sin() * c.sin(),
                    a * b.cos()
                )
                .unwrap();
            } else {
                writeln!(out, "{} {} {}", a * b.sin(), c, a * b.cos()).unwrap();
            }
        }
    }
}
