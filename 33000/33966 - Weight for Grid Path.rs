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

// Reference: https://github.com/kenkoooo/competitive-programming-rs/blob/master/src/math/fast_fourier_transform.rs
pub struct FastFourierTransform {
    modulo: i64,
    sum_e: [i64; 30],
    sum_ie: [i64; 30],
}

impl FastFourierTransform {
    pub fn new(modulo: i64) -> Self {
        let primitive_root = primitive_root(modulo);

        let mut es = [0; 30];
        let mut ies = [0; 30];
        let count2 = (modulo - 1).trailing_zeros();
        let mut e = mod_pow(primitive_root, (modulo - 1) >> count2, modulo);
        let mut ie = mod_inv(e, modulo);
        let count2 = count2 as usize;
        for i in (2..=count2).rev() {
            es[i - 2] = e;
            ies[i - 2] = ie;
            e = (e * e) % modulo;
            ie = (ie * ie) % modulo;
        }

        let mut sum_e = [0; 30];
        let mut now = 1;
        for i in 0..=(count2 - 2) {
            sum_e[i] = (es[i] * now) % modulo;
            now = (now * ies[i]) % modulo;
        }

        let mut es = [0; 30];
        let mut ies = [0; 30];
        let count2 = (modulo - 1).trailing_zeros();
        let mut e = mod_pow(primitive_root, (modulo - 1) >> count2, modulo);
        let mut ie = mod_inv(e, modulo);
        let count2 = count2 as usize;
        for i in (2..=count2).rev() {
            es[i - 2] = e;
            ies[i - 2] = ie;
            e = (e * e) % modulo;
            ie = (ie * ie) % modulo;
        }

        let mut sum_ie = [0; 30];
        let mut now = 1;
        for i in 0..=(count2 - 2) {
            sum_ie[i] = (ies[i] * now) % modulo;
            now = (now * es[i]) % modulo;
        }

        Self {
            sum_e,
            modulo,
            sum_ie,
        }
    }
    fn butterfly(&self, a: &mut [i64]) {
        let h = a.len().next_power_of_two().trailing_zeros();
        for ph in 1..=h {
            let w = 1 << (ph - 1);
            let p = 1 << (h - ph);
            let mut now = 1;
            for s in 0..w {
                let offset = s << (h - ph + 1);
                for i in 0..p {
                    let l = a[i + offset];
                    let r = (a[i + offset + p] * now) % self.modulo;

                    a[i + offset] = l + r;
                    if a[i + offset] >= self.modulo {
                        a[i + offset] -= self.modulo;
                    }

                    a[i + offset + p] = l + self.modulo - r;
                    if a[i + offset + p] >= self.modulo {
                        a[i + offset + p] -= self.modulo;
                    }
                }

                now = (self.sum_e[(!s).trailing_zeros() as usize] * now) % self.modulo;
            }
        }
    }

    fn butterfly_inv(&self, a: &mut [i64]) {
        let h = a.len().next_power_of_two().trailing_zeros();
        for ph in (1..=h).rev() {
            let w = 1 << (ph - 1);
            let p = 1 << (h - ph);
            let mut inv_now = 1;
            for s in 0..w {
                let offset = s << (h - ph + 1);
                for i in 0..p {
                    let l = a[i + offset];
                    let r = a[i + offset + p];

                    a[i + offset] = l + r;
                    if a[i + offset] >= self.modulo {
                        a[i + offset] -= self.modulo;
                    }

                    a[i + offset + p] = ((l + self.modulo - r) * inv_now) % self.modulo;
                }

                inv_now = (self.sum_ie[(!s).trailing_zeros() as usize] * inv_now) % self.modulo;
            }
        }
    }

    pub fn convolution(&self, a: &[i64], b: &[i64]) -> Vec<i64> {
        if a.is_empty() || b.is_empty() {
            return Vec::new();
        }

        let n = a.len();
        let m = b.len();

        let z = (n + m - 1).next_power_of_two();
        let mut a = a.iter().map(|&v| v % self.modulo).collect::<Vec<_>>();
        a.resize(z, 0);
        self.butterfly(&mut a);

        let mut b = b.iter().map(|&v| v % self.modulo).collect::<Vec<_>>();
        b.resize(z, 0);
        self.butterfly(&mut b);

        for i in 0..z {
            a[i] = (a[i] * b[i]) % self.modulo;
        }

        self.butterfly_inv(&mut a);
        a.resize(n + m - 1, 0);
        let iz = mod_inv(z as i64, self.modulo);
        for i in 0..a.len() {
            a[i] = (a[i] * iz) % self.modulo;
        }
        a
    }
}

fn mod_inv(x: i64, m: i64) -> i64 {
    mod_pow(x, m - 2, m)
}

fn mod_pow(x: i64, mut e: i64, m: i64) -> i64 {
    let mut cur = x;
    let mut result = 1;
    while e > 0 {
        if e & 1 == 1 {
            result = (result * cur) % m;
        }
        e >>= 1;
        cur = (cur * cur) % m;
    }
    result
}

fn primitive_root(m: i64) -> i64 {
    if m == 2 {
        return 1;
    };
    if m == 167772161 {
        return 3;
    };
    if m == 469762049 {
        return 3;
    };
    if m == 754974721 {
        return 11;
    };
    if m == 998244353 {
        return 3;
    };
    let mut divs = [0; 20];
    divs[0] = 2;
    let mut cnt = 1;
    let mut x = (m - 1) / 2;
    while x % 2 == 0 {
        x /= 2
    }

    let mut i = 3;
    while i * i <= x {
        if x % i == 0 {
            divs[cnt] = i;
            cnt += 1;
            while x % i == 0 {
                x /= i;
            }
        }
        i += 2;
    }
    if x > 1 {
        divs[cnt] = x;
        cnt += 1;
    }

    for g in 2.. {
        let mut ok = true;
        for i in 0..cnt {
            if mod_pow(g, (m - 1) / divs[i], m) == 1 {
                ok = false;
                break;
            }
        }
        if ok {
            return g;
        }
    }
    unreachable!()
}

const MOD: i64 = 998_244_353;

fn pow(mut base: i64, mut exp: i64) -> i64 {
    let mut ret = 1;

    base %= MOD;

    while exp > 0 {
        if exp & 1 == 1 {
            ret = ret * base % MOD;
        }

        base = base * base % MOD;
        exp >>= 1;
    }

    ret
}

fn comb(fact: &Vec<i64>, fact_inv: &Vec<i64>, a: usize, b: usize) -> i64 {
    fact[a] * fact_inv[a - b] % MOD * fact_inv[b] % MOD
}

fn cdq(
    a: &Vec<i64>,
    b: &mut Vec<i64>,
    s: &mut Vec<i64>,
    kernel: &Vec<i64>,
    left: usize,
    right: usize,
    k: usize,
) {
    if right - left == 1 {
        if left == 0 {
            b[0] = a[0];
        } else if left >= k {
            b[left] = a[left] * s[left] % MOD;
        }

        return;
    }

    let mid = (left + right) / 2;

    cdq(a, b, s, kernel, left, mid, k);

    let len_left = mid - left;

    if len_left == 0 {
        return;
    }

    let len_k = (right - left).saturating_sub(k);

    if len_k == 0 {
        cdq(a, b, s, kernel, mid, right, k);
        return;
    }

    let vec_p = b[left..mid].to_vec();
    let vec_q = kernel[..len_k].to_vec();

    let fft = FastFourierTransform::new(MOD);
    let conv = fft.convolution(&vec_p, &vec_q);

    for (t, val) in conv.into_iter().enumerate() {
        let idx = left + k + t;

        if idx >= mid && idx < right {
            s[idx] = (s[idx] + val) % MOD;
        }
    }

    cdq(a, b, s, kernel, mid, right, k);
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let (n, k) = (scan.token::<usize>(), scan.token::<usize>());
    let mut a = vec![0; n + 1];

    for i in 0..=n {
        a[i] = scan.token::<i64>();
    }

    if a[0] == 0 {
        for _ in 0..n + 1 - k {
            write!(out, "0 ").unwrap();
        }

        writeln!(out).unwrap();
        return;
    }

    let mut fact = vec![1; 2 * n + 1];

    for i in 1..=2 * n {
        fact[i] = (fact[i - 1] * i as i64) % MOD;
    }

    let mut fact_inv = vec![1; 2 * n + 1];
    fact_inv[2 * n] = pow(fact[2 * n], MOD - 2);

    for i in (0..2 * n).rev() {
        fact_inv[i] = (fact_inv[i + 1] * (i as i64 + 1)) % MOD;
    }

    let mut kernel = vec![0; n + 1];
    kernel[0] = 1;

    for i in 1..=n - k {
        let num = comb(&fact, &fact_inv, 2 * i + 1, i);
        let denom_inv = pow(2 * i as i64 + 1, MOD - 2);

        kernel[i] = num * denom_inv % MOD;
    }

    let mut b = vec![0; n + 1];
    let mut s = vec![0; n + 1];

    cdq(&a, &mut b, &mut s, &kernel, 0, n + 1, k);

    let ret = (k..=n).map(|i| b[i]).collect::<Vec<_>>();

    for val in ret {
        write!(out, "{val} ").unwrap();
    }

    writeln!(out).unwrap();
}
