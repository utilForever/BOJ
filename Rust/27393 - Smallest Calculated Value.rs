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

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let (a, b, c) = (
        scan.token::<i64>(),
        scan.token::<i64>(),
        scan.token::<i64>(),
    );
    let mut ret = i64::MAX;

    for i in 0..4 {
        for j in 0..4 {
            let val = match i {
                0 => match j {
                    0 => (a + b) + c,
                    1 => (a + b) - c,
                    2 => (a + b) * c,
                    3 => {
                        if (a + b) % c == 0 {
                            (a + b) / c
                        } else {
                            i64::MAX
                        }
                    }
                    _ => unreachable!(),
                },
                1 => match j {
                    0 => (a - b) + c,
                    1 => (a - b) - c,
                    2 => (a - b) * c,
                    3 => {
                        if (a - b) % c == 0 {
                            (a - b) / c
                        } else {
                            i64::MAX
                        }
                    }
                    _ => unreachable!(),
                },
                2 => match j {
                    0 => (a * b) + c,
                    1 => (a * b) - c,
                    2 => (a * b) * c,
                    3 => {
                        if (a * b) % c == 0 {
                            (a * b) / c
                        } else {
                            i64::MAX
                        }
                    }
                    _ => unreachable!(),
                },
                3 => {
                    if a % b != 0 {
                        i64::MAX
                    } else {
                        match j {
                            0 => (a / b) + c,
                            1 => (a / b) - c,
                            2 => (a / b) * c,
                            3 => {
                                if (a / b) % c == 0 {
                                    (a / b) / c
                                } else {
                                    i64::MAX
                                }
                            }
                            _ => unreachable!(),
                        }
                    }
                }
                _ => unreachable!(),
            };

            if val >= 0 {
                ret = ret.min(val);
            }
        }
    }

    writeln!(out, "{ret}").unwrap();
}
