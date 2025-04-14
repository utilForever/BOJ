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

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let t = scan.token::<i64>();

    for _ in 0..t {
        let (n, m) = (scan.token::<i64>(), scan.token::<i64>());
        let mut digits = vec![0; 1001];
        let mut len = 1;

        digits[0] = 1;

        for i in 1..=n {
            for j in 0..len {
                digits[j] *= i;
            }

            let mut idx = 0;

            while idx < len {
                if digits[idx] >= 10 {
                    digits[idx + 1] += digits[idx] / 10;
                    digits[idx] %= 10;

                    if idx + 1 == len {
                        len += 1;
                    }
                }

                idx += 1;
            }
        }

        let mut ret = 0;

        for i in 0..len {
            if digits[i] == m {
                ret += 1;
            }
        }

        writeln!(out, "{ret}").unwrap();
    }
}
