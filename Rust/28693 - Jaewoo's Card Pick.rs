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

static MOD: i64 = 1_000_000_007;

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let n = scan.token::<usize>();
    let mut inv = vec![0; 2 * n + 1];
    let mut possibilites = vec![vec![0; 2 * n + 1]; 2 * n + 1];

    for i in 0..=2 * n {
        inv[i] = pow(i as i64, MOD - 2, MOD);
    }

    for i in (4..=2 * n).step_by(2) {
        for j in (0..=i / 2 - 1).rev() {
            if j > 0 {
                possibilites[i][j] = (possibilites[i][j]
                    + j as i64 * inv[i - j] % MOD * possibilites[i - 2][j - 1] % MOD)
                    % MOD;
            }
            possibilites[i][j] = (possibilites[i][j]
                + (i - 2 * j) as i64 * inv[i - j] % MOD * inv[i - j - 1] % MOD
                    * possibilites[i - 2][j]
                    % MOD)
                % MOD;
            possibilites[i][j] = (possibilites[i][j]
                + (i - 2 * j) as i64 * inv[i - j] % MOD * j as i64 % MOD * inv[i - j - 1] % MOD
                    * (possibilites[i - 2][j] + 1)
                    % MOD)
                % MOD;
            possibilites[i][j] = (possibilites[i][j]
                + (i - 2 * j) as i64 * inv[i - j] % MOD * (i - 2 * j - 2) as i64 % MOD
                    * inv[i - j - 1]
                    % MOD
                    * (possibilites[i][j + 2] + 1)
                    % MOD)
                % MOD;
        }
    }

    writeln!(out, "{}", n as i64 + possibilites[2 * n][0]).unwrap();
}
