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

fn mobius(n: usize) -> Vec<i64> {
    let mut ret = vec![0; n + 1];
    let mut is_prime = vec![true; n + 1];
    let mut primes = Vec::new();

    if n >= 1 {
        ret[1] = 1;
        is_prime[0] = false;

        if n >= 1 {
            is_prime[1] = false;
        }
    }

    for i in 2..=n {
        if is_prime[i] {
            primes.push(i);
            ret[i] = -1;
        }

        for &prime in primes.iter() {
            let val = i * prime;

            if val > n {
                break;
            }

            is_prime[val] = false;

            if i % prime == 0 {
                ret[val] = 0;
                break;
            } else {
                ret[val] = -ret[i];
            }
        }
    }

    ret
}

fn gcd(mut first: i64, mut second: i64) -> i64 {
    if first < 0 {
        first = -first;
    }

    if second < 0 {
        second = -second;
    }

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
        std::mem::swap(&mut min, &mut max);
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

const EPS: f64 = 1e-12;

fn count_less(mobius: &Vec<i64>, x: f64, n: usize) -> i64 {
    let mut h = vec![0; n + 1];
    let mut sum = 0;

    for k in 1..=n {
        let v = ((x * k as f64) + EPS).floor() as i64;

        sum += v;
        h[k] = sum;
    }

    let mut ret = 0;

    for d in 1..=n {
        let m = n / d;
        ret += mobius[d] * h[m];
    }

    ret
}

fn calculate(x: f64, n: usize) -> (i64, i64) {
    let mut best_p = 0;
    let mut best_q = 1;
    let mut best_val = 0.0;

    for q in 2..=n {
        let mut p = ((x * q as f64) + EPS).floor() as i64;

        if p <= 0 {
            continue;
        }

        while p > 0 {
            if gcd(p, q as i64) == 1 {
                let val = (p as f64) / (q as f64);

                if val > best_val {
                    best_val = val;
                    best_p = p;
                    best_q = q as i64;
                }

                break;
            }

            p -= 1;
        }
    }

    (best_p, best_q)
}

// Reference: https://en.wikipedia.org/wiki/Farey_sequence
fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let (n, k) = (scan.token::<usize>(), scan.token::<i64>());
    let mobius = mobius(n);

    let mut left = 0.0;
    let mut right = 1.0;

    for _ in 0..50 {
        let mid = (left + right) / 2.0;
        let cnt = count_less(&mobius, mid, n);

        if cnt < k {
            left = mid;
        } else {
            right = mid;
        }
    }

    let (p, q) = calculate(right, n);

    writeln!(out, "{p} {q}").unwrap();
}
