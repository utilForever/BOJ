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

    pub fn line(&mut self) -> String {
        let mut input = String::new();
        self.reader.read_line(&mut input).expect("Failed read");
        input
    }
}

pub mod fwht {
    pub fn or(nums: &mut Vec<i64>, inv: bool) {
        let n = nums.len();
        let dir = if inv { -1 } else { 1 };
        let mut s = 2;
        let mut h = 1;

        while s <= n {
            for i in (0..n).step_by(s) {
                for j in 0..h {
                    nums[i + h + j] += dir * nums[i + j];
                }
            }

            s *= 2;
            h *= 2;
        }
    }

    pub fn and(nums: &mut Vec<i64>, inv: bool) {
        let n = nums.len();
        let dir = if inv { -1 } else { 1 };
        let mut s = 2;
        let mut h = 1;

        while s <= n {
            for i in (0..n).step_by(s) {
                for j in 0..h {
                    nums[i + j] += dir * nums[i + h + j];
                }
            }

            s *= 2;
            h *= 2;
        }
    }

    pub fn xor(nums: &mut Vec<i64>, inv: bool) {
        let n = nums.len();
        let mut s = 2;
        let mut h = 1;

        while s <= n {
            for i in (0..n).step_by(s) {
                for j in 0..h {
                    let t = nums[i + h + j];
                    nums[i + h + j] = nums[i + j] - t;
                    nums[i + j] += t;

                    if inv {
                        nums[i + h + j] /= 2;
                        nums[i + j] /= 2;
                    }
                }
            }

            s *= 2;
            h *= 2;
        }
    }

    pub fn convolution_or(nums: &Vec<i64>, idx: usize) -> i64 {
        let mut nums_cloned = nums.clone();

        or(&mut nums_cloned, false);

        for i in 0..nums.len() {
            nums_cloned[i] *= nums_cloned[i];
        }

        or(&mut nums_cloned, true);

        (nums_cloned[idx] - nums[idx]) / 2
    }

    pub fn convolution_and(nums: &Vec<i64>, idx: usize) -> i64 {
        let mut nums_cloned = nums.clone();

        and(&mut nums_cloned, false);

        for i in 0..nums.len() {
            nums_cloned[i] *= nums_cloned[i];
        }

        and(&mut nums_cloned, true);

        (nums_cloned[idx] - nums[idx]) / 2
    }

    pub fn convolution_xor(nums: &Vec<i64>, idx: usize) -> i64 {
        let mut nums_cloned = nums.clone();

        xor(&mut nums_cloned, false);

        for i in 0..nums_cloned.len() {
            nums_cloned[i] *= nums_cloned[i];
        }

        xor(&mut nums_cloned, true);

        if idx == 0 {
            nums_cloned[0] -= nums.iter().sum::<i64>();
        }

        nums_cloned[idx] / 2
    }
}

// Reference: https://gina65.tistory.com/30
// Reference: SUAPC 2022 Summer Editorial
fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let (n, k) = (scan.token::<i64>(), scan.token::<usize>());
    let mut nums = vec![0; 1048576];

    for _ in 0..n {
        let idx = scan.token::<usize>();
        nums[idx] += 1;
    }

    let and = fwht::convolution_and(&nums, k);
    let or = fwht::convolution_or(&nums, k);
    let xor = fwht::convolution_xor(&nums, k);

    writeln!(out, "{and} {or} {xor}").unwrap();
}
