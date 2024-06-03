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
    let mut blubs = vec![vec![0; n]; n];
    let mut ret = i64::MAX;

    for i in 0..n {
        for j in 0..n {
            blubs[i][j] = scan.token::<u8>();
        }
    }

    // Iterate over all possible combinations of the first row
    for i in 0..2i32.pow(n as u32) {
        let mut switches = vec![0; n];
        let mut blubs_clone = blubs.clone();
        let mut cnt = 0;

        // Apply the combination to the first row
        for j in 0..n {
            if i & (1 << j) != 0 {
                switches[j] = 1;
                blubs_clone[0][j] ^= 1;
                cnt += 1;

                if j > 0 {
                    blubs_clone[0][j - 1] ^= 1;
                }

                if j < n - 1 {
                    blubs_clone[0][j + 1] ^= 1;
                }

                blubs_clone[1][j] ^= 1;
            }
        }

        // Apply the combination to the rest of the rows
        for j in 1..n {
            for k in 0..n {
                if blubs_clone[j - 1][k] == 1 {
                    switches[k] = 1;
                    blubs_clone[j][k] ^= 1;
                    cnt += 1;

                    if k > 0 {
                        blubs_clone[j][k - 1] ^= 1;
                    }

                    if k < n - 1 {
                        blubs_clone[j][k + 1] ^= 1;
                    }

                    if j < n - 1 {
                        blubs_clone[j + 1][k] ^= 1;
                    }
                }
            }
        }

        let mut is_satisfy = true;

        for j in 0..n {
            if blubs_clone[n - 1][j] == 1 {
                is_satisfy = false;
                break;
            }
        }

        if is_satisfy {
            ret = ret.min(cnt);
        }
    }

    writeln!(out, "{}", if ret == i64::MAX { -1 } else { ret }).unwrap();
}
