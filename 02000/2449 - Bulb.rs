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

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let (n, k) = (scan.token::<usize>(), scan.token::<usize>());
    let mut bulbs = vec![0; n];

    for i in 0..n {
        bulbs[i] = scan.token::<usize>();
    }

    let mut bulbs_compressed = Vec::new();

    for &bulb in bulbs.iter() {
        if bulbs_compressed.last() != Some(&bulb) {
            bulbs_compressed.push(bulb);
        }
    }

    if bulbs_compressed.len() == 1 {
        writeln!(out, "0").unwrap();
        return;
    }

    let m = bulbs_compressed.len();
    let mut dp = vec![vec![vec![i64::MAX; k + 1]; m]; m];

    for i in 0..m {
        for c in 1..=k {
            dp[i][i][c] = if bulbs_compressed[i] == c { 0 } else { 1 };
        }
    }

    for len in 2..=m {
        for l in 0..=m - len {
            let r = l + len - 1;

            for c in 1..=k {
                let mut val = i64::MAX;

                for i in l..r {
                    val = val.min(dp[l][i][c] + dp[i + 1][r][c]);
                }

                dp[l][r][c] = val;
            }

            let mut dp_min = i64::MAX;

            for i in 1..=k {
                dp_min = dp_min.min(dp[l][r][i]);
            }

            for c in 1..=k {
                dp[l][r][c] = dp[l][r][c].min(dp_min + 1);
            }
        }
    }

    let mut ret = i64::MAX;

    for i in 1..=k {
        ret = ret.min(dp[0][m - 1][i]);
    }

    writeln!(out, "{ret}").unwrap();
}
