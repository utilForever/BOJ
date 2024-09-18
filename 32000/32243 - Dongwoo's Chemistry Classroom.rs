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

struct Rng([u64; 4]);

impl Rng {
    fn split_mix(v: u64) -> u64 {
        let mut z = v.wrapping_add(0x9e3779b97f4a7c15);

        z = (z ^ (z >> 30)).wrapping_mul(0xbf58476d1ce4e5b9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94d049bb133111eb);
        z ^ (z >> 31)
    }

    fn new() -> Self {
        let mut seed = 0;
        unsafe { std::arch::x86_64::_rdrand64_step(&mut seed) };

        let mut prev = seed;

        Self(std::array::from_fn(|_| {
            prev = Self::split_mix(prev);
            prev
        }))
    }

    fn next(&mut self, n: u64) -> u64 {
        let [x, y, z, c] = &mut self.0;
        let t = x.wrapping_shl(58) + *c;

        *c = *x >> 6;
        *x = x.wrapping_add(t);

        if *x < t {
            *c += 1;
        }

        *z = z.wrapping_mul(6906969069).wrapping_add(1234567);
        *y ^= y.wrapping_shl(13);
        *y ^= *y >> 17;
        *y ^= y.wrapping_shl(43);

        let base = x.wrapping_add(*y).wrapping_add(*z);
        ((base as u128 * n as u128) >> 64) as u64
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

fn process(arr: &Vec<(i64, i64)>, ret: &mut Vec<i64>, cnt: usize, val: i64) {
    if cnt == arr.len() {
        ret.push(val);
        return;
    }

    let mut tmp = val;

    for _ in 1..=arr[cnt].1 {
        tmp /= arr[cnt].0;
        ret.push(tmp);
    }

    process(arr, ret, cnt + 1, val);
}

fn get_phi_assist(p: i64, q: i64) -> i64 {
    let limit = 1_000_000_000_000_000_000;

    (p - 1) * pow(p, q - 1, limit)
}

fn get_phi(n: i64) -> i64 {
    let factors = factorize(n);
    let mut ret = 1;

    for (p, q) in factors {
        let tmp = get_phi_assist(p, q);
        ret *= tmp;
    }

    ret
}

fn get_lambda_assist(p: i64, q: i64) -> i64 {
    let limit = 1_000_000_000_000_000_000;

    if p != 2 {
        pow(p, q - 1, limit) * (p - 1)
    } else if q <= 2 {
        pow(2, q - 1, limit)
    } else {
        pow(2, q - 2, limit)
    }
}

fn get_lambda(n: i64) -> i64 {
    let factors = factorize(n);
    let mut ret = 1;

    for (p, q) in factors {
        let tmp = get_lambda_assist(p, q);
        ret = ret / gcd(ret, tmp) * tmp;
    }

    ret
}

fn get_orders(n: i64) -> Vec<i64> {
    let factors = factorize(n);
    let mut arr = Vec::new();
    let mut ret = Vec::new();

    for (a, b) in factors {
        arr.push((a, b));
    }

    process(&arr, &mut ret, 0, n);

    ret
}

fn get_primitive_root(m: i64, m_lambda: i64) -> i64 {
    let orders = get_orders(m_lambda);
    let mut rng = Rng::new();

    loop {
        let p = rng.next(m as u64) as i64;

        if gcd(p, m) != 1 {
            continue;
        }

        let mut check = true;

        for &order in orders.iter() {
            if pow(p, order, m) == 1 && order != m_lambda {
                check = false;
                break;
            }
        }

        if check {
            return p;
        }
    }
}

fn discrete_log_preprocess(
    map: &mut HashMap<i64, i64>,
    val: i64,
    m_phi: i64,
    modular: i64,
    m_lambda_sqrt: i64,
) {
    let val_inv = pow(val, m_phi - 1, modular);
    let mut curr = 1;

    for i in 0..=m_lambda_sqrt {
        if !map.contains_key(&curr) {
            map.insert(curr, i);
        } else {
            break;
        }

        curr = ((curr as i128 * val_inv as i128) % modular as i128) as i64;
    }
}

fn get_discrete_log(
    map: &HashMap<i64, i64>,
    a: i64,
    b: i64,
    modular: i64,
    m_lambda: i64,
    m_lambda_sqrt: i64,
    m_lambda_sqrt_over: i64,
    m_phi: i64,
) -> i64 {
    let p = pow(a, m_lambda_sqrt, modular);
    let mut curr = pow(b, m_phi - 1, modular);

    for i in 0..=m_lambda_sqrt_over {
        if let Some(&val) = map.get(&curr) {
            let mut ret = val as i128 + i as i128 * m_lambda_sqrt as i128;
            ret = ret % m_lambda as i128;

            if ret == 0 {
                ret = m_lambda as i128;
            }

            return ret as i64;
        }

        curr = ((curr as i128 * p as i128) % modular as i128) as i64;
    }

    -1
}

fn main() {
    let stdin = io::stdin();
    let mut scan = UnsafeScanner::new(stdin.lock());

    let m = scan.token::<i64>();
    let m_lambda = get_lambda(m);
    let m_lambda_sqrt = (m_lambda as f64 * 100.0).sqrt() as i64 + 1;
    let m_lambda_sqrt_over = (m_lambda as f64 / 100.0).sqrt() as i64 + 1;
    let m_phi = get_phi(m);

    let primitive_root = get_primitive_root(m, m_lambda);

    println!("{}", m_lambda + 1);

    let (n, k) = (scan.token::<i64>(), scan.token::<i64>());

    for i in 0..n {
        for _ in 0..i {
            print!("1 ");
        }

        print!("{primitive_root} ");

        for _ in i + 1..n + k {
            print!("1 ");
        }

        println!();
    }

    for i in 0..k {
        for _ in 0..n + i {
            print!("1 ");
        }

        print!("{primitive_root} ");

        for _ in i + 1..k {
            print!("1 ");
        }

        println!();
    }

    let mut map = HashMap::new();
    discrete_log_preprocess(&mut map, primitive_root, m_phi, m, m_lambda_sqrt);

    for _ in 0..n {
        let p = scan.token::<i64>();
        let mut ret = m_lambda
            - get_discrete_log(
                &map,
                primitive_root,
                p,
                m,
                m_lambda,
                m_lambda_sqrt,
                m_lambda_sqrt_over,
                m_phi,
            );

        if ret == 0 {
            ret = m_lambda;
        }

        print!("{ret} ");
    }

    for _ in 0..k {
        let p = scan.token::<i64>();
        print!(
            "{} ",
            get_discrete_log(
                &map,
                primitive_root,
                p,
                m,
                m_lambda,
                m_lambda_sqrt,
                m_lambda_sqrt_over,
                m_phi
            )
        );
    }

    println!();
}
