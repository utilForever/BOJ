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

    let (mut a, mut b) = (Vec::new(), Vec::new());
    let q = scan.token::<usize>();
    let mut dp = vec![vec![0; q + 1]; q + 1];
    let mut ret = 0;

    for _ in 0..q {
        let cmd = scan.token::<i64>();

        match cmd {
            1 => {
                let (c, x) = (scan.token::<char>(), scan.token::<i64>());

                if a.last().map_or(false, |&(ch, _)| c == ch) {
                    a.last_mut().unwrap().1 += x;
                } else {
                    a.push((c, x));
                }

                let (i, (ch_a, cnt_a)) = (a.len(), a[a.len() - 1]);

                for j in 1..=b.len() {
                    let (ch_b, cnt_b) = b[j - 1];

                    dp[i][j] = if ch_a != ch_b {
                        0
                    } else if cnt_a == cnt_b {
                        cnt_a + dp[i - 1][j - 1]
                    } else {
                        cnt_a.min(cnt_b)
                    };

                    if ch_a == ch_b {
                        ret = ret.max(cnt_a.min(cnt_b) + dp[i - 1][j - 1]);
                    }
                }
            }
            2 => {
                let (c, x) = (scan.token::<char>(), scan.token::<i64>());

                if b.last().map_or(false, |&(ch, _)| c == ch) {
                    b.last_mut().unwrap().1 += x;
                } else {
                    b.push((c, x));
                }

                let (j, (ch_b, cnt_b)) = (b.len(), b[b.len() - 1]);

                for i in 1..=a.len() {
                    let (ch_a, cnt_a) = a[i - 1];

                    dp[i][j] = if ch_a != ch_b {
                        0
                    } else if cnt_a == cnt_b {
                        cnt_b + dp[i - 1][j - 1]
                    } else {
                        cnt_a.min(cnt_b)
                    };

                    if ch_a == ch_b {
                        ret = ret.max(cnt_b.min(cnt_a) + dp[i - 1][j - 1]);
                    }
                }
            }
            3 => {
                writeln!(out, "{ret}").unwrap();
            }
            _ => unreachable!(),
        }
    }
}
