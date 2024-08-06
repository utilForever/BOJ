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

static MOD: i64 = 1_000_000_007;

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let (n, m, k) = (
        scan.token::<usize>(),
        scan.token::<i64>(),
        scan.token::<i64>(),
    );
    let mut nums = vec![0; n];
    let mut dp = vec![vec![0; m as usize + 1]; k as usize + 1];

    for i in 0..n {
        nums[i] = scan.token::<i64>();
    }

    for i in 0..n {
        if nums[i] == 0 {
            continue;
        }

        dp[(nums[i] % k) as usize][1] += 1;
    }

    for i in 2..=m {
        for j in 0..k {
            for num in nums.iter() {
                let idx = (j * 10 + num) % k;

                dp[idx as usize][i as usize] += dp[j as usize][(i - 1) as usize];
                dp[idx as usize][i as usize] %= MOD;
            }
        }
    }

    writeln!(out, "{}", dp[0][m as usize]).unwrap();
}
