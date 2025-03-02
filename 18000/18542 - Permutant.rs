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

    pub fn line(&mut self) -> String {
        let mut input = String::new();
        self.reader.read_line(&mut input).expect("Failed read");
        input
    }
}

const MOD: i64 = 1_000_000_007;

fn pow(x: i64, mut y: i64) -> i64 {
    let mut ret = 1;
    let mut piv = x % MOD;

    while y != 0 {
        if y & 1 != 0 {
            ret = ret * piv % MOD
        }

        piv = piv * piv % MOD;
        y >>= 1;
    }

    ret
}

// Computes the remainder when polynomial `a(x)` is divided by `b(x)`.
fn polynomial_remainder(mut a: Vec<i64>, b: &Vec<i64>) -> Vec<i64> {
    let mut n = a.len() as i64 - 1;
    let m = b.len() - 1;
    let inv = pow(b[m], MOD - 2);

    while n >= m as i64 {
        let x = inv * (MOD - a[n as usize]) % MOD;

        for i in 0..m {
            a[n as usize - m + i] = (a[n as usize - m + i] + x * b[i]) % MOD;
        }

        a.pop();

        while !a.is_empty() && a[a.len() - 1] == 0 {
            a.pop();
        }

        n = a.len() as i64 - 1;
    }

    a
}

// Recursively computes the resultant of two polynomials a(x) and b(x).
//
// Given:
//   a(x) = a[0] + a[1]*x + ... + a[n]*x^n, (a[n] != 0)
//   b(x) = b[0] + b[1]*x + ... + b[m]*x^m, (b[m] != 0)
//
// The resultant is related to the roots of the polynomials and is computed using a recursive subresultant algorithm.
fn resultant(a: &Vec<i64>, b: &Vec<i64>) -> i64 {
    if b.is_empty() {
        return 0;
    }

    if b.len() == 1 {
        return pow(b[0], (a.len() - 1) as i64);
    }

    let c = polynomial_remainder(a.clone(), b);

    pow(*b.last().unwrap(), a.len() as i64 - c.len() as i64)
        * if ((a.len() as i64 & 1) | (b.len() as i64 & 1)) != 0 {
            1
        } else {
            MOD - 1
        }
        % MOD
        * resultant(b, &c)
        % MOD
}

// Reference: Petrozavodsk Programming Camp, Winter 2019, Day 7: Oleksandr Kulkov Contest 1, Botan Investment Cup Editorial
// Reference: https://codeforces.com/blog/entry/101876
// Reference: https://github.com/ShahjalalShohag/code-library/blob/main/Math/Determinant%20of%20Permutant%20Matrix.cpp
fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let n = scan.token::<usize>();
    let mut sequence = vec![0; n];
    let mut permutation = vec![0; n];

    for i in 0..n {
        sequence[i] = scan.token::<i64>();
    }

    for i in 0..n {
        permutation[i] = scan.token::<usize>() - 1;
    }

    // If permutation is not cyclic (i.e. has more than one cycle), then the answer is 0
    let mut cycles = 0;
    let mut visited = vec![false; n];

    for i in 0..n {
        if !visited[i] {
            cycles += 1;
            let mut j = i;

            while !visited[j] {
                visited[j] = true;
                j = permutation[j];
            }
        }
    }

    if cycles > 1 {
        writeln!(out, "0").unwrap();
        return;
    }

    // Build the array `q` which represents the inverse cycle
    let mut inv = vec![0; n];
    let mut prev = 0;

    for i in (0..n).rev() {
        inv[i] = prev;
        prev = permutation[prev];
    }

    // For each cycle in `q`, flip the sign
    let mut ret = if n % 2 == 1 { MOD - 1 } else { 1 };
    let mut visited = vec![false; n];

    for i in 0..n {
        if visited[i] {
            continue;
        }

        let mut j = i;

        while !visited[j] {
            visited[j] = true;
            j = inv[j];
        }

        ret = MOD - ret;
    }

    // Construct polynomial b(x) using the rearranged sequence based on q
    let mut b = vec![0; n];

    for i in 0..n {
        b[i] = sequence[inv[i]];
    }

    // Construct polynomial a(x) = x^n - 1
    let mut a = vec![0; n + 1];
    a[0] = MOD - 1;
    a[n] = 1;

    ret = ret * resultant(&a, &b) % MOD;

    writeln!(out, "{ret}").unwrap();
}
