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

fn pow(x: i64, mut p: i64, modulo: i64) -> i64 {
    let mut ret = 1;
    let mut piv = x;

    while p != 0 {
        if p & 1 != 0 {
            ret = ret * piv % modulo;
        }

        piv = piv * piv % modulo;
        p >>= 1;
    }

    ret
}

fn berlekamp_massey(vals: Vec<i64>, modulo: i64) -> Vec<i64> {
    let mut ls = Vec::new();
    let mut cur = Vec::new();

    let mut lf = 0;
    let mut ld = 0;

    for i in 0..vals.len() {
        let mut t = 0;

        for j in 0..cur.len() {
            t = (t + vals[i - j - 1] * cur[j]) % modulo;
        }

        if (t - vals[i]) % modulo == 0 {
            continue;
        }

        if cur.is_empty() {
            cur.resize(i + 1, 0);
            lf = i;
            ld = (t - vals[i]) % modulo;

            continue;
        }

        let k = -(vals[i] - t) * pow(ld, modulo - 2, modulo) % modulo;

        let mut c = vec![0; i - lf - 1];
        c.push(k);

        for j in ls.iter() {
            c.push(-j * k % modulo);
        }

        if c.len() < cur.len() {
            c.resize(cur.len(), 0);
        }

        for j in 0..cur.len() {
            c[j] = (c[j] + cur[j]) % modulo;
        }

        if i - lf + ls.len() >= cur.len() {
            (ls, lf, ld) = (cur, i, (t - vals[i]) % modulo);
        }

        cur = c;
    }

    for i in cur.iter_mut() {
        *i = (*i % modulo + modulo) % modulo;
    }

    cur
}

fn get_nth(rec: Vec<i64>, dp: Vec<i64>, mut n: usize, modulo: i64) -> i64 {
    let m = rec.len();
    let mut s = vec![0; m];
    let mut t = vec![0; m];

    s[0] = 1;
    if m != 1 {
        t[1] = 1;
    } else {
        t[0] = rec[0];
    }

    let mul = |v: Vec<i64>, w: Vec<i64>| -> Vec<i64> {
        let m = v.len();
        let mut t = vec![0; 2 * m];

        for j in 0..m {
            for k in 0..m {
                t[j + k] += v[j] * w[k] % modulo;

                if t[j + k] >= modulo {
                    t[j + k] -= modulo;
                }
            }
        }

        for j in (m..=2 * m - 1).rev() {
            for k in 1..=m {
                t[j - k] += t[j] * rec[k - 1] % modulo;

                if t[j - k] >= modulo {
                    t[j - k] -= modulo;
                }
            }
        }

        t.resize(m, 0);

        t
    };

    while n != 0 {
        if n & 1 != 0 {
            s = mul(s, t.clone());
        }

        t = mul(t.clone(), t.clone());
        n >>= 1;
    }

    let mut ret = 0;

    for i in 0..m {
        ret += s[i] * dp[i] % modulo;
    }

    ret % modulo
}

fn guess_nth_term(vals: Vec<i64>, n: usize, modulo: i64) -> i64 {
    if n < vals.len() {
        return vals[n as usize];
    }

    let ret = berlekamp_massey(vals.clone(), modulo);

    if ret.is_empty() {
        0
    } else {
        get_nth(ret, vals, n, modulo)
    }
}

const MOD: i64 = 1e9 as i64 + 7;

// Reference: https://koosaga.com/231
fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let (n, k) = (scan.token::<usize>(), scan.token::<i64>());
    let mut fibonacci = vec![0; 301];
    let mut arr = Vec::with_capacity(300);

    fibonacci[0] = 1;
    fibonacci[1] = 1;

    for i in 2..=300 {
        fibonacci[i] = (fibonacci[i - 1] + fibonacci[i - 2]) % MOD;
    }

    arr.push(fibonacci[1] * pow(1, k, MOD) % MOD);

    for i in 2..300 {
        arr.push((arr.last().unwrap() + fibonacci[i] * pow(i as i64, k, MOD) % MOD) % MOD);
    }

    writeln!(out, "{}", guess_nth_term(arr, n - 1, MOD)).unwrap();
}
