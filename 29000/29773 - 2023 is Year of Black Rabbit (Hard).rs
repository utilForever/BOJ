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

static YEAR: [char; 4] = ['2', '0', '2', '3'];

fn calculate(
    dp: &mut Vec<Vec<Vec<i64>>>,
    n: &Vec<char>,
    flag_contain: usize,
    idx_year: usize,
    digit_n: usize,
) -> i64 {
    if digit_n == n.len() {
        return if idx_year == 4 { 1 } else { 0 };
    }

    if dp[flag_contain][idx_year][digit_n] >= 0 {
        return dp[flag_contain][idx_year][digit_n];
    }

    dp[flag_contain][idx_year][digit_n] = 0;

    for i in 0..=9 {
        if flag_contain == 0 && n[digit_n] as u8 - b'0' < i {
            continue;
        }

        dp[flag_contain][idx_year][digit_n] += calculate(
            dp,
            n,
            if flag_contain == 1 || n[digit_n] as u8 - b'0' > i {
                1
            } else {
                0
            },
            idx_year
                + if idx_year < 4 && YEAR[idx_year] as u8 - b'0' == i {
                    1
                } else {
                    0
                },
            digit_n + 1,
        );
    }

    dp[flag_contain][idx_year][digit_n]
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let n = scan.token::<String>();
    let n = n.chars().collect::<Vec<_>>();
    let mut dp = vec![vec![vec![-1; 20]; 5]; 2];

    writeln!(out, "{}", calculate(&mut dp, &n, 0, 0, 0)).unwrap();
}
