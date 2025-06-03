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

    pub fn all(&mut self) -> String {
        let mut input = String::new();
        self.reader.read_to_string(&mut input).expect("Failed read");
        input
    }

    pub fn line(&mut self) -> String {
        let mut input = String::new();
        self.reader.read_line(&mut input).expect("Failed read");
        input
    }
}

#[inline(always)]
fn calculate(mut n: i64) -> i64 {
    let mut mul = 1;
    let mut acc = 0;

    while n > 2 {
        let q = n / 3;

        match n % 3 {
            0 => {
                acc += mul * (2 + 2 * q);
                n = q + 1;
            }
            1 => {
                acc += mul * (1 + 2 * q);
                n = q + 2;
            }
            _ => {
                acc += mul * (3 + 2 * q);
                n = q;
            }
        }

        mul <<= 1;
    }

    acc + mul * if n == 1 { 1 } else { 3 }
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let q = scan.token::<i64>();

    for _ in 0..q {
        let n = scan.token::<i64>();
        writeln!(out, "{}", calculate(n)).unwrap();
    }
}
