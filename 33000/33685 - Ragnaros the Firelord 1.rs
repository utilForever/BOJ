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

const MOD: i64 = 998_244_353;

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

fn pow_inv(x: i64) -> i64 {
    pow(x, MOD - 2)
}

fn process_backtrack(
    minions: Vec<i64>,
    idx: usize,
    prob: i64,
    n: usize,
    damage: i64,
    health_hero: i64,
) -> i64 {
    if health_hero <= 0 {
        return prob;
    }

    if idx == n {
        return 0;
    }

    let total = 1 + minions.len();
    let total_inv = pow_inv(total as i64);
    let mut ret = 0;

    ret =
        (ret + process_backtrack(
            minions.clone(),
            idx + 1,
            prob * total_inv % MOD,
            n,
            damage,
            health_hero - damage,
        )) % MOD;

    for i in 0..minions.len() {
        let mut minions_new = minions.clone();
        let hp = minions_new[i];
        let hp_new = hp - damage;

        if hp_new > 0 {
            minions_new[i] = hp_new;
            minions_new.sort_unstable();
        } else {
            minions_new.remove(i);
        }

        ret =
            (ret + process_backtrack(
                minions_new,
                idx + 1,
                prob * total_inv % MOD,
                n,
                damage,
                health_hero,
            )) % MOD;
    }

    ret % MOD
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let (n, m, x, y) = (
        scan.token::<usize>(),
        scan.token::<usize>(),
        scan.token::<i64>(),
        scan.token::<i64>(),
    );
    let mut h = vec![0; m];

    for i in 0..m {
        h[i] = scan.token::<i64>();
    }

    h.sort_unstable();

    let ret = process_backtrack(h, 0, 1, n, x, y);

    writeln!(out, "{ret}").unwrap();
}
