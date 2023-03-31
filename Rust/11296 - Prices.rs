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

    let n = scan.token::<i64>();

    for _ in 0..n {
        let (mut price, dots, coupon, payment) = (
            scan.token::<f64>(),
            scan.token::<String>(),
            scan.token::<String>(),
            scan.token::<String>(),
        );

        price *= 1000.0;
        price *= match dots.as_str() {
            "R" => 0.55,
            "G" => 0.7,
            "B" => 0.8,
            "Y" => 0.85,
            "O" => 0.9,
            "W" => 0.95,
            _ => unreachable!(),
        };

        if coupon == "C" {
            price *= 0.95;
        }

        if payment == "C" {
            if price as i64 % 100 < 60 {
                price = (price as i64 / 100) as f64 / 10.0;
            } else {
                price = (price as i64 / 100 + 1) as f64 / 10.0;
            }
        } else {
            price /= 1000.0;
        }

        writeln!(out, "${:.2}", price).unwrap();
    }
}
