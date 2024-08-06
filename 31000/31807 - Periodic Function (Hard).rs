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

    let n = scan.token::<usize>();
    let mut integrals = vec![0; n];

    for i in 0..n {
        integrals[i] = scan.token::<i64>();
    }

    let mut cmp = 0;
    let mut fail = vec![0; n];

    for i in 1..n {
        while cmp > 0 && integrals[cmp] != integrals[i] {
            cmp = fail[cmp - 1] as usize;
        }

        if integrals[cmp] == integrals[i] {
            cmp += 1;
            fail[i] = cmp as i64;
        }
    }

    let mut p = n as i64 - 1;

    while p >= 0 && fail[p as usize] != 1 {
        p -= 1;
    }

    let (mut a, mut b) = (scan.token::<i64>(), scan.token::<i64>());
    a += p * 1_000_000_000;
    b += p * 1_000_000_000;

    let period = (b - a) / p;
    a %= p;
    b %= p;

    let mut ret = integrals.iter().take(p as usize).sum::<i64>() * period;

    if a > b {
        b += p;
    }

    for i in a + 1..=b {
        ret += integrals[((i - 1) % p) as usize];
    }

    writeln!(out, "{ret}").unwrap();
}
