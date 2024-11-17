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

const MOD: i64 = 1_234_543;

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

fn combination(factorial: &Vec<i64>, mut n: usize, mut r: usize) -> i64 {
    // factorial[n] * pow(factorial[r] * factorial[n - r] % MOD, MOD - 2) % MOD
    let mut ret = 1;

    while n > 0 && r > 0 {
        let (n_mod, r_mod) = (n % MOD as usize, r % MOD as usize);

        if n_mod < r_mod {
            return 0;
        }

        ret = ret * factorial[n_mod] % MOD;
        ret = ret * pow(factorial[r_mod] * factorial[n_mod - r_mod] % MOD, MOD - 2) % MOD;

        n /= MOD as usize;
        r /= MOD as usize;
    }

    ret
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let mut factorial = vec![1; 1_234_544];

    for i in 2..=1_234_543 {
        factorial[i] = factorial[i - 1] * i as i64 % MOD;
    }

    let q = scan.token::<i64>();

    for _ in 0..q {
        let (w, b) = (scan.token::<usize>(), scan.token::<usize>());
        writeln!(
            out,
            "{} {}",
            if b % 2 == 0 { "even" } else { "odd" },
            combination(&factorial, w + b - 1, b)
        )
        .unwrap();
    }
}
