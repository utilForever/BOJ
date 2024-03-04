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
    let dir = scan.token::<char>();
    let mut array = vec![vec![0; m]; n];

    match dir {
        'U' => {
            // Snail Array (Left, 0 ~ m / 2 - 1)
            {
                let end = (m / 2 + 1) * n;
                let mut pos = (0, m / 2);

                array[pos.0][pos.1] = 1;

                let mut val = 2;

                while val <= end {
                    // Down
                    while pos.0 + 1 < n && array[pos.0 + 1][pos.1] == 0 {
                        pos.0 += 1;
                        array[pos.0][pos.1] = val;
                        val += 1;
                    }

                    // Left
                    while pos.1 > 0 && array[pos.0][pos.1 - 1] == 0 {
                        pos.1 -= 1;
                        array[pos.0][pos.1] = val;
                        val += 1;
                    }

                    // Up
                    while pos.0 > 0 && array[pos.0 - 1][pos.1] == 0 {
                        pos.0 -= 1;
                        array[pos.0][pos.1] = val;
                        val += 1;
                    }

                    // Right
                    while pos.1 + 1 < m && array[pos.0][pos.1 + 1] == 0 {
                        pos.1 += 1;
                        array[pos.0][pos.1] = val;
                        val += 1;
                    }
                }
            }

            for i in 0..n {
                array[i][m / 2] = 0;
            }

            // Snail Array (Right, m / 2 ~ m - 1)
            {
                let end = (m / 2 + 1) * n;
                let mut pos = (0, m / 2);

                array[pos.0][pos.1] = 1;

                let mut val = 2;

                while val <= end {
                    // Down
                    while pos.0 + 1 < n && array[pos.0 + 1][pos.1] == 0 {
                        pos.0 += 1;
                        array[pos.0][pos.1] = val;
                        val += 1;
                    }

                    // Right
                    while pos.1 + 1 < m && array[pos.0][pos.1 + 1] == 0 {
                        pos.1 += 1;
                        array[pos.0][pos.1] = val;
                        val += 1;
                    }

                    // Up
                    while pos.0 > 0 && array[pos.0 - 1][pos.1] == 0 {
                        pos.0 -= 1;
                        array[pos.0][pos.1] = val;
                        val += 1;
                    }

                    // Left
                    while pos.1 > 0 && array[pos.0][pos.1 - 1] == 0 {
                        pos.1 -= 1;
                        array[pos.0][pos.1] = val;
                        val += 1;
                    }
                }
            }
        }
        'D' => {
            // Snail Array (Left, 0 ~ m / 2 - 1)
            {
                let end = (m / 2 + 1) * n;
                let mut pos = (n - 1, m / 2);

                array[pos.0][pos.1] = 1;

                let mut val = 2;

                while val <= end {
                    // Up
                    while pos.0 > 0 && array[pos.0 - 1][pos.1] == 0 {
                        pos.0 -= 1;
                        array[pos.0][pos.1] = val;
                        val += 1;
                    }

                    // Left
                    while pos.1 > 0 && array[pos.0][pos.1 - 1] == 0 {
                        pos.1 -= 1;
                        array[pos.0][pos.1] = val;
                        val += 1;
                    }

                    // Down
                    while pos.0 + 1 < n && array[pos.0 + 1][pos.1] == 0 {
                        pos.0 += 1;
                        array[pos.0][pos.1] = val;
                        val += 1;
                    }

                    // Right
                    while pos.1 + 1 < m && array[pos.0][pos.1 + 1] == 0 {
                        pos.1 += 1;
                        array[pos.0][pos.1] = val;
                        val += 1;
                    }
                }
            }

            for i in 0..n {
                array[i][m / 2] = 0;
            }

            // Snail Array (Right, m / 2 ~ m - 1)
            {
                let end = (m / 2 + 1) * n;
                let mut pos = (n - 1, m / 2);

                array[pos.0][pos.1] = 1;

                let mut val = 2;

                while val <= end {
                    // Up
                    while pos.0 > 0 && array[pos.0 - 1][pos.1] == 0 {
                        pos.0 -= 1;
                        array[pos.0][pos.1] = val;
                        val += 1;
                    }

                    // Right
                    while pos.1 + 1 < m && array[pos.0][pos.1 + 1] == 0 {
                        pos.1 += 1;
                        array[pos.0][pos.1] = val;
                        val += 1;
                    }

                    // Down
                    while pos.0 + 1 < n && array[pos.0 + 1][pos.1] == 0 {
                        pos.0 += 1;
                        array[pos.0][pos.1] = val;
                        val += 1;
                    }

                    // Left
                    while pos.1 > 0 && array[pos.0][pos.1 - 1] == 0 {
                        pos.1 -= 1;
                        array[pos.0][pos.1] = val;
                        val += 1;
                    }
                }
            }
        }
        'L' => {
            // Snail Array (Up, 0 ~ n / 2 - 1)
            {
                let end = (n / 2 + 1) * m;
                let mut pos = (n / 2, 0);

                array[pos.0][pos.1] = 1;

                let mut val = 2;

                while val <= end {
                    // Right
                    while pos.1 + 1 < m && array[pos.0][pos.1 + 1] == 0 {
                        pos.1 += 1;
                        array[pos.0][pos.1] = val;
                        val += 1;
                    }

                    // Down
                    while pos.0 + 1 < n && array[pos.0 + 1][pos.1] == 0 {
                        pos.0 += 1;
                        array[pos.0][pos.1] = val;
                        val += 1;
                    }

                    // Left
                    while pos.1 > 0 && array[pos.0][pos.1 - 1] == 0 {
                        pos.1 -= 1;
                        array[pos.0][pos.1] = val;
                        val += 1;
                    }

                    // Up
                    while pos.0 > 0 && array[pos.0 - 1][pos.1] == 0 {
                        pos.0 -= 1;
                        array[pos.0][pos.1] = val;
                        val += 1;
                    }
                }
            }

            for i in 0..m {
                array[n / 2][i] = 0;
            }

            // Snail Array (Down, n / 2 ~ n - 1)
            {
                let end = (n / 2 + 1) * m;
                let mut pos = (n / 2, 0);

                array[pos.0][pos.1] = 1;

                let mut val = 2;

                while val <= end {
                    // Right
                    while pos.1 + 1 < m && array[pos.0][pos.1 + 1] == 0 {
                        pos.1 += 1;
                        array[pos.0][pos.1] = val;
                        val += 1;
                    }

                    // Up
                    while pos.0 > 0 && array[pos.0 - 1][pos.1] == 0 {
                        pos.0 -= 1;
                        array[pos.0][pos.1] = val;
                        val += 1;
                    }

                    // Left
                    while pos.1 > 0 && array[pos.0][pos.1 - 1] == 0 {
                        pos.1 -= 1;
                        array[pos.0][pos.1] = val;
                        val += 1;
                    }

                    // Down
                    while pos.0 + 1 < n && array[pos.0 + 1][pos.1] == 0 {
                        pos.0 += 1;
                        array[pos.0][pos.1] = val;
                        val += 1;
                    }
                }
            }
        }
        'R' => {
            // Snail Array (Up, 0 ~ n / 2 - 1)
            {
                let end = (n / 2 + 1) * m;
                let mut pos = (n / 2, m - 1);

                array[pos.0][pos.1] = 1;

                let mut val = 2;

                while val <= end {
                    // Left
                    while pos.1 > 0 && array[pos.0][pos.1 - 1] == 0 {
                        pos.1 -= 1;
                        array[pos.0][pos.1] = val;
                        val += 1;
                    }

                    // Down
                    while pos.0 + 1 < n && array[pos.0 + 1][pos.1] == 0 {
                        pos.0 += 1;
                        array[pos.0][pos.1] = val;
                        val += 1;
                    }

                    // Right
                    while pos.1 + 1 < m && array[pos.0][pos.1 + 1] == 0 {
                        pos.1 += 1;
                        array[pos.0][pos.1] = val;
                        val += 1;
                    }

                    // Up
                    while pos.0 > 0 && array[pos.0 - 1][pos.1] == 0 {
                        pos.0 -= 1;
                        array[pos.0][pos.1] = val;
                        val += 1;
                    }
                }
            }

            for i in 0..m {
                array[n / 2][i] = 0;
            }

            // Snail Array (Down, n / 2 ~ n - 1)
            {
                let end = (n / 2 + 1) * m;
                let mut pos = (n / 2, m - 1);

                array[pos.0][pos.1] = 1;

                let mut val = 2;

                while val <= end {
                    // Left
                    while pos.1 > 0 && array[pos.0][pos.1 - 1] == 0 {
                        pos.1 -= 1;
                        array[pos.0][pos.1] = val;
                        val += 1;
                    }

                    // Up
                    while pos.0 > 0 && array[pos.0 - 1][pos.1] == 0 {
                        pos.0 -= 1;
                        array[pos.0][pos.1] = val;
                        val += 1;
                    }

                    // Right
                    while pos.1 + 1 < m && array[pos.0][pos.1 + 1] == 0 {
                        pos.1 += 1;
                        array[pos.0][pos.1] = val;
                        val += 1;
                    }

                    // Down
                    while pos.0 + 1 < n && array[pos.0 + 1][pos.1] == 0 {
                        pos.0 += 1;
                        array[pos.0][pos.1] = val;
                        val += 1;
                    }
                }
            }
        }
        _ => unreachable!(),
    }

    for i in 0..n {
        for j in 0..m {
            write!(out, "{} ", array[i][j]).unwrap();
        }

        writeln!(out).unwrap();
    }
}
