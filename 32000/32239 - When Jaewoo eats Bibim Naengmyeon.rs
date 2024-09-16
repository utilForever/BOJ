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

const MOD: i64 = 1_000_000_007;

fn pow(x: i64, mut y: i64) -> i64 {
    let mut ret = 1;
    let mut piv = x % MOD;

    while y != 0 {
        if y & 1 != 0 {
            ret = ret * piv % MOD
        }

        piv = piv * piv % MOD;
        y >>= 1;
    }

    ret
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

    pub fn xor(nums1: &mut Vec<i64>, nums2: &mut Vec<i64>, inv: bool) {
        let n = nums1.len();
        let mut cnt = 0;
        let mut s = 2;
        let mut h = 1;

        while s <= n {
            let val_inv = crate::pow(nums2[cnt], crate::MOD - 2);
            let val_inv_minus_one = crate::pow(nums2[cnt] - 1, crate::MOD - 2);

            for i in (0..n).step_by(s) {
                for j in 0..h {
                    let t = nums1[i + h + j];

                    if inv {
                        nums1[i + h + j] = (nums1[i + j] - t) % crate::MOD * (nums2[cnt] - 1)
                            % crate::MOD
                            * val_inv
                            % crate::MOD;
                        nums1[i + j] = (nums1[i + j] + t * (nums2[cnt] - 1)) % crate::MOD * val_inv
                            % crate::MOD;
                    } else {
                        nums1[i + h + j] = (nums1[i + j] - t * val_inv_minus_one) % crate::MOD;
                        nums1[i + j] = (nums1[i + j] + t) % crate::MOD;
                    }
                }
            }

            s *= 2;
            h *= 2;
            cnt += 1;
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
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let n = scan.token::<usize>();
    let mut pieces = vec![0; n];

    for i in 0..n {
        pieces[i] = scan.token::<i64>();
    }

    let m = scan.token::<i64>();
    let m_inv = pow(m, MOD - 2);
    let mut ingredients = vec![0; 1 << n];

    for _ in 0..m {
        let t = scan.token::<i64>();
        let mut val = 0;

        for _ in 0..t {
            let x = scan.token::<usize>();
            val += 1 << (x - 1);
        }

        ingredients[val] = (ingredients[val] + m_inv) % MOD;
    }

    let k = scan.token::<i64>();

    fwht::xor(&mut ingredients, &mut pieces, false);

    for i in 0..ingredients.len() {
        ingredients[i] = pow(ingredients[i], k);
    }

    fwht::xor(&mut ingredients, &mut pieces, true);

    writeln!(out, "{}", (ingredients[0] + MOD) % MOD).unwrap();
}
