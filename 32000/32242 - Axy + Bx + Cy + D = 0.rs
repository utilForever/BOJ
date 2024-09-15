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

    pub fn line(&mut self) -> String {
        let mut input = String::new();
        self.reader.read_line(&mut input).expect("Failed read");
        input
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

    let func = |x, c| (c + multiply(x, x, num)) % num;

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

fn process(nums: &Vec<(i64, i64)>, ret: &mut Vec<i64>, curr: i64, cnt: usize) {
    if cnt == nums.len() {
        ret.push(curr);
        return;
    }

    process(nums, ret, curr, cnt + 1);

    let mut tmp = 1;

    for _ in 1..=nums[cnt].1 {
        tmp *= nums[cnt].0;
        process(nums, ret, curr * tmp, cnt + 1);
    }
}

fn get_divisors(n: i64) -> Vec<i64> {
    let factors = factorize(n);
    let mut nums = Vec::new();
    let mut ret = Vec::new();

    for (key, val) in factors {
        nums.push((key, val));
    }

    process(&nums, &mut ret, 1, 0);

    ret
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let (a, b, c, d) = (
        scan.token::<i64>(),
        scan.token::<i64>(),
        scan.token::<i64>(),
        scan.token::<i64>(),
    );

    if a == 0 && b == 0 && c == 0 {
        writeln!(out, "{}", if d == 0 { "INFINITY" } else { "0" }).unwrap();
        return;
    }

    if a == 0 && b == 0 && c != 0 {
        writeln!(out, "{}", if d % c == 0 { "INFINITY" } else { "0" }).unwrap();
        return;
    }

    if a == 0 && b != 0 && c == 0 {
        writeln!(out, "{}", if d % b == 0 { "INFINITY" } else { "0" }).unwrap();
        return;
    }

    if a == 0 && b != 0 && c != 0 {
        writeln!(out, "{}", if d % gcd(b, c) == 0 { "INFINITY" } else { "0" }).unwrap();
        return;
    }

    if a != 0 && b == 0 && c == 0 {
        if d % a == 0 {
            if d == 0 {
                writeln!(out, "INFINITY").unwrap();
            } else {
                let e = (d / a).abs();
                let mut divisors = get_divisors(e);
                let n = divisors.len();

                for i in 0..n {
                    divisors.push(-divisors[i]);
                }

                divisors.sort();

                writeln!(out, "{}", divisors.len()).unwrap();

                for i in 0..divisors.len() {
                    writeln!(out, "{} {}", divisors[i], -d / a / divisors[i]).unwrap();
                }
            }
        } else {
            writeln!(out, "0").unwrap();
        }

        return;
    }

    if a != 0 && b == 0 && c != 0 {
        if d == 0 {
            writeln!(out, "INFINITY").unwrap();
            return;
        }

        let mut divisors = get_divisors(d.abs());
        let n = divisors.len();

        for i in 0..n {
            divisors.push(-divisors[i]);
        }

        let mut ret = Vec::new();

        for i in 0..divisors.len() {
            if (divisors[i] - c) % a == 0 {
                ret.push(((divisors[i] - c) / a, -d / divisors[i]));
            }
        }

        ret.sort();

        writeln!(out, "{}", ret.len()).unwrap();

        for (x, y) in ret {
            writeln!(out, "{x} {y}").unwrap();
        }

        return;
    }

    if a != 0 && b != 0 && c == 0 {
        if d == 0 {
            writeln!(out, "INFINITY").unwrap();
            return;
        }

        let mut divisors = get_divisors(d.abs());
        let n = divisors.len();

        for i in 0..n {
            divisors.push(-divisors[i]);
        }

        let mut ret = Vec::new();

        for i in 0..divisors.len() {
            if (divisors[i] - b) % a == 0 {
                ret.push((-d / divisors[i], (divisors[i] - b) / a));
            }
        }

        ret.sort();

        writeln!(out, "{}", ret.len()).unwrap();

        for (x, y) in ret {
            writeln!(out, "{x} {y}").unwrap();
        }

        return;
    }

    if a != 0 && b != 0 && c != 0 {
        if b * c == a * d {
            writeln!(
                out,
                "{}",
                if b % a == 0 || c % a == 0 {
                    "INFINITY"
                } else {
                    "0"
                }
            )
            .unwrap();
            return;
        }

        let mut ret = Vec::new();
        let mut divisors = get_divisors((b * c - a * d).abs());
        let n = divisors.len();

        for i in 0..n {
            divisors.push(-divisors[i]);
        }

        for i in 0..divisors.len() {
            let x = divisors[i];
            let y = (b * c - a * d) / x;

            if (x - c) % a == 0 && (y - b) % a == 0 {
                ret.push(((x - c) / a, (y - b) / a));
            }
        }

        ret.sort();

        writeln!(out, "{}", ret.len()).unwrap();

        for (x, y) in ret {
            writeln!(out, "{x} {y}").unwrap();
        }

        return;
    }
}
