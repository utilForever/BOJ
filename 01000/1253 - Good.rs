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
    let mut nums = vec![0; n];

    for i in 0..n {
        nums[i] = scan.token::<i64>();
    }

    nums.sort_unstable();

    let mut ret = 0;

    for i in 0..nums.len() {
        let mut left = 0;
        let mut right = nums.len() - 1;

        while left < right {
            if left == i {
                left += 1;
                continue;
            }

            if right == i {
                right -= 1;
                continue;
            }

            let sum = nums[left] + nums[right];

            if sum == nums[i] {
                ret += 1;
                break;
            } else if sum < nums[i] {
                left += 1;
            } else {
                right -= 1;
            }
        }
    }

    writeln!(out, "{ret}").unwrap();
}
