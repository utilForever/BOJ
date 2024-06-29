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

    let (mut n, mut m, r) = (
        scan.token::<usize>(),
        scan.token::<usize>(),
        scan.token::<i64>(),
    );
    let mut array = vec![vec![0; m]; n];

    for i in 0..n {
        for j in 0..m {
            array[i][j] = scan.token::<i64>();
        }
    }

    for _ in 0..r {
        let op = scan.token::<i64>();

        match op {
            // Inversion up and down
            1 => {
                let mut array_new = vec![vec![0; m]; n];

                for i in 0..n {
                    for j in 0..m {
                        array_new[i][j] = array[n - i - 1][j];
                    }
                }

                array = array_new;
            }
            // Inversion left and right
            2 => {
                let mut array_new = vec![vec![0; m]; n];

                for i in 0..n {
                    for j in 0..m {
                        array_new[i][j] = array[i][m - j - 1];
                    }
                }

                array = array_new;
            }
            // Rotate 90 degrees clockwise
            3 => {
                let mut array_new = vec![vec![0; n]; m];

                for i in 0..n {
                    for j in 0..m {
                        array_new[j][n - i - 1] = array[i][j];
                    }
                }

                std::mem::swap(&mut n, &mut m);
                array = array_new;
            }
            // Rotate 90 degrees counterclockwise
            4 => {
                let mut array_new = vec![vec![0; n]; m];

                for i in 0..n {
                    for j in 0..m {
                        array_new[m - j - 1][i] = array[i][j];
                    }
                }

                std::mem::swap(&mut n, &mut m);
                array = array_new;
            }
            // Divide (n / 2) x (m / 2) submatrices and rotate each submatrix 90 degrees clockwise
            5 => {
                let mut array_subs = vec![vec![vec![0; m / 2]; n / 2]; 4];

                for i in 0..n / 2 {
                    for j in 0..m / 2 {
                        array_subs[0][i][j] = array[i][j];
                        array_subs[1][i][j] = array[i][j + m / 2];
                        array_subs[2][i][j] = array[i + n / 2][j];
                        array_subs[3][i][j] = array[i + n / 2][j + m / 2];
                    }
                }

                for i in 0..n / 2 {
                    for j in 0..m / 2 {
                        array[i][j] = array_subs[2][i][j];
                        array[i][j + m / 2] = array_subs[0][i][j];
                        array[i + n / 2][j] = array_subs[3][i][j];
                        array[i + n / 2][j + m / 2] = array_subs[1][i][j];
                    }
                }
            }
            // Divide (n / 2) x (m / 2) submatrices and rotate each submatrix 90 degrees counterclockwise
            6 => {
                let mut array_subs = vec![vec![vec![0; m / 2]; n / 2]; 4];

                for i in 0..n / 2 {
                    for j in 0..m / 2 {
                        array_subs[0][i][j] = array[i][j];
                        array_subs[1][i][j] = array[i][j + m / 2];
                        array_subs[2][i][j] = array[i + n / 2][j];
                        array_subs[3][i][j] = array[i + n / 2][j + m / 2];
                    }
                }

                for i in 0..n / 2 {
                    for j in 0..m / 2 {
                        array[i][j] = array_subs[1][i][j];
                        array[i][j + m / 2] = array_subs[3][i][j];
                        array[i + n / 2][j] = array_subs[0][i][j];
                        array[i + n / 2][j + m / 2] = array_subs[2][i][j];
                    }
                }
            }
            _ => unreachable!(),
        }
    }

    for i in 0..n {
        for j in 0..m {
            write!(out, "{} ", array[i][j]).unwrap();
        }

        writeln!(out).unwrap();
    }
}
