use io::Write;
use std::{collections::HashSet, io, ops, str};

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

extern "C" {
    fn rand() -> u32;
}

fn gcd(first: i128, second: i128) -> i128 {
    let mut max = first;
    let mut min = second;

    if min == 0 && max == 0 {
        return 0;
    } else if min == 0 {
        return max;
    } else if max == 0 {
        return min;
    }

    if min > max {
        let val = max;

        max = min;
        min = val;
    }

    loop {
        let res = max % min;

        if res == 0 {
            return min;
        }

        max = min;
        min = res;
    }
}

fn multiply(x: i128, y: i128, modular: i128) -> i128 {
    (x as i128 * y as i128 % modular as i128) as i128
}

fn pow(x: i128, mut y: i128, p: i128) -> i128 {
    let mut ret = 1;
    let mut piv = x % p;

    while y != 0 {
        if y & 1 != 0 {
            ret = multiply(ret, piv, p);
        }

        piv = multiply(piv, piv, p);
        y >>= 1;
    }

    ret
}

fn process_miller_rabin(x: i128, a: i128) -> bool {
    if x % a == 0 {
        return false;
    }

    let mut d = x - 1;

    loop {
        let tmp = pow(a, d, x);

        if d & 1 != 0 {
            return tmp != 1 && tmp != x - 1;
        } else if tmp == x - 1 {
            return false;
        }

        d >>= 1;
    }
}

fn is_prime(x: i128) -> bool {
    for val in [2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37] {
        if x == val {
            return true;
        }

        if x > 40 && process_miller_rabin(x, val) {
            return false;
        }
    }

    x > 40
}

fn record(num: i128, values: &mut Vec<i128>) {
    if num == 1 {
        return;
    }

    if num % 2 == 0 {
        values.push(2);
        record(num / 2, values);
        return;
    }

    if is_prime(num) {
        values.push(num);
        return;
    }

    let mut a = 0;
    let mut b = 0;
    let mut c = 0;
    let mut g = num;

    let func = |x, c| {
        return (c + multiply(x, x, num)) % num;
    };

    loop {
        if g == num {
            a = unsafe { rand() as i128 % (num - 2) + 2 };
            b = a;
            c = unsafe { rand() as i128 % 20 + 1 };
        }

        a = func(a, c);
        b = func(func(b, c), c);
        g = gcd((a - b).abs(), num);

        if g != 1 {
            break;
        }
    }

    record(g, values);
    record(num / g, values);
}

fn factorize(num: i128) -> Vec<i128> {
    let mut ret = Vec::new();

    record(num, &mut ret);

    ret.sort();
    ret
}

#[derive(Clone)]
struct GaussianInteger {
    real: i128,
    imaginary: i128,
}

impl GaussianInteger {
    pub fn new(real: i128, imaginary: i128) -> Self {
        Self { real, imaginary }
    }

    fn get_remainder(a: i128, b: i128) -> i128 {
        let mut ret = a % b;

        if ret < 0 {
            ret += b;
        }
        if 2 * ret > b {
            ret -= b;
        }

        ret
    }

    fn get_quotient(a: i128, b: i128) -> i128 {
        (a - GaussianInteger::get_remainder(a, b)) / b
    }

    pub fn gcd(mut w: GaussianInteger, mut z: GaussianInteger) -> GaussianInteger {
        while z.real != 0 || z.imaginary != 0 {
            let new_w = z.clone();
            let new_z = w % z;
            w = new_w;
            z = new_z;
        }

        w
    }
}

impl ops::Div<GaussianInteger> for GaussianInteger {
    type Output = GaussianInteger;

    fn div(self, rhs: GaussianInteger) -> Self::Output {
        let (w0, w1) = (self.real, self.imaginary);
        let (z0, z1) = (rhs.real, rhs.imaginary);

        let n = rhs.real * rhs.real + rhs.imaginary * rhs.imaginary;
        let (u0, u1) = (
            GaussianInteger::get_quotient(w0 * z0 + w1 * z1, n),
            GaussianInteger::get_quotient(w1 * z0 - w0 * z1, n),
        );

        GaussianInteger::new(u0, u1)
    }
}

impl ops::Rem<GaussianInteger> for GaussianInteger {
    type Output = GaussianInteger;

    fn rem(self, rhs: Self) -> Self::Output {
        let (a0, a1) = (self.real, self.imaginary);
        let (b0, b1) = (rhs.real, rhs.imaginary);

        let q = self / rhs;
        let (q0, q1) = (q.real, q.imaginary);

        let r0 = a0 - q0 * b0 + q1 * b1;
        let r1 = a1 - q0 * b1 - q1 * b0;

        GaussianInteger::new(r0, r1)
    }
}

fn quadratic_residue(n: i128) -> i128 {
    let k = n / 4;
    let mut j = 2;

    loop {
        let a = pow(j, k, n);
        let b = a * a % n;

        if b == n - 1 {
            return a;
        }

        j += 1;
    }
}

fn process_sum_of_two_squares_prime(n: i128) -> (i128, i128) {
    if n == 2 {
        return (1, 1);
    }

    let a = quadratic_residue(n);
    let g = GaussianInteger::gcd(GaussianInteger::new(n, 0), GaussianInteger::new(a, 1));

    (g.real.abs(), g.imaginary.abs())
}

fn process_sum_of_two_squares(n: i128) -> Option<(i128, i128)> {
    let mut square = 1;
    let mut primes = HashSet::new();

    let nums = factorize(n);
    for val in nums.iter() {
        if primes.contains(val) {
            square *= val;
            primes.remove(val);
        } else {
            primes.insert(*val);
        }
    }

    if primes.is_empty() {
        return Some((square, 0));
    }

    for val in primes.iter() {
        if val % 4 == 3 {
            return None;
        }
    }

    let (mut a, mut b) = (square, 0);

    for val in primes.iter() {
        let (c, d) = process_sum_of_two_squares_prime(*val);
        let new_a = a * c + b * d;
        let new_b = a * d - b * c;
        a = new_a;
        b = new_b;
    }

    Some((a.abs(), b.abs()))
}

// Reference: http://www.secmem.org/blog/2019/10/18/sum-of-squares/
fn process_sum_of_squares(mut n: i128) -> Vec<i128> {
    if n == 0 {
        return Vec::new();
    }

    let mut cnt = 0;
    if n % 4 == 0 {
        while n % 4 == 0 {
            n /= 4;
            cnt += 1;
        }

        let mut ret = process_sum_of_squares(n);
        for val in ret.iter_mut() {
            *val <<= cnt;
        }
        return ret;
    }

    if n % 8 == 7 {
        let mut ret = process_sum_of_squares(n - 1);
        ret.push(1);
        return ret;
    }

    let mut ab = process_sum_of_two_squares(n);
    if let Some(ab) = ab {
        let (a, b) = ab;
        let mut ret = Vec::new();
        if a != 0 {
            ret.push(a);
        }
        if b != 0 {
            ret.push(b);
        }
        return ret;
    }

    let mut i = if n % 4 == 3 { 1 } else { 2 };

    loop {
        ab = process_sum_of_two_squares(n - i * i);

        if let Some(ab) = ab {
            let mut ret = vec![ab.0, ab.1];
            ret.push(i);
            return ret;
        }

        i += 2;
    }
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let n = scan.token::<i128>();
    let ret = process_sum_of_squares(n);

    writeln!(out, "{}", ret.len()).unwrap();
    for val in ret.iter() {
        write!(out, "{} ", val).unwrap();
    }
    writeln!(out).unwrap();
}
