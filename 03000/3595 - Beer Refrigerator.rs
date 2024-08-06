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
    let mut area = i64::MAX;
    let mut ret = (-1, -1, -1);

    for i in 1..=n {
        if n % i != 0 {
            continue;
        }

        for j in 1..=n / i {
            if n % j != 0 {
                continue;
            }

            let k = n / (i * j);

            if i * j * k == n {
                let val = 2 * i * j + 2 * j * k + 2 * k * i;

                if val < area {
                    area = val;
                    ret = (i, j, k);
                }
            }
        }
    }

    writeln!(out, "{} {} {}", ret.0, ret.1, ret.2).unwrap();
}
