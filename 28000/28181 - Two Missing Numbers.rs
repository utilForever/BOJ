use io::Write;
use std::{
    io,
    ops::{Add, Div, Mul, Sub},
    str,
    sync::OnceLock,
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

pub const K: usize = 64;
pub const Q: u64 = 0x1B;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct GF(u64);

impl GF {
    #[inline]
    pub fn new(x: u64) -> Self {
        GF(x & mask())
    }

    #[inline]
    pub fn value(self) -> u64 {
        self.0
    }

    #[inline]
    pub fn square(self) -> Self {
        self * self
    }

    #[inline]
    pub fn pow3(self) -> Self {
        self.square() * self
    }

    pub fn inv(self) -> Self {
        let modulus = (1u128 << (K as u32)) | (Q as u128);
        let mut u = self.0 as u128;
        let mut v = modulus;
        let mut g1 = 1;
        let mut g2 = 0;

        while u != 1 {
            let mut deg_u = deg(u);
            let mut deg_v = deg(v);

            if deg_u < deg_v {
                std::mem::swap(&mut u, &mut v);
                std::mem::swap(&mut g1, &mut g2);
                std::mem::swap(&mut deg_u, &mut deg_v);
            }

            let diff = (deg_u - deg_v) as u32;

            u ^= v << diff;
            g1 ^= g2 << diff;
        }

        GF(mod_reduce(g1, modulus))
    }

    pub fn solve_z2_plus_z(c: GF) -> GF {
        let lcols = lcols_cached();
        let mut rows = [0; K];

        for i in 0..K {
            let mut mask_row = 0;

            for j in 0..K {
                if ((lcols[j] >> i) & 1) != 0 {
                    mask_row |= 1u64 << j;
                }
            }

            let rhs = ((c.0 >> i) & 1) as u128;
            rows[i] = (mask_row as u128) | (rhs << K);
        }

        let mut r = 0;
        let mut pos_col = [-1; K];

        for col in 0..K {
            let mut piv = None;

            for i in r..K {
                if ((rows[i] >> col) & 1) != 0 {
                    piv = Some(i);
                    break;
                }
            }

            if let Some(p) = piv {
                if p != r {
                    rows.swap(r, p);
                }

                for i in 0..K {
                    if i != r && ((rows[i] >> col) & 1) != 0 {
                        rows[i] ^= rows[r];
                    }
                }

                pos_col[col] = r as i32;
                r += 1;

                if r == K {
                    break;
                }
            }
        }

        let mut z = 0;

        for col in 0..K {
            let w = pos_col[col];

            if w != -1 {
                let rhs = ((rows[w as usize] >> K) & 1) as u64;

                if rhs != 0 {
                    z |= 1u64 << col;
                }
            }
        }

        GF(z)
    }
}

impl Add for GF {
    type Output = GF;

    #[inline]
    fn add(self, rhs: GF) -> GF {
        GF(self.0 ^ rhs.0)
    }
}

impl Sub for GF {
    type Output = GF;

    #[inline]
    fn sub(self, rhs: GF) -> GF {
        GF(self.0 ^ rhs.0)
    }
}

impl Mul for GF {
    type Output = GF;

    #[inline]
    fn mul(self, rhs: GF) -> GF {
        let mut a = self.0;
        let mut b = rhs.0;
        let mut c = 0;

        for _ in 0..K {
            if (b & 1) != 0 {
                c ^= a;
            }

            let carry = (a >> (K - 1)) & 1;

            a = a.wrapping_shl(1);

            if carry != 0 {
                a ^= Q;
            }

            b >>= 1;
        }

        GF(c)
    }
}

impl Div for GF {
    type Output = GF;

    #[inline]
    fn div(self, rhs: GF) -> GF {
        assert!(rhs.0 != 0, "division by zero");
        self * rhs.inv()
    }
}

#[inline]
fn mask() -> u64 {
    if K == 64 {
        !0
    } else {
        (1u64 << K) - 1
    }
}

#[inline]
fn deg(x: u128) -> i32 {
    if x == 0 {
        -1
    } else {
        127 - x.leading_zeros() as i32
    }
}

fn mod_reduce(mut r: u128, modulus: u128) -> u64 {
    while deg(r) >= K as i32 {
        let shift = (deg(r) - K as i32) as u32;
        r ^= modulus << shift;
    }

    r as u64
}

static LCOLS: OnceLock<[u64; K]> = OnceLock::new();

fn lcols_cached() -> &'static [u64; K] {
    LCOLS.get_or_init(|| {
        let mut cols = [0; K];

        for j in 0..K {
            let exp = 1u64 << j;
            cols[j] = (GF(exp).square().0) ^ exp;
        }

        cols
    })
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let q = scan.token::<u32>();
    let n = scan.token::<usize>();
    let (mut c, mut d) = (0, 0);

    if q == 2 {
        c = scan.token::<u64>();
        d = scan.token::<u64>();
    }

    let mut a = 0;
    let mut b = 0;

    for _ in 0..n {
        let z = scan.token::<u64>();

        a ^= z;
        b ^= GF::new(z).pow3().value();
    }

    if q == 1 {
        writeln!(out, "{a} {b}").unwrap();
        return;
    }

    let p1 = GF::new(c ^ a);
    let p3 = GF::new(d ^ b);

    let c = (p3 + p1.pow3()) / p1.pow3();
    let z = GF::solve_z2_plus_z(c);
    let x = (p1 * z).value();
    let y = x ^ p1.value();

    writeln!(out, "{x} {y}").unwrap();
}
