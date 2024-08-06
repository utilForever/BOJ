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

    let (n, k) = (scan.token::<usize>(), scan.token::<i64>());
    let mut nums = vec![0; n];

    for i in 0..n {
        nums[i] = scan.token::<i64>();
    }

    let mut cnt = 0;

    for last in (1..n).rev() {
        let max = *nums[0..=last].iter().max().unwrap();
        let pos = nums[0..=last].iter().position(|&x| x == max).unwrap();

        if pos != last {
            nums.swap(pos, last);
            cnt += 1;

            if cnt == k {
                if nums[pos] > nums[last] {
                    writeln!(out, "{} {}", nums[last], nums[pos]).unwrap();
                } else {
                    writeln!(out, "{} {}", nums[pos], nums[last]).unwrap();
                }

                return;
            }
        }
    }

    writeln!(out, "-1").unwrap();
}
