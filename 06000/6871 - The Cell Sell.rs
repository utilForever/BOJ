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

    let (daytime, evening, weekend) = (
        scan.token::<i64>(),
        scan.token::<i64>(),
        scan.token::<i64>(),
    );
    let cost_a = (daytime - 100).max(0) * 25 + evening * 15 + weekend * 20;
    let cost_b = (daytime - 250).max(0) * 45 + evening * 35 + weekend * 25;

    writeln!(out, "Plan A costs {:.2}", cost_a as f64 / 100.0).unwrap();
    writeln!(out, "Plan B costs {:.2}", cost_b as f64 / 100.0).unwrap();

    if cost_a < cost_b {
        writeln!(out, "Plan A is cheapest.").unwrap();
    } else if cost_a > cost_b {
        writeln!(out, "Plan B is cheapest.").unwrap();
    } else {
        writeln!(out, "Plan A and B are the same price.").unwrap();
    }
}
