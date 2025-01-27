use io::Write;
use std::{collections::VecDeque, io, str};

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

    pub fn line(&mut self) -> String {
        let mut input = String::new();
        self.reader.read_line(&mut input).expect("Failed read");
        input
    }
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let (n, m) = (scan.token::<usize>(), scan.token::<usize>());
    let mut nums = vec![(0, 0); m];

    for i in 0..m {
        nums[i] = (scan.token::<i64>(), scan.token::<usize>());
    }

    nums.sort_by(|a, b| a.0.cmp(&b.0));

    let mut x = vec![0; m];
    let mut c = vec![0; m];

    for i in 0..m {
        x[i] = nums[i].0;
        c[i] = nums[i].1.min(n);
    }

    let mut graph = vec![Vec::new(); m];

    for i in 0..m - 1 {
        for j in i + 1..m {
            if x[j] % x[i] == 0 {
                graph[i].push(j);
            }
        }
    }

    let val_min = i64::MIN / 4;
    let mut dp = vec![vec![val_min; n + 1]; m];
    let mut max = vec![vec![val_min; n + 1]; m];

    for i in 0..m {
        dp[i][0] = 0;
    }

    for i in (0..m).rev() {
        max[i][0] = 0;

        for r in 1..=n {
            max[i][r] = val_min;
        }

        for &next in graph[i].iter() {
            for r in 0..=n {
                max[i][r] = max[i][r].max(dp[next][r]);
            }
        }

        let mut h = vec![val_min; n + 1];

        for r in 0..=n {
            if max[i][r] != val_min {
                h[r] = max[i][r] - x[i] * (r as i64);
            }
        }

        let mut deque = VecDeque::new();

        if h[0] != val_min {
            deque.push_back(0);
        }

        for t in 1..=n {
            let left = if t >= c[i] { t - c[i] } else { 0 };

            while let Some(&front) = deque.front() {
                if front < left {
                    deque.pop_front();
                } else {
                    break;
                }
            }

            if let Some(&r) = deque.front() {
                dp[i][t] = h[r] + x[i] * (t as i64);
            } else {
                dp[i][t] = val_min;
            }

            if h[t] != val_min {
                while let Some(&back) = deque.back() {
                    if h[t] >= h[back] {
                        deque.pop_back();
                    } else {
                        break;
                    }
                }

                deque.push_back(t);
            }
        }
    }

    let mut ret = val_min;

    for i in 0..m {
        ret = ret.max(dp[i][n]);
    }

    if ret < 0 {
        writeln!(out, "-1").unwrap();
        return;
    }

    writeln!(out, "{ret}").unwrap();
}
