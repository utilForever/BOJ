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

    let yonsei = ['Y', 'O', 'N', 'S', 'E', 'I'];
    let korea = ['K', 'O', 'R', 'E', 'A'];

    let s = scan.token::<String>();
    let s = s.chars().collect::<Vec<_>>();
    let mut idx_yonsei = 0;
    let mut idx_korea = 0;

    for c in s {
        if c == yonsei[idx_yonsei] {
            idx_yonsei += 1;
        }

        if c == korea[idx_korea] {
            idx_korea += 1;
        }

        if idx_yonsei == 6 {
            writeln!(out, "YONSEI").unwrap();
            return;
        }

        if idx_korea == 5 {
            writeln!(out, "KOREA").unwrap();
            return;
        }
    }
}
