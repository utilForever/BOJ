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

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let n = scan.token::<i64>();
    let n_sqrt = (n as f64).sqrt() as i64;

    if n_sqrt * n_sqrt == n {
        writeln!(out, "-1").unwrap();
        return;
    }

    let mut ret = 0;
    let mut a = 1;

    while 2 * a * a <= n {
        let b = n - a * a;
        let b_sqrt = (b as f64).sqrt() as i64;

        if b_sqrt * b_sqrt == b {
            ret += 1;
        }

        a += 1;
    }

    let mut a = 1;

    while a * a < n {
        if n % a != 0 {
            a += 1;
            continue;
        }

        let b = n / a;

        if a % 2 == b % 2 {
            ret += 1;
        }

        a += 1;
    }

    writeln!(out, "{ret}").unwrap();
}
