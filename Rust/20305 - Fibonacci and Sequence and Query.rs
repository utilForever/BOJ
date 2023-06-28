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

static MOD: i64 = 1_000_000_007;

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let (n, q) = (scan.token::<usize>(), scan.token::<usize>());
    let mut fibonacci = vec![0; n + 2];
    fibonacci[1] = 1;
    fibonacci[2] = 1;

    for i in 3..=n {
        fibonacci[i] = (fibonacci[i - 1] + fibonacci[i - 2]) % MOD;
    }

    let mut ret = vec![0; n + 3];

    for _ in 0..q {
        let (l, r) = (scan.token::<usize>(), scan.token::<usize>());
        ret[l] = (ret[l] + fibonacci[1]) % MOD;
        ret[l + 1] = (ret[l + 1] + fibonacci[2] - fibonacci[1]) % MOD;
        ret[r + 1] = (ret[r + 1] + MOD - fibonacci[r - l + 2]) % MOD;
        ret[r + 2] = (ret[r + 2] + MOD - fibonacci[r - l + 1]) % MOD;
    }

    for i in 2..=n {
        ret[i] = (ret[i] + ret[i - 1] + ret[i - 2]) % MOD;
    }

    for i in 1..=n {
        write!(out, "{} ", ret[i]).unwrap();
    }

    writeln!(out).unwrap();
}
