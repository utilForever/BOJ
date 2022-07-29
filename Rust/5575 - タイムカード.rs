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

    for _ in 0..3 {
        let (h1, m1, s1) = (
            scan.token::<i32>(),
            scan.token::<i32>(),
            scan.token::<i32>(),
        );
        let (h2, m2, s2) = (
            scan.token::<i32>(),
            scan.token::<i32>(),
            scan.token::<i32>(),
        );

        let mut ret_s = (s2 - s1) % 60;
        let mut ret_m = (m2 - m1) % 60;
        let mut ret_h = (h2 - h1) % 24;

        if ret_s < 0 {
            ret_s += 60;
            ret_m -= 1;
        }

        if ret_m < 0 {
            ret_m += 60;
            ret_h -= 1;
        }

        writeln!(out, "{} {} {}", ret_h, ret_m, ret_s).unwrap();
    }
}
