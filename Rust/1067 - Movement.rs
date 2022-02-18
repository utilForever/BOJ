use io::Write;
use std::{
    cmp,
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

fn multiply(x: &mut Vec<Complex>, y: &mut Vec<Complex>) {
    let n = x.len();
    let mut np = n;

    if n & (n - 1) != 0 {
        np = 1;
        
        while np < 2 * n {
            np *= 2;
        }

        x.resize(np, Complex::default());
        y.resize(np, Complex::default());

        for i in 0..n {
            y[np - n + i] = y[i].clone();
        }
    }

    process_fft(x, false);
    process_fft(y, false);

    for i in 0..np {
        x[i] *= y[i].clone();
    }

    process_fft(x, true);

    if n & (n - 1) != 0 {
        x.resize(n, Complex::default());
        y.resize(n, Complex::default());
    }
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let n = scan.token::<usize>();
    let mut x = vec![Complex::default(); n];
    let mut y = vec![Complex::default(); n];

    for i in 0..n {
        x[i] = Complex::new(scan.token::<f64>(), 0.0);
    }

    for i in 1..=n {
        y[n - i] = Complex::new(scan.token::<f64>(), 0.0);
    }

    multiply(&mut x, &mut y);

    let mut ans = 0;

    for i in 0..n {
        ans = cmp::max(ans, x[i].real.round() as i64);
    }

    writeln!(out, "{}", ans).unwrap();
}
