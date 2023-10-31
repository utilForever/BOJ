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

    let n = scan.token::<i64>();
    let mut prices = [0; 4];

    for _ in 0..n {
        let price = scan.token::<usize>();
        prices[price] += 1;
    }

    let mut ret = 0;

    // 0 XOR 3 and 1 XOR 2 is first
    while prices[0] > 0 && prices[3] > 0 {
        prices[0] -= 1;
        prices[3] -= 1;
        ret += 3;
    }

    while prices[1] > 0 && prices[2] > 0 {
        prices[1] -= 1;
        prices[2] -= 1;
        ret += 3;
    }

    for i in 0..3 {
        for j in i + 1..4 {
            while prices[i] > 0 && prices[j] > 0 {
                prices[i] -= 1;
                prices[j] -= 1;
                ret += i ^ j;
            }
        }
    }

    writeln!(out, "{ret}").unwrap();
}
