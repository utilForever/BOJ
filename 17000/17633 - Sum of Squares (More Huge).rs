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
        if values.contains_key(&2) {
            *values.get_mut(&2).unwrap() += 1;
        } else {
            values.insert(2, 1);
        }

        record(num / 2, values);
        return;
    }

    if is_prime(num) {
        if values.contains_key(&num) {
            *values.get_mut(&num).unwrap() += 1;
        } else {
            values.insert(num, 1);
        }

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

// Reference: http://www.secmem.org/blog/2019/10/18/sum-of-squares/
fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let mut n = scan.token::<i64>();

    while n % 4 == 0 {
        n /= 4;
    }

    if n % 8 == 7 {
        writeln!(out, "4").unwrap();
        return;
    }

    let ret = factorize(n);
    let new_ret: HashMap<i64, i64> = ret
        .iter()
        .filter_map(|(key, val)| {
            if *val % 2 == 0 {
                return None;
            } else {
                Some((*key, *val))
            }
        })
        .collect();

    for (val, _) in new_ret.iter() {
        if val % 4 == 3 {
            writeln!(out, "3").unwrap();
            return;
        }
    }

    let val = (n as f64).sqrt() as i64;
    if val * val != n {
        writeln!(out, "2").unwrap();
    } else {
        writeln!(out, "1").unwrap();
    }
}
