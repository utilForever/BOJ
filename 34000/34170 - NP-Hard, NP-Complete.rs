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

const MAX_BITS: usize = 60;

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let t = scan.token::<i64>();

    for _ in 0..t {
        let (n, p, k) = (
            scan.token::<i64>(),
            scan.token::<i64>(),
            scan.token::<usize>(),
        );

        let mut temp = n;
        let mut step = 0;

        while step < k && temp > 0 {
            temp /= p;
            step += 1;
        }

        if temp == 0 {
            writeln!(out, "0").unwrap();
            continue;
        }

        let mut p_pow = vec![0; MAX_BITS + 1];
        let mut level = 0;

        p_pow[0] = 1;

        loop {
            if level > 0 {
                p_pow[level] = p_pow[level - 1] * p;
            }

            if n / p < p_pow[level] {
                break;
            }

            level += 1;
        }

        let mut seg_max = vec![[-1; 2]; level + 1];
        seg_max[level][0] = n;

        for i in (1..=level).rev() {
            let val = seg_max[i][0] % p_pow[i];

            seg_max[i - 1][0] = val;
            seg_max[i - 1][1] = seg_max[i - 1][0] + p_pow[i];

            if seg_max[i - 1][1] > seg_max[i][0] && seg_max[i - 1][1] > seg_max[i][1] {
                seg_max[i][1] = -1;
            }
        }

        let mut dp = vec![[vec![0; level + 1], vec![0; level + 1]]; level + 1];
        dp[level][0][0] = 1;

        for i in (1..=level).rev() {
            for a in 0..2 {
                let max_upper = seg_max[i][a];

                if max_upper < 0 {
                    continue;
                }

                for b in 0..2 {
                    let max_lower = seg_max[i - 1][b];

                    if max_lower < 0 || max_upper < max_lower {
                        continue;
                    }

                    let mut choices = (max_upper - max_lower) / p_pow[i];
                    choices = (choices + 1).min(p * 2 - 1 - choices);

                    let carry_inc = if max_lower >= p_pow[i] { 1 } else { 0 };

                    for c in 0..level {
                        if c + carry_inc <= level {
                            let add = choices * dp[i][a][c];
                            dp[i - 1][b][c + carry_inc] += add;
                        }
                    }
                }
            }
        }

        let mut ret = 0;

        if k <= level {
            let val0 = (seg_max[0][0] + 1).min(p * 2 - 1 - seg_max[0][0]);
            let val1 = (seg_max[0][1] + 1).min(p * 2 - 1 - seg_max[0][1]);

            for i in k..=level {
                ret += dp[0][0][i] * val0 + dp[0][1][i] * val1;
            }
        }

        writeln!(out, "{ret}").unwrap();
    }
}
