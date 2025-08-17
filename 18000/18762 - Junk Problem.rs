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

// Reference: https://www.mathworks.com/help/comm/ref/gf.html
#[rustfmt::skip]
const PRIMITIVES: [u32; 17] = [
    0,
    3,        // x^1 + x^0
    7,        // x^2 + x + 1
    11,       // x^3 + x + 1
    19,       // x^4 + x + 1
    37,       // x^5 + x^2 + 1
    67,       // x^6 + x + 1
    137,      // x^7 + x^3 + 1
    285,      // x^8 + x^4 + x^3 + x^2 + 1
    529,      // x^9 + x^4 + 1
    1033,     // x^10 + x^3 + 1
    2053,     // x^11 + x^2 + 1
    4179,     // x^12 + x^6 + x^4 + x + 1
    8219,     // x^13 + x^4 + x^3 + x + 1
    17475,    // x^14 + x^10 + x^6 + x + 1
    32771,    // x^15 + x + 1
    69643,    // x^16 + x^12 + x^3 + x + 1
];

fn gf_mul(mut a: u32, mut b: u32, poly: u32, q: u32) -> u32 {
    let mask = (1 << q) - 1;
    let red = poly & mask;
    let mut ret = 0;

    while b != 0 {
        if (b & 1) != 0 {
            ret ^= a;
        }

        b >>= 1;

        let carry = (a >> (q - 1)) & 1;

        a = (a << 1) & mask;

        if carry != 0 {
            a ^= red;
        }
    }

    ret & mask
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let n = scan.token::<u64>();

    if n == 1 {
        writeln!(out, "1").unwrap();
        writeln!(out, "1").unwrap();
        return;
    }

    let mut s = ((n / 2) as f64).sqrt() as u64;

    while (s + 1) * (s + 1) <= n / 2 {
        s += 1;
    }

    while s * s > n / 2 {
        s -= 1;
    }

    let q = if s <= 1 {
        1
    } else {
        64u32 - (s - 1).leading_zeros()
    };
    let poly = PRIMITIVES[q as usize];
    let mut ret = Vec::with_capacity(s as usize);

    for i in 0..s {
        let xi = i as u32;
        let x2 = gf_mul(xi, xi, poly, q);
        let x3 = gf_mul(x2, xi, poly, q);
        let fi = x3 ^ 1;
        let val = ((i as u64) << (q as usize)) | fi as u64;

        ret.push(val);
    }

    writeln!(out, "{s}").unwrap();

    for val in ret {
        write!(out, "{val} ").unwrap();
    }

    writeln!(out).unwrap();
}
