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

    let mut steps = Vec::new();

    loop {
        let step = scan.token::<usize>();

        if step == 0 {
            break;
        }

        steps.push(step);
    }

    let costs = [
        [1, 2, 2, 2, 2],
        [3, 1, 3, 4, 3],
        [3, 3, 1, 3, 4],
        [3, 4, 3, 1, 3],
        [3, 3, 4, 3, 1],
    ];
    let mut dp = vec![vec![vec![1_000_000_000; 5]; 5]; steps.len() + 1];
    dp[0][0][0] = 0;

    for i in 1..=steps.len() {
        let step_cur = steps[i - 1];

        for j in 0..5 {
            for k in 0..5 {
                dp[i][j][step_cur] = dp[i][j][step_cur].min(dp[i - 1][j][k] + costs[k][step_cur]);
                dp[i][step_cur][j] = dp[i][step_cur][j].min(dp[i - 1][j][k] + costs[k][step_cur]);
            }
        }
    }

    let mut ret = usize::MAX;

    for i in 0..5 {
        for j in 0..5 {
            ret = ret.min(dp[steps.len()][i][j]);
        }
    }

    writeln!(out, "{ret}").unwrap();
}
