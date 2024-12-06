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

    let n = scan.token::<usize>();
    let mut grid = vec![vec![' '; n]; n];

    for i in 0..n {
        let line = scan.token::<String>();

        for (j, c) in line.chars().enumerate() {
            grid[i][j] = c;
        }
    }

    let mobis = ['M', 'O', 'B', 'I', 'S'];
    let mut ret = 0;

    for i in 0..n {
        for j in 0..n {
            // North
            {
                if i as i64 - 4 >= 0 {
                    let mut found = true;

                    for k in 0..5 {
                        if grid[i - k][j] != mobis[k] {
                            found = false;
                            break;
                        }
                    }

                    if found {
                        ret += 1;
                    }
                }
            }

            // North-East
            {
                if i as i64 - 4 >= 0 && j + 4 < n {
                    let mut found = true;

                    for k in 0..5 {
                        if grid[i - k][j + k] != mobis[k] {
                            found = false;
                            break;
                        }
                    }

                    if found {
                        ret += 1;
                    }
                }
            }

            // East
            {
                if j + 4 < n {
                    let mut found = true;

                    for k in 0..5 {
                        if grid[i][j + k] != mobis[k] {
                            found = false;
                            break;
                        }
                    }

                    if found {
                        ret += 1;
                    }
                }
            }

            // South-East
            {
                if i + 4 < n && j + 4 < n {
                    let mut found = true;

                    for k in 0..5 {
                        if grid[i + k][j + k] != mobis[k] {
                            found = false;
                            break;
                        }
                    }

                    if found {
                        ret += 1;
                    }
                }
            }

            // South
            {
                if i + 4 < n {
                    let mut found = true;

                    for k in 0..5 {
                        if grid[i + k][j] != mobis[k] {
                            found = false;
                            break;
                        }
                    }

                    if found {
                        ret += 1;
                    }
                }
            }

            // South-West
            {
                if i + 4 < n && j as i64 - 4 >= 0 {
                    let mut found = true;

                    for k in 0..5 {
                        if grid[i + k][j - k] != mobis[k] {
                            found = false;
                            break;
                        }
                    }

                    if found {
                        ret += 1;
                    }
                }
            }

            // West
            {
                if j as i64 - 4 >= 0 {
                    let mut found = true;

                    for k in 0..5 {
                        if grid[i][j - k] != mobis[k] {
                            found = false;
                            break;
                        }
                    }

                    if found {
                        ret += 1;
                    }
                }
            }

            // North-West
            {
                if i as i64 - 4 >= 0 && j as i64 - 4 >= 0 {
                    let mut found = true;

                    for k in 0..5 {
                        if grid[i - k][j - k] != mobis[k] {
                            found = false;
                            break;
                        }
                    }

                    if found {
                        ret += 1;
                    }
                }
            }
        }
    }

    writeln!(out, "{ret}").unwrap();
}
