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

    pub fn all(&mut self) -> String {
        let mut input = String::new();
        self.reader.read_to_string(&mut input).expect("Failed read");
        input
    }

    pub fn line(&mut self) -> String {
        let mut input = String::new();
        self.reader.read_line(&mut input).expect("Failed read");
        input
    }
}

fn gcd_extended(a: i64, b: i64) -> (i64, i64, i64) {
    if b == 0 {
        (a, 1, 0)
    } else {
        let (g, x, y) = gcd_extended(b, a % b);
        (g, y, x - (a / b) * y)
    }
}

fn inv_pow2(val: i64, exp: u32) -> i64 {
    let mask = 1i64 << exp;
    let (_, inv, _) = gcd_extended(val, mask as i64);

    let mut inv = inv % (mask as i64);

    if inv < 0 {
        inv += mask as i64;
    }

    inv
}

fn subset_sums(vals: &[i64], mask: i64) -> Vec<i64> {
    let mut sums = vec![0];

    for &val in vals {
        for i in 0..sums.len() {
            let s = sums[i];
            sums.push((s + val) & mask);
        }
    }

    sums.sort_unstable();
    sums.dedup();
    sums
}

fn lower_bound(v: &Vec<i64>, x: i64) -> usize {
    let mut left = 0;
    let mut right = v.len();

    while left < right {
        let mid = (left + right) >> 1;

        if v[mid] < x {
            left = mid + 1;
        } else {
            right = mid;
        }
    }

    left
}

fn calculate(x: i64) -> (usize, Vec<i64>) {
    if x == 0 {
        return (1, vec![2]);
    }

    let tz = x.trailing_zeros();
    let odd = x >> tz;
    let len = 64 - odd.leading_zeros();
    let pow2 = 1i64 << len;
    let mask = pow2 - 1;

    if odd == mask {
        let a = pow2;
        return (2, vec![a, a + 1]);
    }

    let inv = inv_pow2(odd, len);
    let mut zeros = Vec::new();

    for t in 0..len {
        if ((odd >> t) & 1) == 0 {
            zeros.push(t);
        }
    }

    let base = zeros
        .iter()
        .map(|&t| ((inv << t) & mask))
        .collect::<Vec<_>>();
    let mid = base.len() / 2;
    let left = subset_sums(&base[..mid], mask);
    let right = subset_sums(&base[mid..], mask);

    let mut ret = pow2;

    for &val in left.iter() {
        if val >= 2 && val < ret {
            ret = val;
        }
    }

    for &val in right.iter() {
        if val >= 2 && val < ret {
            ret = val;
        }
    }

    for &val in right.iter() {
        let need = if val >= 2 { 0 } else { 2 - val };
        let idx = lower_bound(&left, need);

        if idx < left.len() {
            let v = left[idx] + val;

            if v < pow2 && v >= 2 && v < ret {
                ret = v;
            }
        }

        let need = pow2 - val + 2;
        let idx = lower_bound(&left, need);

        if idx < left.len() {
            let v = left[idx] + val - pow2;

            if v >= 2 && v < ret {
                ret = v;
            }
        }
    }

    (2, vec![ret, ret + 1])
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let t = scan.token::<i64>();

    for _ in 0..t {
        let x = scan.token::<i64>();
        let ret = calculate(x);

        writeln!(out, "{}", ret.0).unwrap();

        for val in ret.1 {
            write!(out, "{val} ").unwrap();
        }

        writeln!(out).unwrap();
    }
}
