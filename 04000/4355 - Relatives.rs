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

fn euler_totient(mut n: usize) -> usize {
    let mut p = 2;
    let mut ret = n;

    while p * p <= n {
        if n % p == 0 {
            while n % p == 0 {
                n /= p;
            }

            ret -= ret / p;
        }

        p += 1;
    }

    if n > 1 {
        ret -= ret / n;
    }

    ret
}

// Reference: A005728 - Number of fractions in Farey series of order n
fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    loop {
        let n = scan.token::<usize>();

        if n == 0 {
            break;
        } else if n == 1 {
            writeln!(out, "0").unwrap();
            continue;
        }

        writeln!(out, "{}", euler_totient(n)).unwrap();
    }
}
