use io::Write;
use std::{collections::HashMap, io, str};

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

const MOD: i64 = 1_000_000_007;

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

fn pow(x: i64, mut y: i64, p: i64) -> i64 {
    let mut ret = 1;
    let mut piv = x % p;

    while y != 0 {
        if y & 1 != 0 {
            ret = ret * piv % p;
        }

        piv = piv * piv % p;
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
    for val in [2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37] {
        if x == val {
            return true;
        }

        if process_miller_rabin(x, val) {
            return false;
        }
    }

    true
}

fn record(num: i64, values: &mut HashMap<i64, i64>) {
    if num == 1 {
        return;
    }

    if num % 2 == 0 {
        if let std::collections::hash_map::Entry::Vacant(e) = values.entry(2) {
            e.insert(1);
        } else {
            *values.get_mut(&2).unwrap() += 1;
        }

        record(num / 2, values);
        return;
    }

    if is_prime(num) {
        if let std::collections::hash_map::Entry::Vacant(e) = values.entry(num) {
            e.insert(1);
        } else {
            *values.get_mut(&num).unwrap() += 1;
        }

        return;
    }

    let mut a = 0;
    let mut b = 0;
    let mut c = 0;
    let mut g = num;

    let func = |x, c| (c + (x * x) % num) % num;

    loop {
        if g == num {
            a = unsafe { rand() as i64 % (num - 2) + 2 };
            b = a;
            c = unsafe { rand() as i64 % 20 + 1 };
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

fn factorize(num: i64) -> HashMap<i64, i64> {
    let mut ret = HashMap::new();

    record(num, &mut ret);

    ret
}

fn calculate_k(n: i64, num: i64) -> i64 {
    let factors = factorize(num);
    let mut map = HashMap::new();
    map.insert(1, 1);

    for (key, val) in factors {
        let mut temp = map.clone();

        for (k, v) in map.iter() {
            for j in 0..val {
                let new_key = key.pow((j + 1) as u32) * k;
                let new_val = key.pow(j as u32) * (key - 1) * v;

                temp.entry(new_key)
                    .and_modify(|e| *e = new_val)
                    .or_insert(new_val);
            }
        }

        map = temp;
    }

    let mut k = 0;

    for (key, val) in map {
        if key == 1 {
            k += 1;
            continue;
        }

        if key % 2 == 0 {
            continue;
        }

        k += val
            * pow(
                2,
                (num / key * (1 - key) * pow(num, n - 1, MOD - 1)).rem_euclid(MOD - 1),
                MOD,
            );
        k %= MOD;
    }

    pow(num, MOD - 2, MOD) * k % MOD
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let (n, m) = (scan.token::<i64>(), scan.token::<i64>());
    let k_orig = calculate_k(n, m);

    for i in 1..m {
        if k_orig == calculate_k(n, i) {
            writeln!(out, "{i}").unwrap();
            return;
        }
    }

    writeln!(out, "Smart Oldbie").unwrap();
}
