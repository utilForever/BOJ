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
        let (n, m) = (scan.token::<usize>(), scan.token::<i64>());
        let mut teams = vec![(0, 0); n];

        for _ in 0..m {
            let (a, b, p, q) = (
                scan.token::<usize>(),
                scan.token::<usize>(),
                scan.token::<i64>(),
                scan.token::<i64>(),
            );

            teams[a - 1].0 += p;
            teams[a - 1].1 += q;
            teams[b - 1].0 += q;
            teams[b - 1].1 += p;
        }

        let mut ret_max = i64::MIN;
        let mut ret_min = i64::MAX;

        for (s, a) in teams {
            let val = (((s * s) as f64) / ((s * s + a * a) as f64) * 1000.0) as i64;
            ret_max = ret_max.max(val);
            ret_min = ret_min.min(val);
        }

        writeln!(out, "{ret_max}").unwrap();
        writeln!(out, "{ret_min}").unwrap();
    }
}
