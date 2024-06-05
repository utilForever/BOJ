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
    let mut cnt = vec![0; 11];

    for i in 0..n {
        nums[i] = scan.token::<usize>();
    }

    let mut left = 0;
    let mut right = 0;
    let mut min = 11;
    let mut max = 0;
    let mut ret = 0;

    while left < n && right < n {
        cnt[nums[right]] += 1;

        if cnt[nums[right]] == 1 {
            min = min.min(nums[right]);
            max = max.max(nums[right]);
        }

        while max - min > 2 {
            cnt[nums[left]] -= 1;

            if cnt[nums[left]] == 0 {
                min = 11;
                max = 0;

                for i in 1..=10 {
                    if cnt[i] == 0 {
                        continue;
                    }

                    min = min.min(i);
                    max = max.max(i);
                }
            }

            left += 1;
        }

        ret = ret.max(right - left + 1);
        right += 1;
    }

    writeln!(out, "{ret}").unwrap();
}
