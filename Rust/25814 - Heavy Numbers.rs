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

    let (mut a, mut b) = (scan.token::<i64>(), scan.token::<i64>());

    let mut num_digit_a = 0;
    let mut num_digit_b = 0;
    let mut sum_digit_a = 0;
    let mut sum_digit_b = 0;

    while a > 0 {
        num_digit_a += 1;
        sum_digit_a += a % 10;
        a /= 10;
    }

    while b > 0 {
        num_digit_b += 1;
        sum_digit_b += b % 10;
        b /= 10;
    }

    let weight_a = num_digit_a * sum_digit_a;
    let weight_b = num_digit_b * sum_digit_b;

    writeln!(
        out,
        "{}",
        match weight_a.cmp(&weight_b) {
            std::cmp::Ordering::Less => 2,
            std::cmp::Ordering::Equal => 0,
            std::cmp::Ordering::Greater => 1,
        }
    )
    .unwrap();
}
