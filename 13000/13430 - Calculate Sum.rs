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

fn pow(x: i64, mut y: i64) -> i64 {
    let mut ret = 1;
    let mut piv = x % 1_000_000_007;

    while y != 0 {
        if y & 1 != 0 {
            ret = multiply(ret, piv, 1_000_000_007);
        }

        piv = multiply(piv, piv, 1_000_000_007);
        y >>= 1;
    }

    ret
}

fn factorial(val: i64) -> i64 {
    let mut ret = 1;

    if val == 0 {
        return 1;
    }

    for i in 1..=val {
        ret = (ret * i) % 1_000_000_007;
    }

    ret
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let (k, n) = (scan.token::<i64>(), scan.token::<i64>());
    let mut ret = 1;

    for i in n..=n + k {
        ret = (ret * i) % 1_000_000_007;
    }

    ret = (ret * pow(factorial(k + 1), 1_000_000_005)) % 1_000_000_007;

    writeln!(out, "{}", ret).unwrap();
}
