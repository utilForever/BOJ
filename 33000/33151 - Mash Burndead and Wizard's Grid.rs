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

    let (n, k) = (scan.token::<usize>(), scan.token::<usize>());

    if n == 1 {
        writeln!(out, "{k}").unwrap();
        return;
    }

    if n % 2 == 0 {
        let minimum = n * n / 2;

        if k % 2 == 1 || k < minimum {
            writeln!(out, "-1").unwrap();
            return;
        }

        let remain = k - minimum;

        if remain % 2 == 1 {
            writeln!(out, "-1").unwrap();
            return;
        }

        let q0 = remain / 2;
        let q = q0 / (n * n);
        let r = q0 % (n * n);
        let mut ret = vec![vec![0; n]; n];

        for i in 0..n {
            for j in 0..n {
                ret[i][j] = q * 2 + if (i + j) % 2 == 0 { 0 } else { 1 };
            }
        }

        let mut cnt = 0;

        'outer: for i in 0..n {
            for j in (if i % 2 == 0 { 0 } else { 1 }..n).step_by(2) {
                if cnt == r {
                    break 'outer;
                }

                ret[i][j] += 2;
                cnt += 1;
            }
        }

        'outer: for i in 0..n {
            for j in (if i % 2 == 0 { 1 } else { 0 }..n).step_by(2) {
                if cnt == r {
                    break 'outer;
                }

                ret[i][j] += 2;
                cnt += 1;
            }
        }

        for i in 0..n {
            for j in 0..n {
                write!(out, "{} ", ret[i][j]).unwrap();
            }

            writeln!(out).unwrap();
        }
    } else {
        if k % 2 == 0 {
            let minimum = n * (n / 2) + n / 2;

            if k < minimum {
                writeln!(out, "-1").unwrap();
                return;
            }

            let remain = k - minimum;

            if remain % 2 == 1 {
                writeln!(out, "-1").unwrap();
                return;
            }

            let q0 = remain / 2;
            let q = q0 / (n * n);
            let r = q0 % (n * n);
            let mut ret = vec![vec![0; n]; n];

            for i in 0..n {
                for j in 0..n {
                    ret[i][j] = q * 2 + if (i + j) % 2 == 0 { 0 } else { 1 };
                }
            }

            let mut cnt = 0;

            'outer: for i in 0..n {
                for j in (if i % 2 == 0 { 0 } else { 1 }..n).step_by(2) {
                    if cnt == r {
                        break 'outer;
                    }

                    ret[i][j] += 2;
                    cnt += 1;
                }
            }

            'outer: for i in 0..n {
                for j in (if i % 2 == 0 { 1 } else { 0 }..n).step_by(2) {
                    if cnt == r {
                        break 'outer;
                    }

                    ret[i][j] += 2;
                    cnt += 1;
                }
            }

            for i in 0..n {
                for j in 0..n {
                    write!(out, "{} ", ret[i][j]).unwrap();
                }

                writeln!(out).unwrap();
            }
        } else {
            let minimum = n * (n / 2) + n / 2 + 1;

            if k < minimum {
                writeln!(out, "-1").unwrap();
                return;
            }

            let remain = k - minimum;

            if remain % 2 == 1 {
                writeln!(out, "-1").unwrap();
                return;
            }

            let q0 = remain / 2;
            let q = q0 / (n * n);
            let r = q0 % (n * n);
            let mut ret = vec![vec![0; n]; n];

            for i in 0..n {
                for j in 0..n {
                    ret[i][j] = q * 2 + if (i + j) % 2 == 0 { 1 } else { 0 };
                }
            }

            let mut cnt = 0;

            'outer: for i in 0..n {
                for j in (if i % 2 == 0 { 1 } else { 0 }..n).step_by(2) {
                    if cnt == r {
                        break 'outer;
                    }

                    ret[i][j] += 2;
                    cnt += 1;
                }
            }

            'outer: for i in 0..n {
                for j in (if i % 2 == 0 { 0 } else { 1 }..n).step_by(2) {
                    if cnt == r {
                        break 'outer;
                    }

                    ret[i][j] += 2;
                    cnt += 1;
                }
            }

            for i in 0..n {
                for j in 0..n {
                    write!(out, "{} ", ret[i][j]).unwrap();
                }

                writeln!(out).unwrap();
            }
        }
    }
}
