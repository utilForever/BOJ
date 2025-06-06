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

    let t = scan.token::<i64>();

    for _ in 0..t {
        let (n, s, c) = (
            scan.token::<i128>(),
            scan.token::<i128>(),
            scan.token::<i128>(),
        );
        let mut ret = 0i128;

        if s == c {
            if n % s != 0 {
                writeln!(out, "0").unwrap();
                continue;
            }

            let q = (n / s) as u128 + 1;

            if !q.is_power_of_two() {
                writeln!(out, "0").unwrap();
                continue;
            }

            let k = q.trailing_zeros() as usize;

            if k < 4 {
                writeln!(out, "0").unwrap();
                continue;
            }

            let pattern = "SCSC".chars().collect::<Vec<_>>();
            let alphabet = ['S', 'C'];
            let mut state_next = vec![vec![0; 2]; 5];

            for state in 0..=4 {
                for (idx, &c) in alphabet.iter().enumerate() {
                    if state == 4 {
                        state_next[state][idx] = 4;
                    } else {
                        let mut next = state + 1;

                        while next > 0 {
                            let mut check = true;

                            for i in 0..next {
                                let a = state + 1 - next + i;
                                let b = if a < state { pattern[a] } else { c };

                                if pattern[i] != b {
                                    check = false;
                                    break;
                                }
                            }

                            if check {
                                break;
                            }

                            next -= 1;
                        }

                        state_next[state][idx] = next;
                    }
                }
            }

            let mut dp = vec![vec![0; 5]; k + 1];
            dp[0][0] = 1;

            for i in 0..k {
                for j in 0..=4 {
                    if dp[i][j] == 0 {
                        continue;
                    }

                    for idx in 0..2 {
                        let next = state_next[j][idx];
                        dp[i + 1][next] += dp[i][j];
                    }
                }
            }

            ret = dp[k][4];
        } else {
            let d = s - c;

            for k in 4..=60 {
                let p = (1i128 << k) - 1;
                let m = n - c * p;

                if m % d != 0 {
                    continue;
                }

                let t = m / d;

                if t < 0 || t > p {
                    continue;
                }

                let mut s = String::with_capacity(k);

                for i in 0..k {
                    let j = k - 1 - i;

                    s.push(if (t >> j) & 1 == 1 { 'S' } else { 'C' });
                }

                if s.contains("SCSC") {
                    ret += 1;
                }
            }
        }

        writeln!(out, "{ret}").unwrap();
    }
}
