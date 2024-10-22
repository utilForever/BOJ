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

fn combination(n: i64, k: i64, total: i64) -> f64 {
    let mut ret = 1.0;

    for i in 0..k {
        ret *= (n - i) as f64 / (total - i) as f64;
    }

    ret
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let m = scan.token::<usize>();
    let mut stones = vec![0; m];

    for i in 0..m {
        stones[i] = scan.token::<i64>();
    }

    let k = scan.token::<i64>();
    let total = stones.iter().sum::<i64>();
    let mut ret = 0.0;

    for stone in stones {
        if stone < k {
            continue;
        }

        ret += combination(stone, k, total);
    }

    writeln!(out, "{:.9}", ret as f64).unwrap();
}
