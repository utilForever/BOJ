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

    let (n, m) = (scan.token::<usize>(), scan.token::<usize>());
    let mut ret = vec![vec![0; m]; n];

    if n == 2 {
        for i in 0..m {
            ret[0][i] = 1;
            ret[1][i] = 0;
        }
    } else if m == 2 {
        for i in 0..n {
            ret[i][0] = 1;
            ret[i][1] = 0;
        }
    } else if n <= m {
        for i in 0..n {
            for j in 0..m {
                ret[i][j] = if i == 1 || i == n - 2 {
                    1
                } else if i == 0 || i == n - 1 {
                    0
                } else {
                    if j % 2 == 0 {
                        0
                    } else {
                        1
                    }
                };
            }
        }
    } else {
        for i in 0..m {
            for j in 0..n {
                ret[j][i] = if j == 1 || j == n - 2 {
                    1
                } else if j == 0 || j == n - 1 {
                    0
                } else {
                    if i % 2 == 0 {
                        0
                    } else {
                        1
                    }
                };
            }
        }
    }

    writeln!(out, "YES").unwrap();

    for i in 0..n {
        for j in 0..m {
            write!(out, "{}", ret[i][j]).unwrap();
        }

        writeln!(out).unwrap();
    }
}
