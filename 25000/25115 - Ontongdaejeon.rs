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

    pub fn all(&mut self) -> String {
        let mut input = String::new();
        self.reader.read_to_string(&mut input).expect("Failed read");
        input
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

    let n = scan.token::<usize>();
    let mut prices = vec![0; n];

    for i in 0..n {
        prices[i] = scan.token::<i64>();
    }

    let mut left = 0;
    let mut right = prices.iter().sum::<i64>();
    let mut ret = right;

    while left <= right {
        let mid = (left + right) / 2;
        let mut remain = mid;
        let mut cashback = 0;
        let mut can_buy = true;

        for &price in prices.iter() {
            if remain >= price {
                remain -= price;
                cashback += price;
            } else {
                if price - remain <= cashback / 10 {
                    cashback += remain;
                    cashback -= (price - remain) * 10;
                    remain = 0;
                } else {
                    can_buy = false;
                    break;
                }
            }
        }

        if can_buy {
            ret = mid;
            right = mid - 1;
        } else {
            left = mid + 1;
        }
    }

    writeln!(out, "{ret}").unwrap();
}
