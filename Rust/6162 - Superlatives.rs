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

    let k = scan.token::<i64>();

    for i in 1..=k {
        let (mut e, a) = (scan.token::<i64>(), scan.token::<i64>());

        writeln!(out, "Data Set {i}:").unwrap();

        if e <= a {
            writeln!(out, "no drought").unwrap();
        } else {
            let mut cnt = 0;

            while e > a * 5 {
                e /= 5;
                cnt += 1;
            }

            for _ in 0..cnt {
                write!(out, "mega ").unwrap();
            }

            writeln!(out, "drought").unwrap();
        }

        if i != k {
            writeln!(out).unwrap();
        }
    }
}
