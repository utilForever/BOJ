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

static MOD: i64 = 998_244_353;

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

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let mut dp1 = vec![vec![0; 2001]; 2001];
    let mut dp2 = vec![vec![0; 2001]; 2001];

    dp1[1][0] = (MOD + 1) / 2;
    dp2[1][1] = 1;

    for i in 1..2000 {
        dp2[i + 1][i + 1] = (dp2[i][i] + pow(2 * i as i64 + 1, MOD - 2, MOD)) % MOD;

        dp1[i + 1][i] = (dp1[i][i] * 2 - dp1[i][i - 1]) % MOD;
        if dp1[i + 1][i] < 0 {
            dp1[i + 1][i] += MOD;
        }

        dp2[i + 1][i] = (dp2[i][i] * 2 - dp2[i][i - 1]) % MOD;
        if dp2[i + 1][i] < 0 {
            dp2[i + 1][i] += MOD;
        }

        dp1[i + 1][0] = (dp1[i][0] * 4 - dp1[i - 1][0] - dp1[i][1] * 2) % MOD;
        if dp1[i + 1][0] < 0 {
            dp1[i + 1][0] += MOD;
        }

        dp2[i + 1][0] = (dp2[i][0] * 4 - dp2[i - 1][0] - dp2[i][1] * 2) % MOD;
        if dp2[i + 1][0] < 0 {
            dp2[i + 1][0] += MOD;
        }

        for j in 1..i {
            dp1[i + 1][j] = (dp1[i][j] * 4 - dp1[i][j - 1] - dp1[i][j + 1] - dp1[i - 1][j]) % MOD;
            if dp1[i + 1][j] < 0 {
                dp1[i + 1][j] += MOD;
            }

            dp2[i + 1][j] = (dp2[i][j] * 4 - dp2[i][j - 1] - dp2[i][j + 1] - dp2[i - 1][j]) % MOD;
            if dp2[i + 1][j] < 0 {
                dp2[i + 1][j] += MOD;
            }
        }
    }

    let t = scan.token::<i64>();

    for _ in 0..t {
        let (x, y) = (scan.token::<i64>(), scan.token::<i64>());
        let (mut x, mut y) = (x.abs() as usize, y.abs() as usize);

        if x < y {
            std::mem::swap(&mut x, &mut y);
        }

        writeln!(out, "{} {}", dp1[x][y], dp2[x][y]).unwrap();
    }
}
