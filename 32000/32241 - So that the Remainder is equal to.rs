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
}

extern "C" {
    fn rand() -> u32;
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

const MOD: i64 = 1_000_000_007;

fn modmul(a: i64, b: i64, modular: i64) -> i64 {
    let c = a as i128 * b as i128;
    (c % modular as i128) as i64
}

fn modexp(mut a: i64, mut b: i64, modular: i64) -> i64 {
    let mut ret = 1;

    while b > 0 {
        if b % 2 != 0 {
            ret = modmul(ret, a, modular);
            b -= 1;
        }

        a = modmul(a, a, modular);
        b /= 2;
    }

    ret
}

fn trial(num: i64, modular: i64) -> bool {
    if num % modular == 0 {
        return false;
    }

    let mut cnt = -1;
    let mut d = num - 1;

    while d % 2 == 0 {
        d /= 2;
        cnt += 1;
    }

    let mut p = modexp(modular, d, num);

    if p == 1 || p == num - 1 {
        return true;
    }

    while cnt > 0 {
        p = modmul(p, p, num);

        if p == num - 1 {
            return true;
        }

        cnt -= 1;
    }

    false
}

fn is_prime(num: i64) -> bool {
    if num == 1 {
        return false;
    }

    let test_cases = [2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37];

    for val in test_cases {
        if num == val {
            return true;
        }

        if num > 40 && !trial(num, val) {
            return false;
        }
    }

    num > 40
}

fn pollard_rho(num: i64) -> i64 {
    let mut x = unsafe { rand() as i64 % (num - 2) } + 2;
    let mut y = x;
    let c = unsafe { rand() as i64 % (num - 1) } + 1;

    loop {
        x = modmul(x, x, num) + c;

        if x >= num {
            x -= num;
        }

        y = modmul(y, y, num) + c;

        if y >= num {
            y -= num;
        }

        y = modmul(y, y, num) + c;

        if y >= num {
            y -= num;
        }

        let d = gcd((x - y).abs(), num);

        if d == 1 {
            continue;
        }

        if !is_prime(d) {
            return pollard_rho(d);
        } else {
            return d;
        }
    }
}

fn get_factors(mut num: i64, primes: &mut Vec<i64>) {
    if num % 2 == 0 {
        primes.push(2);
    }

    while num % 2 == 0 {
        num /= 2;
    }

    while num != 1 && !is_prime(num) {
        let d = pollard_rho(num);

        if num % d == 0 {
            primes.push(d);
        }

        while num % d == 0 {
            num /= d;
        }
    }

    if num != 1 {
        primes.push(num);
    }
}

fn vp(p: i64, mut val: i64) -> i64 {
    let mut ret = 0;

    while val % p == 0 {
        val /= p;
        ret += 1;
    }

    ret
}

fn calculate(a: i64, b: i64) -> i64 {
    if b == 0 {
        return 1;
    }

    let tmp = calculate(a, b / 2);
    
    tmp * tmp % MOD * (if b % 2 == 1 { a } else { 1 }) % MOD
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let (n, k) = (scan.token::<usize>(), scan.token::<i64>());
    let mut nums = vec![0; n];

    for i in 0..n {
        nums[i] = scan.token::<i64>();
    }

    let mut gcd_val = nums[1] - nums[0];

    for i in 2..n {
        gcd_val = gcd(gcd_val, nums[i] - nums[i - 1]);
    }

    let mut tmp = gcd(gcd_val, nums[n - 1]);

    while tmp != 1 && gcd_val % tmp == 0 {
        gcd_val /= tmp;
        tmp = gcd(gcd_val, nums[n - 1]);
    }

    if gcd_val == 1 || k == 1 {
        writeln!(out, "1").unwrap();
        return;
    }

    let mut primes = Vec::new();
    get_factors(gcd_val, &mut primes);

    primes.sort();

    let mut prime_cnt = vec![0; primes.len()];
    let mut ret = 1;
    let mut check = false;

    if primes[0] == 2 {
        while gcd_val % 2 == 0 {
            gcd_val /= 2;
            prime_cnt[0] += 1;
        }

        tmp = 0;

        for i in 1..n {
            let a = nums[i - 1].abs();
            let b = nums[i].abs();

            if nums[i - 1] <= 0 && nums[i] >= 0 {
                if k * prime_cnt[0] == 1 {
                    tmp = tmp.max(1);
                } else {
                    writeln!(out, "-1").unwrap();
                    return;
                }
            } else {
                let apb = vp(2, a + b);
                let amb = vp(2, (a - b).abs());

                tmp = tmp.max((prime_cnt[0] * k - amb - apb + 1).max(1));
            }
        }

        if tmp > 0 {
            check = true;
        }

        ret = calculate(2, tmp);
    }

    for i in if primes[0] == 2 { 1 } else { 0 }..primes.len() {
        while gcd_val % primes[i] == 0 {
            gcd_val /= primes[i];
            prime_cnt[i] += 1;
        }

        tmp = 0;

        for j in 1..n {
            let a = nums[j - 1].abs();
            let b = nums[j].abs();
            let apb = vp(primes[i], a + b);
            let amb = vp(primes[i], (a - b).abs());

            if nums[j - 1] <= 0 && nums[j] >= 0 {
                if check {
                    writeln!(out, "-1").unwrap();
                    return;
                } else {
                    tmp = tmp.max(k * prime_cnt[i] - apb);
                }
            } else {
                tmp = tmp.max(k * prime_cnt[i] - amb);
            }
        }

        ret = ret * calculate(primes[i] % MOD, tmp) % MOD;
    }

    writeln!(out, "{ret}").unwrap();
}
