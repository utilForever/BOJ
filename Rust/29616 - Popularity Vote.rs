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

fn gcd(first: i64, second: i64) -> i64 {
    let mut max = first;
    let mut min = second;

    if min == 0 && max == 0 {
        return 0;
    } else if min == 0 {
        return max;
    } else if max == 0 {
        return min;
    }

    if min > max {
        std::mem::swap(&mut min, &mut max);
    }

    loop {
        let res = max % min;

        if res == 0 {
            return min;
        }

        max = min;
        min = res;
    }
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let (n, _) = (scan.token::<usize>(), scan.token::<i64>());
    let mut vote_prev = vec![0; n];
    let mut vote_curr = vec![0; n];

    for i in 0..n {
        vote_prev[i] = scan.token::<i64>();
    }

    for i in 0..n {
        vote_curr[i] = scan.token::<i64>();
    }

    let mut gcd_prev = if vote_prev.is_empty() {
        0
    } else {
        vote_prev[0]
    };
    let mut gcd_curr = if vote_curr.is_empty() {
        0
    } else {
        vote_curr[0]
    };

    for i in 1..n {
        gcd_prev = gcd(gcd_prev, vote_prev[i]);
    }

    for i in 1..n {
        gcd_curr = gcd(gcd_curr, vote_curr[i]);
    }

    let mut ret_prev = 0;
    let mut ret_curr = 0;

    for i in 0..n {
        vote_prev[i] /= gcd_prev;
        vote_curr[i] /= gcd_curr;

        ret_prev += vote_prev[i];
        ret_curr += vote_curr[i];
    }

    let mut multiplier = 1;

    for i in 0..n {
        if vote_curr[i] != 0 {
            multiplier = multiplier.max((vote_prev[i] + vote_curr[i] - 1) / vote_curr[i]);
        }
    }

    writeln!(out, "{ret_prev} {}", ret_curr.max(ret_curr * multiplier)).unwrap();
}
