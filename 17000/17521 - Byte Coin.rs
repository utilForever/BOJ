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

    let (n, w) = (scan.token::<usize>(), scan.token::<i64>());
    let mut prices = vec![0; n];

    for i in 0..n {
        prices[i] = scan.token::<i64>();
    }

    prices.push(0);

    let mut curr_money = w;
    let mut price_min = i64::MAX;

    for i in 0..n {
        if prices[i] < price_min {
            price_min = prices[i];
        }

        if prices[i] > prices[i + 1] {
            let coin_buy = curr_money / price_min;
            curr_money -= coin_buy * price_min;

            curr_money += coin_buy * prices[i];
            price_min = i64::MAX;
        }
    }

    writeln!(out, "{curr_money}").unwrap();
}
