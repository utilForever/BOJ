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

fn pow(x: i64, mut y: i64, rem: i64) -> i64 {
    let mut ret = 1;
    let mut piv = x % rem;

    while y != 0 {
        if y & 1 != 0 {
            ret = ret * piv % rem
        }

        piv = piv * piv % rem;
        y >>= 1;
    }

    ret
}

fn legendre(a: i64, p: i64) -> i32 {
    if a % p == 0 {
        return 0;
    }

    let v = pow(a, (p - 1) / 2, p);

    if v == 1 {
        1
    } else if v == p - 1 {
        -1
    } else {
        0
    }
}

fn find_non_residue(p: i64) -> i64 {
    let mut z = 2;

    while z < p {
        if legendre(z, p) == -1 {
            return z;
        }

        z += 1;
    }

    2
}

fn tonelli_shanks(n: i64, p: i64) -> i64 {
    if p == 2 {
        return n % 2;
    }

    if legendre(n, p) != 1 {
        return -1;
    }

    let mut q = p - 1;
    let mut s = 0;

    while q % 2 == 0 {
        q >>= 1;
        s += 1;
    }

    if s == 1 {
        return pow(n, (p + 1) / 4, p);
    }

    let z = find_non_residue(p);
    let mut m = s;
    let mut c = pow(z, q, p);
    let mut t = pow(n, q, p);
    let mut r = pow(n, (q + 1) / 2, p);

    while t != 1 {
        let mut idx = 1;
        let mut tmp = (t * t) % p;

        while tmp != 1 {
            tmp = (tmp * tmp) % p;
            idx += 1;
        }

        let b = pow(c, 1 << (m - idx - 1), p);

        m = idx;
        c = (b * b) % p;
        t = (t * c) % p;
        r = (r * b) % p;
    }

    r
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let n = scan.token::<usize>();
    let mut nums = vec![0; n];

    for i in 0..n {
        nums[i] = scan.token::<i64>();
    }

    let p = 10_000_121;
    let val = tonelli_shanks(p - 1, p);

    let inv2 = pow(2, p - 2, p);
    let inv2_val = pow(2 * val % p, p - 2, p);

    writeln!(out, "{p}").unwrap();

    for i in 0..n {
        let x = (nums[i] + 1) * inv2 % p;
        let y = (1 + p - nums[i]) * inv2_val % p;

        writeln!(out, "{x} {y}").unwrap();
    }
}
