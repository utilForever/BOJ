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

const PRICE_APPLE: i64 = 4000;
const SIZE_APPLE: i64 = 12;
const WEIGHT_APPLE: i64 = 500;
const WEIGHT_BOX: i64 = 1000;
const WEIGHT_PEAR: i64 = 120;
const COUNT_PEAR: i64 = 50;

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let n = scan.token::<i64>();
    let mut total_weight_box = 0;
    let mut total_price_apple = 0;

    for _ in 0..n {
        let (t, w, h, l) = (
            scan.token::<char>(),
            scan.token::<i64>(),
            scan.token::<i64>(),
            scan.token::<i64>(),
        );

        if t == 'A' {
            let count_apple = (w / SIZE_APPLE) * (h / SIZE_APPLE) * (l / SIZE_APPLE);
            total_weight_box += WEIGHT_BOX + count_apple * WEIGHT_APPLE;
            total_price_apple += count_apple * PRICE_APPLE;
        } else {
            total_weight_box += COUNT_PEAR * WEIGHT_PEAR;
        }
    }

    writeln!(out, "{total_weight_box}").unwrap();
    writeln!(out, "{total_price_apple}").unwrap();
}
