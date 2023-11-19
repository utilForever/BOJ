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

    // 4, 6, 8
    // 8, 10, 10, 12, 12
    // 12, 14, 14, 14, 16, 16, 16
    // ...

    let t = scan.token::<i64>();

    for _ in 0..t {
        let n = scan.token::<i64>();

        let pos_start = (n as f64).sqrt() as i64;
        let mut pos_rest = n - pos_start * pos_start;
        let mut ret = 4 * pos_start;

        if pos_rest == 0 {
            writeln!(out, "{ret}").unwrap();
        } else {
            while pos_rest > 0 {
                ret += 2;
                pos_rest -= pos_start;
            }

            writeln!(out, "{ret}").unwrap();
        }
    }
}
