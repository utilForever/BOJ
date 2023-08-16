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

static MOD: i64 = 1_000_000_007;

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let s = scan.token::<String>();
    let marks = s.chars().collect::<Vec<_>>();

    let _ = scan.token::<i64>();

    for _ in 0..=220 {
        let _ = scan.token::<i64>();
    }

    let mut dp = vec![vec![vec![vec![0; 2]; 2]; 256]; 10];
    let mut ret = vec![0; 384];

    dp[0][0][0][0] = 1;

    for i in 1..=9 {
        let first = marks[2 * (i - 1)];
        let second = marks[2 * (i - 1) + 1];

        for pin_first in 0..=10 {
            for pin_second in 0..=10 - pin_first {
                let mark_first = if pin_first == 0 {
                    '-'
                } else if pin_first == 10 {
                    'X'
                } else {
                    std::char::from_digit(pin_first as u32, 10).unwrap()
                };

                let mark_second = if pin_first == 10 {
                    '.'
                } else if pin_second == 0 {
                    '-'
                } else if pin_first + pin_second == 10 {
                    '/'
                } else {
                    std::char::from_digit(pin_second as u32, 10).unwrap()
                };

                if first != '?' && first != mark_first {
                    continue;
                }

                if second != '?' && second != mark_second {
                    continue;
                }

                for j in 0..=220 {
                    for k in 0..2 {
                        for l in 0..2 {
                            let score =
                                j + pin_first + pin_second + (k + l) * pin_first + l * pin_second;
                            let is_strike = if pin_first == 10 { 1 } else { 0 };
                            let is_spare = if pin_first + pin_second == 10 && pin_second != 0 {
                                1
                            } else {
                                0
                            };

                            dp[i][score][is_spare][is_strike] += dp[i - 1][j][k][l];
                            dp[i][score][is_spare][is_strike] %= MOD;
                        }
                    }
                }
            }
        }
    }

    let first = marks[18];
    let second = marks[19];
    let third = marks[20];

    for pin_first in 0..=10 {
        for pin_second in 0..=10 {
            for pin_third in 0..=10 {
                if pin_first != 10 && pin_first + pin_second > 10 {
                    continue;
                }

                if pin_first != 10 && pin_first + pin_second != 10 && pin_third != 0 {
                    continue;
                }

                if pin_first == 10 && pin_second != 10 && pin_second + pin_third > 10 {
                    continue;
                }

                let mark_first = if pin_first == 0 {
                    '-'
                } else if pin_first == 10 {
                    'X'
                } else {
                    std::char::from_digit(pin_first as u32, 10).unwrap()
                };

                let mark_second = if pin_second == 0 {
                    '-'
                } else if pin_first + pin_second == 10 {
                    '/'
                } else if pin_second == 10 {
                    'X'
                } else {
                    std::char::from_digit(pin_second as u32, 10).unwrap()
                };

                let mark_third = if pin_first + pin_second < 10 {
                    '.'
                } else if pin_third == 0 {
                    '-'
                } else if pin_second != 10
                    && (pin_first == 10 || pin_first + pin_second != 10)
                    && pin_second + pin_third == 10
                {
                    '/'
                } else if pin_third == 10 {
                    'X'
                } else {
                    std::char::from_digit(pin_third as u32, 10).unwrap()
                };

                if first != mark_first {
                    continue;
                }

                if second != mark_second {
                    continue;
                }

                if third != mark_third {
                    continue;
                }

                for j in 0..=220 {
                    for k in 0..2 {
                        for l in 0..2 {
                            let score = j
                                + pin_first
                                + pin_second
                                + pin_third
                                + (k + l) * pin_first
                                + l * pin_second;

                            ret[score] += dp[9][j][k][l];
                            ret[score] %= MOD;
                        }
                    }
                }
            }
        }
    }

    for i in 0..=220 {
        write!(out, "{} ", ret[i]).unwrap();
    }

    writeln!(out).unwrap();
}
