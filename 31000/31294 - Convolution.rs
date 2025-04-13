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

const MOD: i64 = 1 << 32;

pub mod fwht {
    use super::MOD;

    pub fn or(nums: &mut Vec<i64>, inv: bool) {
        let n = nums.len();
        let dir = if inv { -1 } else { 1 };
        let mut s = 2;
        let mut h = 1;

        while s <= n {
            for i in (0..n).step_by(s) {
                for j in 0..h {
                    let val = (dir * nums[i + j]).rem_euclid(MOD);
                    nums[i + h + j] = (nums[i + h + j] + val).rem_euclid(MOD);
                }
            }

            s *= 2;
            h *= 2;
        }
    }

    pub fn convolution_or(f: &Vec<i64>, g: &Vec<i64>) -> Vec<i64> {
        let n = f.len();
        let mut f_t = f.clone();
        let mut g_t = g.clone();

        or(&mut f_t, false);
        or(&mut g_t, false);

        let mut h = vec![0; n];

        for i in 0..n {
            h[i] = (f_t[i] * g_t[i]).rem_euclid(MOD);
        }

        or(&mut h, true);

        h
    }
}

fn next_rand16(cur: &mut u32, a: u32, b: u32) -> u32 {
    *cur = *cur * a + b;
    *cur >> 16
}

// Reference: https://gina65.tistory.com/30
fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let (n, t) = (scan.token::<usize>(), scan.token::<i64>());
    let (a, b) = (scan.token::<u32>(), scan.token::<u32>());
    let mut seed = 0;

    for _ in 0..t {
        let mut f = Vec::with_capacity(1 << n);
        let mut g = Vec::with_capacity(1 << n);

        for _ in 0..1 << n {
            f.push(next_rand16(&mut seed, a, b) as i64);
        }

        for _ in 0..1 << n {
            g.push(next_rand16(&mut seed, a, b) as i64);
        }

        let h = fwht::convolution_or(&f, &g);
        let mut ret = 0;

        for i in 0..1 << n {
            ret = (ret + h[i] * (i as i64 + 1)) % MOD;
        }

        writeln!(out, "{}", ret % MOD).unwrap();
    }
}
