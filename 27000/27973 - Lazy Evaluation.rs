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

    let q = scan.token::<i64>();
    let mut offset_plus = 0;
    let mut offset_multiply = 1;
    let mut num = 1;

    for _ in 0..q {
        let command = scan.token::<i64>();

        if command == 0 {
            let x = scan.token::<i64>();
            offset_plus += x;
        } else if command == 1 {
            let x = scan.token::<i64>();
            offset_multiply *= x;
            offset_plus *= x;
        } else if command == 2 {
            let n = scan.token::<i64>();
            num += n;
        } else {
            writeln!(out, "{}", num * offset_multiply + offset_plus).unwrap();
        }
    }
}
