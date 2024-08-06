use io::Write;
use std::{
    collections::{BTreeMap, BTreeSet},
    io, str,
};

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

const INF: i64 = 2 * 10i64.pow(18);

fn count(c: i64, left: usize, right: usize) -> i64 {
    let len = right as i64 - left as i64;
    let ret = if len > 60 { INF } else { 1i64 << len };

    if ret > INF / c {
        INF
    } else {
        ret * c
    }
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let t = scan.token::<i64>();

    for _ in 0..t {
        let (n, mut k) = (scan.token::<usize>(), scan.token::<i64>());
        let a = scan.token::<String>();
        let mut a = a.chars().collect::<Vec<_>>();
        a.insert(0, '0');

        let mut dp = vec![vec![0; n + 2]; n + 2];
        let mut ret = String::new();
        let mut ranges = BTreeSet::new();

        dp[1][n] = 1;
        ranges.insert((1, n));

        while ret.len() < n {
            let mut candidates = BTreeMap::new();

            for &(left, right) in ranges.iter() {
                let count = count(dp[left][right], left, right);

                ret.push(a[left]);
                candidates
                    .entry(ret.clone())
                    .and_modify(|e| *e = INF.min(*e + count))
                    .or_insert(count);
                ret.pop();

                ret.push(a[right]);
                candidates
                    .entry(ret.clone())
                    .and_modify(|e| *e = INF.min(*e + count))
                    .or_insert(count);
                ret.pop();
            }

            let mut ret_new = String::new();

            for (str, count) in candidates {
                if k <= count {
                    ret_new = str;
                    break;
                } else {
                    k -= count;
                }
            }

            std::mem::swap(&mut ret, &mut ret_new);

            let mut ranges_new = BTreeSet::new();

            for &(left, right) in ranges.iter() {
                if ret.ends_with(a[left]) {
                    ranges_new.insert((left + 1, right));
                    dp[left + 1][right] = (dp[left + 1][right] + dp[left][right]).min(INF);
                }

                if ret.ends_with(a[right]) {
                    ranges_new.insert((left, right - 1));
                    dp[left][right - 1] = (dp[left][right - 1] + dp[left][right]).min(INF);
                }
            }

            std::mem::swap(&mut ranges, &mut ranges_new);
        }

        writeln!(out, "{ret}").unwrap();
    }
}
