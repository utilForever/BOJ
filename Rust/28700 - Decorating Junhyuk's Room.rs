use io::Write;
use std::{io, ops::BitXorAssign, str};

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

#[derive(Clone, PartialEq, Eq)]
struct BitSet {
    bits: Vec<bool>,
}

impl BitSet {
    fn new(size: usize) -> BitSet {
        BitSet {
            bits: vec![false; size],
        }
    }

    fn set(&mut self, index: usize, value: bool) {
        self.bits[index] = value;
    }

    fn get(&self, index: usize) -> bool {
        self.bits[index]
    }
}

impl BitXorAssign for BitSet {
    fn bitxor_assign(&mut self, rhs: Self) {
        for i in 0..self.bits.len() {
            self.bits[i] ^= rhs.bits[i];
        }
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

const MOD: i64 = 1_000_000_007;

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let (n, m, k) = (
        scan.token::<usize>(),
        scan.token::<usize>(),
        scan.token::<usize>(),
    );
    let mut mat_a = vec![BitSet::new(1000); 1001];
    let mut mat_b = vec![BitSet::new(2000); 1001];

    for i in 0..n {
        for j in 0..m {
            let (p, q) = (scan.token::<i64>(), scan.token::<i64>());

            if p.abs() != 1 && q.abs() != 1 {
                writeln!(out, "0").unwrap();
                return;
            }

            if p.abs() != 2 && q.abs() != 2 {
                writeln!(out, "0").unwrap();
                return;
            }

            if p.abs() == 2 {
                mat_b[i].set(2 * j, true);
            }

            if p * q < 0 {
                mat_b[i].set(2 * j + 1, true);
            }
        }
    }

    for i in 0..k {
        let p = scan.token::<usize>();

        for _ in 0..p {
            let q = scan.token::<usize>();
            mat_a[q - 1].set(i, true);
        }
    }

    let mut i = 0;
    let mut p = 0;
    let mut r = 0;

    while i < n && p < k {
        let mut q = i;

        for j in i + 1..n {
            if mat_a[j].get(p) {
                q = j;
            }
        }

        if q == i && !mat_a[i].get(p) {
            r += 1;
            p += 1;
            continue;
        }

        if i != q {
            mat_a.swap(i, q);
            mat_b.swap(i, q);
        }

        for j in i + 1..n {
            if mat_a[j].get(p) {
                let tmp_a = mat_a[i].clone();
                let tmp_b = mat_b[i].clone();

                mat_a[j] ^= tmp_a;
                mat_b[j] ^= tmp_b;
            }
        }

        i += 1;
        p += 1;
    }

    r += k - p;

    for i in 0..n {
        if mat_a[i] == mat_a[1000] && mat_b[i] != mat_b[1000] {
            writeln!(out, "0").unwrap();
            return;
        }
    }

    writeln!(out, "{}", pow(4, (m * r) as i64, MOD)).unwrap();
}
