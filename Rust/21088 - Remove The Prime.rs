use io::Write;
use std::{cmp, collections::HashSet, io, str};

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

fn gcd(first: i64, second: i64) -> i64 {
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

fn multiply(x: i64, y: i64, modular: i64) -> i64 {
    (x as i128 * y as i128 % modular as i128) as i64
}

fn pow(x: i64, mut y: i64, p: i64) -> i64 {
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

fn process_miller_rabin(x: i64, a: i64) -> bool {
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

fn is_prime(x: i64) -> bool {
    if x < 2 {
        return false;
    }
    if x == 2 || x == 3 {
        return true;
    }
    if x % 6 != 1 && x % 6 != 5 {
        return false;
    }

    let base = if x > 4_759_123_141 {
        vec![2, 325, 9375, 28178, 450775, 9780504, 1795265022]
    } else {
        vec![2, 7, 61]
    };

    for val in base.iter() {
        if x == *val {
            return true;
        }

        if process_miller_rabin(x, *val) {
            return false;
        }
    }

    true
}

// Reference: https://github.com/kth-competitive-programming/kactl/blob/main/content/number-theory/Factor.h
fn record(num: i64, values: &mut Vec<i64>) {
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

    let func = |x| {
        return multiply(x, x, num) + 1;
    };

    let mut x = 0;
    let mut y = 0;
    let mut t = 30;
    let mut prd = 2;
    let mut i = 1;
    let mut q;

    while t % 40 != 0 || gcd(prd, num) == 1 {
        t += 1;

        if x == y {
            i += 1;
            x = i;
            y = func(x);
        }

        q = multiply(prd, cmp::max(x, y) - cmp::min(x, y), num);
        if q != 0 {
            prd = q;
        }

        x = func(x);
        y = func(func(y));
    }

    let g = gcd(prd, num);

    record(g, values);
    record(num / g, values);
}

fn factorize(num: i64) -> Vec<i64> {
    let mut ret = Vec::new();

    record(num, &mut ret);

    ret.sort();
    ret
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let n = scan.token::<usize>();
    let mut arr = vec![0; n];

    for i in 0..n {
        arr[i] = scan.token::<i64>();
    }

    let mut values = HashSet::new();

    for i in 0..n {
        let ret = factorize(arr[i]);

        for factor in ret {
            values.insert(factor);
        }
    }

    let mut ans = 0;

    for value in values {
        let mut len = 0;

        for i in 0..n {
            if multiply(arr[i], 1, value) == 0 {
                len += 1;
            } else {
                ans ^= len;
                len = 0;
            }
        }

        ans ^= len;
    }

    writeln!(out, "{}", if ans > 0 { "First" } else { "Second" }).unwrap();
}
