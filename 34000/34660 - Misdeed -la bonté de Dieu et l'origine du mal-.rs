use io::Write;
use std::{
    io,
    ops::{Add, Div, Index, IndexMut, Mul, Sub},
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

pub const K: usize = 4;
pub const Q: u64 = 0x03;

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

        GF::new(c)
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
#[derive(Clone, Copy)]
struct Matrix<const R: usize, const C: usize> {
    mat: [[GF; C]; R],
}

impl<const R: usize, const C: usize> Matrix<R, C> {
    #[inline]
    fn from_array(a: [[GF; C]; R]) -> Self {
        Self { mat: a }
    }

    #[inline]
    fn transpose(&self) -> Matrix<C, R> {
        let mut trans = [[GF::new(0); R]; C];

        for i in 0..R {
            for j in 0..C {
                trans[j][i] = self.mat[i][j];
            }
        }

        Matrix::<C, R>::from_array(trans)
    }

    #[inline]
    fn multiply<const K: usize, const C2: usize>(&self, rhs: &Matrix<K, C2>) -> Matrix<R, C2>
    where
        [(); K]:,
    {
        let mut ret = [[GF::new(0); C2]; R];

        for i in 0..R {
            for k in 0..K {
                let aik = self.mat[i][k];

                if aik.value() == 0 {
                    continue;
                }

                for j in 0..C2 {
                    ret[i][j] = ret[i][j] + aik * rhs.mat[k][j];
                }
            }
        }

        Matrix::<R, C2>::from_array(ret)
    }
}

impl<const N: usize> Matrix<N, N> {
    #[inline]
    fn identity() -> Self {
        let mut mat = [[GF::new(0); N]; N];

        for i in 0..N {
            mat[i][i] = GF::new(1);
        }

        Self { mat }
    }

    fn invert(&self) -> Self {
        let mut left = self.mat;
        let mut right = Self::identity().mat;

        for col in 0..N {
            let mut piv = col;

            while piv < N && left[piv][col].value() == 0 {
                piv += 1;
            }

            if piv != col {
                left.swap(piv, col);
                right.swap(piv, col);
            }

            let inv = left[col][col].inv();

            for j in 0..N {
                left[col][j] = left[col][j] * inv;
                right[col][j] = right[col][j] * inv;
            }

            for i in 0..N {
                if i == col {
                    continue;
                }

                let f = left[i][col];

                if f.value() != 0 {
                    for j in 0..N {
                        left[i][j] = left[i][j] + f * left[col][j];
                        right[i][j] = right[i][j] + f * right[col][j];
                    }
                }
            }
        }

        Self { mat: right }
    }
}

impl<const R: usize, const C: usize> Index<(usize, usize)> for Matrix<R, C> {
    type Output = GF;

    #[inline]
    fn index(&self, idx: (usize, usize)) -> &Self::Output {
        &self.mat[idx.0][idx.1]
    }
}

impl<const R: usize, const C: usize> IndexMut<(usize, usize)> for Matrix<R, C> {
    #[inline]
    fn index_mut(&mut self, idx: (usize, usize)) -> &mut Self::Output {
        &mut self.mat[idx.0][idx.1]
    }
}

const N: usize = 13;
const D: usize = 7;

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let t = scan.token::<i64>();
    let mut x = [[GF::new(0); D]; N];
    let mut y = [[GF::new(0); D]; N];

    for i in 0..N {
        let xi = GF::new(i as u64);
        let yi = GF::new(i as u64);

        x[i][0] = GF::new(1);
        y[i][0] = GF::new(1);

        for j in 1..D {
            x[i][j] = x[i][j - 1] * xi;
            y[i][j] = y[i][j - 1] * yi;
        }
    }

    if t == 0 {
        let binary = scan.token::<String>().bytes().collect::<Vec<_>>();
        let mut mat = [[GF::new(0); D]; D];

        for idx in 0..D * D {
            let mut val = 0;

            for i in 0..4 {
                let c = binary[4 * idx + i];
                let bit = if c == b'1' { 1 } else { 0 };

                val = (val << 1) | bit;
            }

            mat[idx / D][idx % D] = GF::new(val);
        }

        for i in 0..N {
            for j in 0..N {
                let mut acc = GF::new(0);

                for a in 0..D {
                    if x[i][a].value() == 0 {
                        continue;
                    }

                    for b in 0..D {
                        if y[j][b].value() == 0 {
                            continue;
                        }

                        acc = acc + mat[a][b] * x[i][a] * y[j][b];
                    }
                }

                write!(out, "{} ", acc.value()).unwrap();
            }

            writeln!(out).unwrap();
        }
    } else {
        let mut r = [0; D];
        let mut c = [0; D];

        for i in 0..D {
            r[i] = scan.token::<usize>() - 1;
        }

        for i in 0..D {
            c[i] = scan.token::<usize>() - 1;
        }

        let mut mat = [[GF::new(0); D]; D];

        for i in 0..D {
            for j in 0..D {
                mat[i][j] = GF::new(scan.token::<u64>());
            }
        }

        let mut vandemonde_x_arr = [[GF::new(0); D]; D];
        let mut vandemonde_y_arr = [[GF::new(0); D]; D];

        for p in 0..D {
            for a in 0..D {
                vandemonde_x_arr[p][a] = x[r[p]][a];
                vandemonde_y_arr[p][a] = y[c[p]][a];
            }
        }

        let vandemonde_x = Matrix::<D, D>::from_array(vandemonde_x_arr);
        let vandemonde_y = Matrix::<D, D>::from_array(vandemonde_y_arr);
        let vandemonde_x_inv = vandemonde_x.invert();
        let vandemonde_y_inv = vandemonde_y.invert();
        let coeffs = vandemonde_x_inv
            .multiply(&Matrix::<D, D>::from_array(mat))
            .multiply(&vandemonde_y_inv.transpose());

        for i in 0..D {
            for j in 0..D {
                let val = (coeffs[(i, j)].value() & 0xF) as u8;

                for k in (0..4).rev() {
                    write!(out, "{}", if ((val >> k) & 1) != 0 { 1 } else { 0 }).unwrap();
                }
            }
        }
    }
}
