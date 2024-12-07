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

    let n = scan.token::<usize>();
    let mut costs = vec![vec![0; n]; n];

    for i in 0..n {
        for j in 0..n {
            costs[i][j] = scan.token::<i64>();
        }
    }

    let initial = scan.token::<String>().chars().collect::<Vec<_>>();
    let p = scan.token::<usize>();

    if p == 0 {
        writeln!(out, "0").unwrap();
        return;
    }

    let mut state_init = 0;
    let mut cnt = 0;

    for (i, &c) in initial.iter().enumerate() {
        if c == 'Y' {
            state_init |= 1 << i;
            cnt += 1;
        }
    }

    if cnt >= p {
        writeln!(out, "0").unwrap();
        return;
    }

    let mut dp = vec![i64::MAX; 1 << n];
    dp[state_init] = 0;

    for state in 0..(1 << n) {
        if dp[state] == i64::MAX {
            continue;
        }

        let mut cnt_curr = 0;

        for i in 0..n {
            if (state & (1 << i)) != 0 {
                cnt_curr += 1;
            }
        }

        if cnt_curr >= p {
            continue;
        }

        for i in 0..n {
            if (state & (1 << i)) == 0 {
                continue;
            }

            for j in 0..n {
                if (state & (1 << j)) == 0 {
                    let state_next = state | (1 << j);
                    let cost_new = dp[state] + costs[i][j];

                    dp[state_next] = dp[state_next].min(cost_new);
                }
            }
        }
    }

    let mut ret = i64::MAX;

    for state in 0..(1 << n) {
        let cnt = (0..n).filter(|&i| (state & (1 << i)) != 0).count();

        if cnt >= p {
            ret = ret.min(dp[state]);
        }
    }

    writeln!(out, "{}", if ret == i64::MAX { -1 } else { ret }).unwrap();
}
