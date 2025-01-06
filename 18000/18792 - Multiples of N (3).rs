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

fn mod_exp(base: usize, mut exp: usize, m: usize) -> usize {
    let mut ret = 1;
    let mut val = base % m;

    while exp > 0 {
        if exp % 2 == 1 {
            ret = (ret * val) % m;
        }

        val = (val * val) % m;
        exp /= 2;
    }

    ret
}

fn mod_inverse(d: usize, p: usize) -> usize {
    mod_exp(d, p - 2, p)
}

fn find_t(p: usize, t: &Vec<bool>, d: usize, u: usize, v: usize) -> usize {
    let d_inv = mod_inverse(d, p);
    let mut left = (u * d_inv) % p;
    let mut right = p + ((v * d_inv) % p);

    while left < right {
        let m = (left + right) / 2;

        if t[((m % p) * d) % p] {
            left = m + 1;
        } else {
            right = m;
        }
    }

    (right % p) * d % p
}

fn egz_prime(p: usize, nums: &Vec<i64>) -> Vec<bool> {
    let mut k = (0..2 * p - 1).collect::<Vec<usize>>();
    k.sort_by_key(|&x| nums[x] % p as i64);

    let mut ret = vec![false; 2 * p - 1];

    for i in 0..p - 1 {
        if (nums[k[1 + i]] % p as i64) == (nums[k[p + i]] % p as i64) {
            for j in i + 1..i + p + 1 {
                ret[k[j]] = true;
            }

            return ret;
        }
    }

    let mut sum = 0;

    for i in 0..p {
        sum = (sum + nums[k[i]] as usize) % p;
    }

    let mut t_check = vec![false; p];
    let mut t_idxes = vec![None; p];

    t_check[sum] = true;

    for i in 1..p {
        if t_check[0] {
            break;
        }

        let diff = (nums[k[p + i - 1]] - nums[k[i]]).rem_euclid(p as i64) as usize;
        let t = find_t(p, &t_check, diff, sum, 0);

        t_check[t] = true;
        t_idxes[t] = Some(i);
    }

    for i in 0..p {
        ret[k[i]] = true;
    }

    let mut c = 0;

    while sum != c {
        let idx = t_idxes[c].unwrap();

        ret[k[p + idx - 1]] = true;
        ret[k[idx]] = false;

        let diff = (nums[k[p + idx - 1]] - nums[k[idx]]).rem_euclid(p as i64);
        let c_new = c as i64 - diff;

        c = c_new.rem_euclid(p as i64) as usize;
    }

    ret
}

fn egz_composite(p: usize, q: usize, nums: &Vec<i64>) -> Vec<bool> {
    let mut s = (0..p - 1).collect::<Vec<usize>>();
    let mut t = vec![None; 2 * q - 1];

    for i in 0..2 * q - 1 {
        let start = (i + 1) * p - 1;
        let end = (i + 2) * p - 1;

        for val in start..end {
            s.push(val);
        }

        let mut nums_sub = Vec::with_capacity(nums.len());

        for &idx in s.iter() {
            nums_sub.push(nums[idx]);
        }

        let ret_p = egz(p, &nums_sub);

        let mut selected = Vec::new();
        let mut not_selected = Vec::new();

        for j in 0..(2 * p - 1) {
            if ret_p[j] {
                selected.push(s[j]);
            } else {
                not_selected.push(s[j]);
            }
        }

        t[i] = Some(selected);
        s = not_selected;
    }

    let mut ret = vec![false; 2 * p * q - 1];
    let mut sum_div_p = Vec::with_capacity(2 * q - 1);

    for i in 0..(2 * q - 1) {
        let group = t[i].as_ref().unwrap();
        let mut sum = 0;

        for &t in group {
            sum += nums[t];
        }

        sum_div_p.push(sum / p as i64);
    }

    let ret_q = egz(q, &sum_div_p);

    for i in 0..2 * q - 1 {
        if ret_q[i] {
            let group = t[i].as_ref().unwrap();

            for &t in group {
                ret[t] = true;
            }
        }
    }

    ret
}

pub fn egz(n: usize, nums: &Vec<i64>) -> Vec<bool> {
    if n == 1 {
        return vec![true];
    }

    // Check composite
    for i in 2..n {
        if n % i == 0 {
            return egz_composite(i, n / i, nums);
        }
    }

    // Check prime
    egz_prime(n, nums)
}

// Reference: https://arxiv.org/abs/2208.07728
// Reference: https://infossm.github.io/blog/2020/03/18/mult-n/
// Reference: https://infossm.github.io/blog/2023/07/22/egz/
fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let n = scan.token::<usize>();
    let mut nums = vec![0; 2 * n - 1];

    for i in 0..2 * n - 1 {
        nums[i] = scan.token::<i64>();
    }

    let ret = egz(n, &nums);

    for i in 0..2 * n - 1 {
        if ret[i] {
            write!(out, "{} ", nums[i]).unwrap();
        }
    }

    writeln!(out).unwrap();
}
