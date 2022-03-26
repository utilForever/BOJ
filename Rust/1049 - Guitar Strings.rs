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

    let (mut n, m) = (scan.token::<usize>(), scan.token::<usize>());
    let mut min_price_piece = usize::MAX;
    let mut min_price_bundle = usize::MAX;

    for _ in 0..m {
        min_price_bundle = min_price_bundle.min(scan.token::<usize>());
        min_price_piece = min_price_piece.min(scan.token::<usize>());
    }

    let mut ans = if min_price_bundle < min_price_piece * 6 {
        min_price_bundle * (n / 6)
    } else {
        min_price_piece * (n / 6) * 6
    };

    n %= 6;

    ans += if min_price_bundle < min_price_piece * n {
        min_price_bundle
    } else {
        min_price_piece * n
    };

    writeln!(out, "{}", ans).unwrap();
}
