use cmp::Ordering;
use io::Write;
use std::{cmp, collections::HashMap, io, str};
use Ordering::Less;

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

pub trait Ext {
    type Item;

    fn lower_bound(&self, x: &Self::Item) -> usize
    where
        Self::Item: Ord;

    fn lower_bound_by<'a, F>(&'a self, f: F) -> usize
    where
        F: FnMut(&'a Self::Item) -> Ordering;
}

impl<T> Ext for [T] {
    type Item = T;
    fn lower_bound(&self, x: &Self::Item) -> usize
    where
        T: Ord,
    {
        self.lower_bound_by(|y| y.cmp(x))
    }

    fn lower_bound_by<'a, F>(&'a self, mut f: F) -> usize
    where
        F: FnMut(&'a Self::Item) -> Ordering,
    {
        let s = self;
        let mut size = s.len();
        if size == 0 {
            return 0;
        }
        let mut base = 0usize;
        while size > 1 {
            let half = size / 2;
            let mid = base + half;
            let cmp = f(unsafe { s.get_unchecked(mid) });
            base = if cmp == Less { mid } else { base };
            size -= half;
        }
        let cmp = f(unsafe { s.get_unchecked(base) });
        base + (cmp == Less) as usize
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

fn compress(values: Vec<i64>) -> Vec<(i64, i64)> {
    let mut map = HashMap::new();

    for val in values {
        *map.entry(val).or_insert(0) += 1;
    }

    let mut ret = Vec::new();

    for val in map.iter() {
        ret.push((val.0.clone(), val.1.clone()));
    }

    ret
}

fn process_dfs(
    count: &mut HashMap<usize, usize>,
    primes: &Vec<(i64, i64)>,
    prime: &(i64, i64),
    size: usize,
    iter: usize,
    mut val: usize,
) {
    if iter == size {
        for _ in 0..prime.1 {
            val *= prime.0 as usize;
        }

        for i in (0..=(prime.1 - 1)).rev() {
            let key = val / prime.0 as usize;
            let value = *match count.get(&val) {
                Some(value) => value,
                None => &0,
            };

            if count.contains_key(&key) {
                *count.get_mut(&key).unwrap() += value;
            } else {
                count.insert(key, value);
            }

            if i > 0 {
                val /= prime.0 as usize;
            }
        }
    } else {
        if primes[iter].0 == prime.0 {
            process_dfs(count, primes, prime, size, iter + 1, val);
        } else {
            for i in 0..=primes[iter].1 {
                process_dfs(count, primes, prime, size, iter + 1, val);

                if i < primes[iter].1 {
                    val *= primes[iter].0 as usize;
                }
            }
        }
    }
}

extern "C" {
    fn rand() -> u32;
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let (n, k) = (scan.token::<usize>(), scan.token::<usize>());
    let mut arr = vec![0; n];
    let mut visited = vec![false; n];

    for i in 0..n {
        arr[i] = scan.token::<i64>();
    }

    let mut ans = 0;
    let iter = cmp::min(n, 20);

    for _ in 0..iter {
        let mut idx;

        loop {
            idx = unsafe { rand() as usize % n };

            if !visited[idx] {
                break;
            }
        }

        visited[idx] = true;

        let mut count = HashMap::new();
        for i in 0..n {
            let key = gcd(arr[i], arr[idx]) as usize;

            if count.contains_key(&key) {
                *count.get_mut(&key).unwrap() += 1;
            } else {
                count.insert(key, 1);
            }
        }

        let factors = factorize(arr[idx]);
        let compressed_factors = compress(factors);

        for factor in compressed_factors.iter() {
            process_dfs(
                &mut count,
                &compressed_factors,
                factor,
                compressed_factors.len(),
                0,
                1,
            );
        }

        for value in count.iter() {
            if *value.1 >= n - k {
                ans = cmp::max(ans, *value.0);
            }
        }
    }

    writeln!(out, "{}", ans).unwrap();
}
