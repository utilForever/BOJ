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

    let (n, m) = (scan.token::<usize>(), scan.token::<usize>());
    let mut nums_n = vec![0; n];
    let mut nums_m = vec![0; m];

    for i in 0..n {
        nums_n[i] = scan.token::<i64>();
    }

    for i in 0..m {
        nums_m[i] = scan.token::<i64>();
    }

    let mut ret = vec![0; n + m];
    let mut idx = 0;
    let mut idx_n = 0;
    let mut idx_m = 0;

    while idx_n < n && idx_m < m {
        if nums_n[idx_n] < nums_m[idx_m] {
            ret[idx] = nums_n[idx_n];

            idx += 1;
            idx_n += 1;
        } else if nums_n[idx_n] > nums_m[idx_m] {
            ret[idx] = nums_m[idx_m];

            idx += 1;
            idx_m += 1;
        } else {
            ret[idx] = nums_n[idx_n];
            ret[idx + 1] = nums_m[idx_m];

            idx += 2;
            idx_n += 1;
            idx_m += 1;
        }
    }

    while idx_n < n {
        ret[idx] = nums_n[idx_n];

        idx += 1;
        idx_n += 1;
    }

    while idx_m < m {
        ret[idx] = nums_m[idx_m];

        idx += 1;
        idx_m += 1;
    }

    for num in ret {
        write!(out, "{} ", num).unwrap();
    }

    writeln!(out).unwrap();
}
