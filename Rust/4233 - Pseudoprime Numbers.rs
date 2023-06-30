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

fn is_prime(x: i64) -> bool {
    let mut idx = 2;

    while idx * idx <= x {
        if x % idx == 0 {
            return false;
        }

        idx += 1;
    }

    true
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    loop {
        let (p, a) = (scan.token::<i64>(), scan.token::<i64>());

        if p == 0 && a == 0 {
            break;
        }

        writeln!(
            out,
            "{}",
            if !is_prime(p) && pow(a, p, p) == a {
                "yes"
            } else {
                "no"
            }
        )
        .unwrap();
    }
}
