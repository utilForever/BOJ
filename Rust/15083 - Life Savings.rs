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

    let mut prices = [0; 3];
    let mut coupons = [0; 3];

    for i in 0..3 {
        prices[i] = scan.token::<i64>();
    }

    for i in 0..3 {
        coupons[i] = scan.token::<i64>();
    }

    if coupons[1] < coupons[2] {
        coupons.swap(1, 2);
    }

    prices.sort_by(|a, b| b.cmp(a));

    let price_total = prices.iter().sum::<i64>() as f64;
    let price_entire = ((100 - coupons[0]) * prices.iter().sum::<i64>()) as f64 / 100.0;
    let price_each_item =
        ((100 - coupons[1]) * prices[0] + (100 - coupons[2]) * prices[1] + 100 * prices[2]) as f64
            / 100.0;

    if price_entire < price_each_item {
        writeln!(out, "one {:.2}", price_total - price_entire).unwrap();
    } else {
        writeln!(out, "two {:.2}", price_total - price_each_item).unwrap();
    }
}
