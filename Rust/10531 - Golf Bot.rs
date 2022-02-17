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

fn multiply(arr: &mut Vec<Complex>) {
    let mut n = 1;

    while n <= arr.len() {
        n <<= 1;
    }

    n <<= 1;
    arr.resize(
        n,
        Complex {
            real: 0.0,
            imaginary: 0.0,
        },
    );

    process_fft(arr, false);

    for i in 0..n {
        let ret = arr[i].clone() * arr[i].clone();
        arr[i] = ret;
    }

    process_fft(arr, true);
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let n = scan.token::<usize>();
    let mut arr = vec![
        Complex {
            real: 0.0,
            imaginary: 0.0,
        };
        200_001
    ];

    for _ in 0..n {
        let t = scan.token::<usize>();
        arr[t] = Complex {
            real: 1.0,
            imaginary: 0.0,
        };
    }

    arr[0] = Complex {
        real: 1.0,
        imaginary: 0.0,
    };

    multiply(&mut arr);

    let m = scan.token::<usize>();
    let mut ans = 0;

    for _ in 0..m {
        let t = scan.token::<usize>();
        if arr[t].real.round() > 0.0 {
            ans += 1;
        }
    }

    writeln!(out, "{}", ans).unwrap();
}
