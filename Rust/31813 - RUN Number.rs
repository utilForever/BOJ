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

    let t = scan.token::<i64>();

    for _ in 0..t {
        let (mut n, mut k) = (scan.token::<u32>(), scan.token::<i64>());
        let mut ret = Vec::new();

        while k > 9 {
            let digit = k / 10i64.pow(n - 1);

            if digit == 0 {
                n -= 1;
                continue;
            }

            let mut num = 0;

            for i in 0..n {
                num += digit * 10i64.pow(i);
            }

            if k >= num {
                ret.push(num);
                k -= num;
                n -= 1;
            } else {
                let mut num = 0;

                if digit == 1 {
                    for i in 0..n - 1 {
                        num += 9 * 10i64.pow(i);
                    }
                } else {
                    for i in 0..n {
                        num += (digit - 1) * 10i64.pow(i);
                    }
                }

                ret.push(num);
                k -= num;
            }
        }

        if k > 0 {
            ret.push(k);
        }

        writeln!(out, "{}", ret.len()).unwrap();

        for val in ret {
            write!(out, "{} ", val).unwrap();
        }

        writeln!(out).unwrap();
    }
}
