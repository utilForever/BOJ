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

    let (l, d, x) = (
        scan.token::<i64>(),
        scan.token::<i64>(),
        scan.token::<i64>(),
    );
    let mut n = 0;
    let mut m = 0;

    for i in l..=d {
        let mut sum = 0;
        let mut j = i;

        while j > 0 {
            sum += j % 10;
            j /= 10;
        }

        if sum == x {
            n = i;
            break;
        }
    }

    for i in (l..=d).rev() {
        let mut sum = 0;
        let mut j = i;

        while j > 0 {
            sum += j % 10;
            j /= 10;
        }

        if sum == x {
            m = i;
            break;
        }
    }

    writeln!(out, "{n}").unwrap();
    writeln!(out, "{m}").unwrap();
}
