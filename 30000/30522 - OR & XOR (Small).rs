use io::Write;
use std::{io, str, vec};

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

    let (n, p) = (scan.token::<usize>(), scan.token::<i64>());
    let mut a = vec![0; n];
    let mut b = vec![0; n];

    for i in 0..n {
        a[i] = scan.token::<i64>();
    }

    for i in 0..n {
        b[i] = scan.token::<i64>();
    }

    let mut cnt_a = vec![0; 17];
    let mut cnt_b = vec![0; 17];

    for &val in a.iter() {
        for bit in 0..17 {
            if (val >> bit) & 1 == 1 {
                cnt_a[bit] += 1;
            }
        }
    }

    for &val in b.iter() {
        for bit in 0..17 {
            if (val >> bit) & 1 == 1 {
                cnt_b[bit] += 1;
            }
        }
    }

    let mut base = 0;

    for bit in 0..17 {
        let val = 1i64 << bit;
        let a_ones = cnt_a[bit];
        let b_ones = cnt_b[bit];
        let a_zeros = n as i64 - a_ones;
        let b_zeros = n as i64 - b_ones;

        base += val * (a_ones * b_zeros + a_zeros * b_ones);
    }

    let mut fa = vec![0; 1 << 17];
    let mut fb = vec![0; 1 << 17];

    for i in 0..n {
        fa[a[i] as usize] += 1;
        fb[b[i] as usize] += 1;
    }

    fwht::and(&mut fa, false);
    fwht::and(&mut fb, false);

    for i in 0..(1 << 17) {
        fa[i] *= fb[i];
    }

    fwht::and(&mut fa, true);

    let mut remain = p;
    let mut extra = 0;

    for mask in (0..1 << 17).rev() {
        if remain == 0 {
            break;
        }

        let cnt = fa[mask];

        if cnt == 0 {
            continue;
        }

        if cnt <= remain {
            extra += (mask as i64) * cnt;
            remain -= cnt;
        } else {
            extra += (mask as i64) * remain;
            remain = 0;
        }
    }

    writeln!(out, "{}", base + extra).unwrap();
}
