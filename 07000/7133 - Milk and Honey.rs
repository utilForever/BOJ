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

    let (m, dm) = (scan.token::<i64>(), scan.token::<i64>());
    let (h, dh) = (scan.token::<i64>(), scan.token::<i64>());
    let n = scan.token::<i64>();
    let mut ret = 0;

    for _ in 0..n {
        let c = scan.token::<i64>();
        let d = scan.token::<i64>();
        let mut sum_c = 0;
        let mut sum_d = 0;

        for i in 0..c {
            sum_c += (m - i * dm).max(0);
        }

        for i in 0..d {
            sum_d += (h - i * dh).max(0);
        }

        ret += sum_c.max(sum_d);
    }

    writeln!(out, "{ret}").unwrap();
}
