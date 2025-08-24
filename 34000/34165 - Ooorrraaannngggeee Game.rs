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

    let s = scan.token::<String>().chars().collect::<Vec<_>>();
    let t = " orange".chars().collect::<Vec<_>>();
    let n = s.len();

    let mut dp = vec![vec![0; 7]; n + 1];

    for i in 1..=6 {
        dp[0][i] = i64::MIN;
    }

    for i in 1..=n {
        for j in 1..=6 {
            dp[i][j] = if s[i - 1] == t[j] {
                dp[i - 1][j - 1].max(dp[i - 1][j]) + 1
            } else {
                dp[i - 1][j]
            };
        }
    }

    if dp[n][6] < 1 {
        writeln!(out, "0").unwrap();
        return;
    }

    writeln!(out, "{}", dp[n][6]).unwrap();

    let mut ret = vec![' '; dp[n][6] as usize];
    let mut pos = dp[n][6] as usize;
    let mut idx = 6;

    for i in (0..n).rev() {
        if s[i] == t[idx] {
            pos -= 1;
            ret[pos] = t[idx];

            if idx > 0 && dp[i][idx - 1] > dp[i][idx] {
                idx -= 1;
            }

            if pos == 0 {
                break;
            }
        }
    }

    writeln!(out, "{}", ret.iter().collect::<String>()).unwrap();
}
