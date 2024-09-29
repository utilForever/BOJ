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

    pub fn line(&mut self) -> String {
        let mut input = String::new();
        self.reader.read_line(&mut input).expect("Failed read");
        input
    }
}

const MOD: i32 = 1_000_000_007;

// Reference: UCPC 2018 Editorial
// Reference: https://zigui.tistory.com/27
fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let s = scan.token::<String>();
    let len_s = s.len();
    let len_t = scan.token::<usize>();
    let len_total = len_s + len_t;

    let mut parentheses = vec![')'; len_total + 1];

    for (i, c) in s.chars().enumerate() {
        parentheses[i + 1] = c;
    }

    let mut prefix_sum = vec![0; len_total + 1];

    for i in 1..=len_total {
        prefix_sum[i] += prefix_sum[i - 1] + if parentheses[i] == '(' { 1 } else { -1 };
    }

    let mut next = vec![i32::MAX; len_total + 1];
    let mut prev = vec![i32::MAX; len_total + 1];

    for i in 0..=len_total {
        for j in (0..i).rev() {
            if prefix_sum[i] + 1 <= prefix_sum[j] {
                prev[i] = j as i32;
                break;
            }
        }

        for j in i + 1..=len_total {
            if prefix_sum[i] - 2 >= prefix_sum[j] {
                next[i] = j as i32 - 1;
                break;
            }
        }
    }

    let mut idx_start = 0;

    for i in 1..=len_total {
        if prefix_sum[i] < 0 {
            break;
        } else if prefix_sum[i] == 0 {
            idx_start = i;
        }
    }

    let mut dp = vec![vec![0; len_total + 1]; len_t + 1];
    dp[0][idx_start] = 1;

    for i in 0..len_t {
        for j in 0..=len_total {
            if prev[j] != i32::MAX {
                dp[i + 1][prev[j] as usize] = (dp[i + 1][prev[j] as usize] + dp[i][j]) % MOD;
            }

            if next[j] != i32::MAX {
                dp[i + 1][next[j] as usize] = (dp[i + 1][next[j] as usize] + dp[i][j]) % MOD;
            }
        }
    }

    writeln!(out, "{}", dp[len_t][len_s]).unwrap();
}
