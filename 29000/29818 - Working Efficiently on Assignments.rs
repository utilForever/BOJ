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

    pub fn all(&mut self) -> String {
        let mut input = String::new();
        self.reader.read_to_string(&mut input).expect("Failed read");
        input
    }

    pub fn line(&mut self) -> String {
        let mut input = String::new();
        self.reader.read_line(&mut input).expect("Failed read");
        input
    }
}

struct Assignment {
    time_required: i64,
    time_left: i64,
    score: i64,
}

impl Assignment {
    fn new(time_required: i64, time_left: i64, score: i64) -> Self {
        Self {
            time_required,
            time_left,
            score,
        }
    }
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let n = scan.token::<usize>();
    let mut assignments = Vec::with_capacity(n);

    for _ in 0..n {
        let (t, d, p) = (
            scan.token::<i64>(),
            scan.token::<i64>(),
            scan.token::<i64>(),
        );
        assignments.push(Assignment::new(t, d, p));
    }

    let mut sum = vec![0; 1usize << n];

    for mask in 1..(1usize << n) {
        let prev = mask & (mask - 1);
        let last = mask ^ prev;
        let idx = last.trailing_zeros() as usize;

        sum[mask] = sum[prev] + assignments[idx].time_required;
    }

    let mut dp = vec![-1; 1usize << n];
    dp[0] = 0;

    for mask in 1..(1usize << n) {
        let mut max = i64::MIN;
        let mut submask = mask;

        while submask > 0 {
            let last = submask & (!submask + 1);
            let idx = last.trailing_zeros() as usize;
            let prev = mask ^ last;

            if dp[prev] == -1 {
                submask &= submask - 1;
                continue;
            }

            let val = dp[prev]
                + if sum[mask] <= assignments[idx].time_left {
                    assignments[idx].score
                } else if sum[mask] <= assignments[idx].time_left + 24 {
                    assignments[idx].score / 2
                } else {
                    0
                };

            max = max.max(val);
            submask &= submask - 1;
        }

        dp[mask] = max;
    }

    let mut score_max = i64::MIN;

    for &score in dp.iter() {
        score_max = score_max.max(score);
    }

    let mut time_min = i64::MAX;

    for mask in 0..(1usize << n) {
        if dp[mask] == score_max {
            time_min = time_min.min(sum[mask]);
        }
    }

    writeln!(out, "{score_max} {time_min}").unwrap();
}
