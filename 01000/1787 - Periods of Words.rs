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

fn process_dp(fail: &Vec<i64>, dp: &mut Vec<i64>, pos: i64) -> i64 {
    if pos < 0 || fail[pos as usize] == 0 {
        return i64::MAX;
    }

    let pos = pos as usize;

    if dp[pos] != -1 {
        return dp[pos];
    }

    dp[pos] = fail[pos].min(process_dp(fail, dp, fail[pos] as i64 - 1));
    dp[pos]
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let n = scan.token::<usize>();
    let s = scan.token::<String>();
    let s = s.chars().collect::<Vec<_>>();

    let mut cmp = 0;
    let mut fail = vec![0; n];

    for i in 1..n {
        while cmp > 0 && s[cmp] != s[i] {
            cmp = fail[cmp - 1] as usize;
        }

        if s[cmp] == s[i] {
            cmp += 1;
            fail[i] = cmp as i64;
        }
    }

    let mut dp = vec![-1; n];
    let mut ret = 0;

    for i in 0..n {
        let val = process_dp(&fail, &mut dp, i as i64);

        if val == i64::MAX {
            continue;
        }

        ret += i as i64 - dp[i] + 1;
    }

    writeln!(out, "{ret}").unwrap();
}
