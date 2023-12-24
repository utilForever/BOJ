use io::Write;
use std::{
    f64::consts::PI,
    io,
    ops::{Add, AddAssign, Mul, MulAssign, Sub},
    str,
};

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

#[derive(Default, Clone)]
struct Complex {
    real: f64,
    imaginary: f64,
}

impl Complex {
    fn new(real: f64, imaginary: f64) -> Self {
        Self { real, imaginary }
    }
}

impl Add for Complex {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Complex {
            real: self.real + other.real,
            imaginary: self.imaginary + other.imaginary,
        }
    }
}

impl AddAssign for Complex {
    fn add_assign(&mut self, other: Self) {
        self.real += other.real;
        self.imaginary += other.imaginary;
    }
}

impl Sub for Complex {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Complex {
            real: self.real - other.real,
            imaginary: self.imaginary - other.imaginary,
        }
    }
}

impl Mul for Complex {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Complex {
            real: self.real * rhs.real - self.imaginary * rhs.imaginary,
            imaginary: self.real * rhs.imaginary + self.imaginary * rhs.real,
        }
    }
}

impl MulAssign for Complex {
    fn mul_assign(&mut self, rhs: Self) {
        let real = self.real * rhs.real - self.imaginary * rhs.imaginary;
        let imaginary = self.real * rhs.imaginary + self.imaginary * rhs.real;
        self.real = real;
        self.imaginary = imaginary;
    }
}

fn process_fft(arr: &mut Vec<Complex>, is_reverse: bool) {
    let n = arr.len();
    let (mut i, mut j) = (1, 0);

    while i < n {
        let mut bit = n >> 1;
        j ^= bit;

        while j & bit == 0 {
            bit >>= 1;
            j ^= bit;
        }

        if i < j {
            arr.swap(i, j);
        }

        i += 1;
    }

    i = 1;

    while i < n {
        let x = if is_reverse {
            PI / i as f64
        } else {
            -PI / i as f64
        };
        let w = Complex::new(f64::cos(x), f64::sin(x));

        j = 0;

        while j < n {
            let mut wp = Complex::new(1.0, 0.0);
            let mut k = 0;

            while k < i {
                let tmp = arr[i + j + k].clone() * wp.clone();
                arr[i + j + k] = arr[j + k].clone() - tmp.clone();
                arr[j + k] += tmp.clone();
                wp *= w.clone();

                k += 1;
            }

            j += i << 1;
        }

        i <<= 1;
    }

    if is_reverse {
        for i in 0..n {
            arr[i] = Complex {
                real: arr[i].real / n as f64,
                imaginary: arr[i].imaginary / n as f64,
            };
        }
    }
}

fn multiply(a: &Vec<i64>, b: &Vec<i64>) -> Vec<i64> {
    let mut x = a
        .iter()
        .map(|val| Complex::new(*val as f64, 0.0))
        .collect::<Vec<_>>();
    let mut y = b
        .iter()
        .map(|val| Complex::new(*val as f64, 0.0))
        .collect::<Vec<_>>();
    let mut np = 2;

    while np < x.len() + y.len() {
        np *= 2;
    }

    x.resize(np, Complex::default());
    y.resize(np, Complex::default());

    process_fft(&mut x, false);
    process_fft(&mut y, false);

    for i in 0..np {
        x[i] *= y[i].clone();
    }

    process_fft(&mut x, true);

    x.resize(np, Complex::default());
    y.resize(np, Complex::default());

    let mut ret = vec![0; np];

    for i in 0..np {
        ret[i] = x[i].real.round() as i64;
    }

    ret
}

fn pow(values: &mut Vec<i64>, mut k: i64) -> Vec<i64> {
    let mut ret = vec![1];

    while k > 0 {
        if k % 2 == 1 {
            ret = multiply(&ret, values);
        }

        *values = multiply(&values, &values);

        for val in ret.iter_mut() {
            if *val != 0 {
                *val = 1;
            }
        }

        for val in values.iter_mut() {
            if *val != 0 {
                *val = 1;
            }
        }

        k /= 2;
    }

    ret
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let (n, m, k) = (
        scan.token::<usize>(),
        scan.token::<i64>(),
        scan.token::<usize>(),
    );
    let mut stimulations = vec![0; n];
    let mut plans = vec![0; k];

    for i in 0..n {
        stimulations[i] = scan.token::<usize>();
    }

    for i in 0..k {
        plans[i] = scan.token::<usize>();
    }

    stimulations.sort();

    let mut values = vec![0; stimulations[n - 1] + 1];

    for i in 0..n {
        values[stimulations[i]] = 1;
    }

    let weights = pow(&mut values, m);
    let mut s1 = vec!['0'; weights.len()];
    let mut s2 = vec!['0'; plans[k - 1] + 1];

    for i in 0..weights.len() {
        if weights[i] != 0 {
            s1[i] = '1';
        }
    }

    for i in 0..k {
        s2[plans[i]] = '1';
    }

    let mut cmp = 0;
    let mut fail = vec![0; plans[k - 1] + 1];

    for i in 1..plans[k - 1] + 1 {
        while cmp > 0 && s2[cmp] != s2[i] {
            cmp = fail[cmp - 1];
        }

        if s2[cmp] == s2[i] {
            cmp += 1;
            fail[i] = cmp;
        }
    }

    let mut ret = 0;
    cmp = 0;

    for i in 0..weights.len() {
        if s1[i] == s2[cmp] {
            if cmp == plans[k - 1] {
                ret += 1;
                cmp = fail[cmp];
            } else {
                cmp += 1;
            }
        } else {
            while cmp > 0 && s1[i] != s2[cmp] {
                cmp = fail[cmp - 1];
            }

            if s1[i] == s2[cmp] {
                cmp += 1;
            }
        }
    }

    writeln!(out, "{ret}").unwrap();
}
