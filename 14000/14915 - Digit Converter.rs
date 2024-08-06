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

    let (mut n, m) = (scan.token::<i64>(), scan.token::<i64>());
    let mut ret = String::new();

    if n == 0 {
        writeln!(out, "0").unwrap();
        return;
    }

    while n > 0 {
        let q = n / m;
        let r = n % m;

        match r {
            num @ 0..=9 => ret.insert_str(0, &num.to_string()),
            10 => ret.insert(0, 'A'),
            11 => ret.insert(0, 'B'),
            12 => ret.insert(0, 'C'),
            13 => ret.insert(0, 'D'),
            14 => ret.insert(0, 'E'),
            15 => ret.insert(0, 'F'),
            _ => unreachable!(),
        }

        n = q;
    }

    writeln!(out, "{ret}").unwrap();
}
