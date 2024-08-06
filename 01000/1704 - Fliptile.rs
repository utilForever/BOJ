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

    let (m, n) = (scan.token::<usize>(), scan.token::<usize>());
    let mut cows = vec![vec![0; n]; m];
    let mut cnt_min = i64::MAX;
    let mut ret = Vec::new();

    for i in 0..m {
        for j in 0..n {
            cows[i][j] = scan.token::<u8>();
        }
    }

    // Iterate over all possible combinations of the first row
    for i in 0..2i32.pow(n as u32) {
        let mut switches = vec![vec![0; n]; m];
        let mut cows_clone = cows.clone();
        let mut cnt = 0;

        // Apply the combination to the first row
        for j in 0..n {
            if i & (1 << j) != 0 {
                let j = n - j - 1;

                switches[0][j] = 1;
                cows_clone[0][j] ^= 1;
                cnt += 1;

                if j > 0 {
                    cows_clone[0][j - 1] ^= 1;
                }

                if j < n - 1 {
                    cows_clone[0][j + 1] ^= 1;
                }

                if m > 1 {
                    cows_clone[1][j] ^= 1;
                }
            }
        }

        // Apply the combination to the rest of the rows
        for j in 1..m {
            for k in 0..n {
                if cows_clone[j - 1][k] == 1 {
                    switches[j][k] = 1;
                    cows_clone[j][k] ^= 1;
                    cnt += 1;

                    if k > 0 {
                        cows_clone[j][k - 1] ^= 1;
                    }

                    if k < n - 1 {
                        cows_clone[j][k + 1] ^= 1;
                    }

                    if j < m - 1 {
                        cows_clone[j + 1][k] ^= 1;
                    }
                }
            }
        }

        let mut is_satisfy = true;

        for j in 0..n {
            if cows_clone[m - 1][j] == 1 {
                is_satisfy = false;
                break;
            }
        }

        if is_satisfy && cnt < cnt_min {
            cnt_min = cnt;
            ret = switches;
        }
    }

    if cnt_min == i64::MAX {
        writeln!(out, "IMPOSSIBLE").unwrap();
    } else {
        for i in 0..m {
            for j in 0..n {
                write!(out, "{} ", ret[i][j]).unwrap();
            }

            writeln!(out).unwrap();
        }
    }
}
