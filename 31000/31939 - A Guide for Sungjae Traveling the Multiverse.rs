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

const MOD: i64 = 998_244_353;
const G: i64 = 3;

fn mod_inv(a: i64) -> i64 {
    mod_pow(a, MOD - 2)
}

fn mod_pow(mut base: i64, mut exp: i64) -> i64 {
    base %= MOD;

    if base < 0 {
        base += MOD;
    }

    let mut ret = 1;

    while exp > 0 {
        if exp % 2 == 1 {
            ret = ret * base % MOD;
        }

        base = base * base % MOD;
        exp /= 2;
    }

    ret
}

fn ntt(a: &mut [i64], invert: bool) {
    let n = a.len();
    let mut j = 0;

    for i in 1..n {
        let mut bit = n >> 1;

        while j & bit != 0 {
            j ^= bit;
            bit >>= 1;
        }

        j ^= bit;

        if i < j {
            a.swap(i, j);
        }
    }

    let mut len = 2;

    while len <= n {
        let w_len = if invert {
            mod_inv(mod_pow(G, (MOD - 1) / len as i64))
        } else {
            mod_pow(G, (MOD - 1) / len as i64)
        };

        for i in (0..n).step_by(len) {
            let mut w = 1i64;

            for j in 0..len / 2 {
                let u = a[i + j];
                let v = a[i + j + len / 2] * w % MOD;

                a[i + j] = (u + v) % MOD;
                a[i + j + len / 2] = (u - v + MOD) % MOD;
                w = w * w_len % MOD;
            }
        }

        len <<= 1;
    }

    if invert {
        let n_inv = mod_inv(n as i64);

        for x in a.iter_mut() {
            *x = *x * n_inv % MOD;
        }
    }
}

#[derive(Clone)]
struct Poly {
    real: Vec<i64>,
    imag: Vec<i64>,
}

impl Poly {
    fn mul(&self, other: &Poly) -> Poly {
        let n = self.real.len() + other.real.len() - 1;
        let size = n.next_power_of_two();

        let mut fa_real = self.real.clone();
        fa_real.resize(size, 0);
        let mut fa_imag = self.imag.clone();
        fa_imag.resize(size, 0);
        let mut fb_real = other.real.clone();
        fb_real.resize(size, 0);
        let mut fb_imag = other.imag.clone();
        fb_imag.resize(size, 0);

        ntt(&mut fa_real, false);
        ntt(&mut fa_imag, false);
        ntt(&mut fb_real, false);
        ntt(&mut fb_imag, false);

        let mut real = vec![0i64; size];
        let mut imag = vec![0i64; size];

        for i in 0..size {
            let a = fa_real[i];
            let b = fa_imag[i];
            let c = fb_real[i];
            let d = fb_imag[i];

            real[i] = (a * c - b * d) % MOD;
            imag[i] = (a * d + b * c) % MOD;
        }

        ntt(&mut real, true);
        ntt(&mut imag, true);

        real.resize(n, 0);
        imag.resize(n, 0);

        for x in real.iter_mut() {
            if *x < 0 {
                *x += MOD;
            }
        }

        for x in imag.iter_mut() {
            if *x < 0 {
                *x += MOD;
            }
        }

        Poly { real, imag }
    }
}

fn poly_mul(polys: &[Poly]) -> Poly {
    if polys.len() == 1 {
        return polys[0].clone();
    }

    let mid = polys.len() / 2;
    let left = poly_mul(&polys[..mid]);
    let right = poly_mul(&polys[mid..]);

    left.mul(&right)
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());
    let (n, r) = (scan.token::<usize>(), scan.token::<i64>());
    let mut polys = Vec::with_capacity(n);

    for _ in 0..n {
        let (x, y) = (scan.token::<i64>(), scan.token::<i64>());
        let c0_real = (-x) % MOD;
        let c0_imag = (-y) % MOD;
        let poly = Poly {
            real: vec![c0_real, 1],
            imag: vec![c0_imag, 0],
        };

        polys.push(poly);
    }

    let poly_mul = poly_mul(&polys);
    let r_squared = r * r % MOD;
    let mut r_power = 1;
    let mut ret = 0;

    for k in 0..poly_mul.real.len() {
        let a_k_real = poly_mul.real[k] % MOD;
        let a_k_imag = poly_mul.imag[k] % MOD;
        let abs_squared = (a_k_real * a_k_real + a_k_imag * a_k_imag) % MOD;

        if abs_squared == 0 {
            r_power = r_power * r_squared % MOD;
            continue;
        }

        let denom_inv = mod_inv((k as i64 + 1) % MOD);
        let term = abs_squared * r_power % MOD * denom_inv % MOD;

        r_power = r_power * r_squared % MOD;
        ret = (ret + term) % MOD;
    }

    if ret < 0 {
        ret += MOD;
    }

    writeln!(out, "{ret}").unwrap();
}
