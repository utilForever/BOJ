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

fn pow(x: i64, mut p: i64) -> i64 {
    let mut ret = 1;
    let mut piv = x;

    while p != 0 {
        if p & 1 != 0 {
            ret = ret * piv % MOD;
        }

        piv = piv * piv % MOD;
        p >>= 1;
    }

    ret
}

fn berlekamp_massey(vals: &Vec<i64>) -> Vec<i64> {
    let mut ls = Vec::new();
    let mut cur = Vec::new();

    let mut lf = 0;
    let mut ld = 0;

    for i in 0..vals.len() {
        let mut t = 0;

        for j in 0..cur.len() {
            t = (t + vals[i - j - 1] * cur[j]) % MOD;
        }

        if (t - vals[i]) % MOD == 0 {
            continue;
        }

        if cur.is_empty() {
            cur.resize(i + 1, 0);
            lf = i;
            ld = (t - vals[i]) % MOD;

            continue;
        }

        let k = -(vals[i] - t) * pow(ld, MOD - 2) % MOD;

        let mut c = vec![0; i - lf - 1];
        c.push(k);

        for j in ls.iter() {
            c.push(-j * k % MOD);
        }

        if c.len() < cur.len() {
            c.resize(cur.len(), 0);
        }

        for j in 0..cur.len() {
            c[j] = (c[j] + cur[j]) % MOD;
        }

        if i - lf + ls.len() >= cur.len() {
            (ls, lf, ld) = (cur, i, (t - vals[i]) % MOD);
        }

        cur = c;
    }

    for i in cur.iter_mut() {
        *i = (*i % MOD + MOD) % MOD;
    }

    cur
}

fn get_nth(rec: &Vec<i64>, dp: &Vec<i64>, mut n: usize) -> i64 {
    let m = rec.len();
    let mut s = vec![0; m];
    let mut t = vec![0; m];

    s[0] = 1;
    if m != 1 {
        t[1] = 1;
    } else {
        t[0] = rec[0];
    }

    let mul = |v: &Vec<i64>, w: &Vec<i64>| -> Vec<i64> {
        let m = v.len();
        let mut t = vec![0; 2 * m];

        for j in 0..m {
            for k in 0..m {
                t[j + k] += v[j] * w[k] % MOD;

                if t[j + k] >= MOD {
                    t[j + k] -= MOD;
                }
            }
        }

        for j in (m..=2 * m - 1).rev() {
            for k in 1..=m {
                t[j - k] += t[j] * rec[k - 1] % MOD;

                if t[j - k] >= MOD {
                    t[j - k] -= MOD;
                }
            }
        }

        t.resize(m, 0);

        t
    };

    while n != 0 {
        if n & 1 != 0 {
            s = mul(&s, &t);
        }

        t = mul(&t, &t);
        n >>= 1;
    }

    let mut ret = 0;

    for i in 0..m {
        ret += s[i] * dp[i] % MOD;
    }

    ret % MOD
}

fn guess_nth_term(vals: &Vec<i64>, n: usize) -> i64 {
    if n < vals.len() {
        return vals[n];
    }

    let ret = berlekamp_massey(vals);

    if ret.is_empty() {
        0
    } else {
        get_nth(&ret, vals, n)
    }
}

const MOD: i64 = 1_000_000_007;

// Reference: https://koosaga.com/231
fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let (n, m) = (scan.token::<usize>(), scan.token::<usize>());
    let mut qualities = vec![0; m];

    for i in 0..m {
        qualities[i] = scan.token::<i64>();
    }

    let mut dp = vec![0; 2001];
    dp[0] = 1;

    for i in 0..m {
        for j in 1..=2000 {
            dp[j] = (dp[j] + (qualities[i] * dp[j - 1]) % MOD) % MOD;
        }
    }

    writeln!(out, "{}", guess_nth_term(&dp, n)).unwrap();
}
