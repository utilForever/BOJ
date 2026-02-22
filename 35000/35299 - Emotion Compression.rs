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

const LEN_MAX: usize = 10;

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let s = scan.token::<String>().chars().collect::<Vec<_>>();
    let cnt_reverse = scan.token::<usize>();
    let n = s.len() / 2;

    let mut emotions = vec![0; n];
    let mut cnt_happy = 0;

    for i in 0..n {
        emotions[i] = if s[2 * i + 1] == ')' { 1 } else { 0 };
        cnt_happy += emotions[i];
    }

    let idx = |i: usize, j: usize, k: usize, l: usize| -> usize {
        ((i * (cnt_reverse + 1) + j) * 2 + k) * LEN_MAX + (l - 1)
    };

    let size = (cnt_happy + 1) * (cnt_reverse + 1) * 2 * LEN_MAX;
    let mut dp = vec![i64::MAX / 4; size];

    for i in 0..=1 {
        if i > cnt_happy {
            continue;
        }

        let cnt_diff = if i != emotions[0] { 1 } else { 0 };

        if cnt_diff > cnt_reverse {
            continue;
        }

        if i + n - 1 < cnt_happy {
            continue;
        }

        dp[idx(i, cnt_diff, i, 1)] = 2;
    }

    for pos in 1..n {
        let mut dp_new = vec![i64::MAX / 4; size];

        for i in 0..=cnt_happy {
            for j in 0..=cnt_reverse {
                for k in 0..=1 {
                    for l in 1..=LEN_MAX {
                        let curr = dp[idx(i, j, k, l)];

                        if curr >= i64::MAX / 4 {
                            continue;
                        }

                        for m in 0..=1 {
                            let i2 = i + m;

                            if i2 > cnt_happy {
                                continue;
                            }

                            let j2 = j + if m != emotions[pos] { 1 } else { 0 };

                            if j2 > cnt_reverse {
                                continue;
                            }

                            if i2 + n - 1 - pos < cnt_happy {
                                continue;
                            }

                            let (add, l2) = if m == k {
                                let extra = if l == 1 || l == 9 { 1 } else { 0 };
                                let len_new = if l == LEN_MAX { LEN_MAX } else { l + 1 };
                                (extra, len_new)
                            } else {
                                (2, 1)
                            };

                            let next = idx(i2, j2, m, l2);
                            dp_new[next] = dp_new[next].min(curr + add);
                        }
                    }
                }
            }
        }

        dp = dp_new;
    }

    let mut ret = i64::MAX;

    for j in 0..=cnt_reverse {
        for k in 0..=1 {
            for l in 1..=LEN_MAX {
                ret = ret.min(dp[idx(cnt_happy, j, k, l)]);
            }
        }
    }

    writeln!(out, "{ret}").unwrap();
}
