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

    let mut max_first = i64::MIN;
    let mut max_second = i64::MIN;
    let mut min_first = i64::MAX;
    let mut min_second = i64::MAX;

    for i in 0..n {
        if nums[i] > max_first {
            max_second = max_first;
            max_first = nums[i];
        } else if nums[i] > max_second {
            max_second = nums[i];
        }

        if nums[i] < min_first {
            min_second = min_first;
            min_first = nums[i];
        } else if nums[i] < min_second {
            min_second = nums[i];
        }
    }

    let diff1 = (2 * max_first * max_second - max_first - max_second).max(0);
    let diff2 = (2 * min_first * min_second - min_first - min_second).max(0);
    let diff_max = diff1.max(diff2);

    writeln!(out, "{}", nums.iter().sum::<i64>() + diff_max).unwrap();
}
