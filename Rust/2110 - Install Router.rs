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

fn check_condition(x: &Vec<i64>, num_house: usize, dist: i64, num_router: usize) -> bool {
    let mut ret = 1;
    let mut base = x[0];

    for i in 1..num_house {
        if x[i] - base >= dist {
            ret += 1;
            base = x[i];
        }
    }

    ret >= num_router
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let (n, c) = (scan.token::<usize>(), scan.token::<usize>());
    let mut x = vec![0; n];

    for i in 0..n {
        x[i] = scan.token::<i64>();
    }

    x.sort();

    let mut left = 1;
    let mut right = x[n - 1] - x[0];
    let mut ret = 0;

    while left <= right {
        let mid = (left + right) / 2;

        if check_condition(&x, n, mid, c) {
            ret = ret.max(mid);
            left = mid + 1;
        } else {
            right = mid - 1;
        }
    }

    writeln!(out, "{}", ret).unwrap();
}
