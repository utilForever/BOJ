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
    let mut nums = vec![0; n + 1];

    for i in 0..n {
        nums[i] = i + 1;
    }

    write!(out, "<").unwrap();

    let (mut idx, mut cnt, mut total_cnt) = (0, 0, 0);

    while total_cnt < n {
        if nums[idx] == 0 {
            idx += 1;
        } else {
            cnt += 1;

            if cnt == k {
                write!(out, "{}", nums[idx]).unwrap();

                nums[idx] = 0;
                cnt = 0;
                total_cnt += 1;

                if total_cnt < n {
                    write!(out, ", ").unwrap();
                }
            }

            idx += 1;
        }

        if idx == n {
            idx = 0;
        }
    }

    write!(out, ">").unwrap();
}
