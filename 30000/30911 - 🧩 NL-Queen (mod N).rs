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

fn gcd(first: usize, second: usize) -> usize {
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

fn find_factor(n: usize) -> usize {
    if n % 5 == 0 {
        return 5;
    }

    let mut d = 5;

    while d * d <= n {
        if n % d == 0 {
            return d;
        }

        d += 2;
    }

    n
}

fn build_composite(n: usize) -> Vec<usize> {
    let factor = find_factor(n);
    let m = n / factor;

    let mut sigma = vec![0; factor];
    let mut rsq_mod_m = vec![0; factor];
    let mut ret = vec![0; n];

    for r in 0..factor {
        sigma[r] = (2 * r) % factor;
        rsq_mod_m[r] = (r * r) % m;
    }

    for q in 0..m {
        let two_q = (2 * q) % m;

        for r in 0..factor {
            let i = r + factor * q;
            let inner = (two_q + rsq_mod_m[r]) % m;
            let a0 = sigma[r] + factor * inner;

            ret[i] = a0 + 1;
        }
    }

    ret
}

fn build_prime(p: usize) -> Option<Vec<usize>> {
    let mut is_qr = vec![false; p];

    for t in 1..=((p - 1) / 2) {
        is_qr[(t * t) % p] = true;
    }

    let mut buckets = [Vec::new(), Vec::new(), Vec::new(), Vec::new()];

    for a in 2..=p - 2 {
        if !is_qr[a] {
            continue;
        }

        let chi_ap1 = if is_qr[(a + 1) % p] { 1 } else { -1 };
        let chi_am1 = if is_qr[(a + p - 1) % p] { 1 } else { -1 };
        let idx = ((if chi_ap1 == 1 { 1 } else { 0 }) << 1) | (if chi_am1 == 1 { 1 } else { 0 });

        buckets[idx].push(a);
    }

    let (mut val_a, mut val_b) = (None, 0);

    for bucket in buckets.iter() {
        if bucket.len() >= 2 {
            val_a = Some(bucket[0]);
            val_b = bucket[1];
            break;
        }
    }

    let val_a = match val_a {
        Some(val) => val,
        None => return None,
    };

    let mut ret = vec![0; p];
    ret[0] = 1;

    for x in 1..p {
        let slope = if is_qr[x] { val_a } else { val_b };
        let f_x = (((slope as u128) * (x as u128)) % (p as u128)) as usize;

        ret[x] = f_x + 1;
    }

    Some(ret)
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let n = scan.token::<usize>();

    if gcd(n, 6) != 1 {
        writeln!(out, "-1").unwrap();
        return;
    }

    let factor = find_factor(n);

    if factor < n {
        let ret = build_composite(n);

        for val in ret {
            write!(out, "{val} ").unwrap();
        }

        writeln!(out).unwrap();
    } else {
        let ret = build_prime(n);

        match ret {
            Some(arr) => {
                for val in arr {
                    write!(out, "{val} ").unwrap();
                }

                writeln!(out).unwrap();
            }
            None => {
                writeln!(out, "-1").unwrap();
            }
        }
    }
}
