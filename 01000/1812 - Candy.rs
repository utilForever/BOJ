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

    let n = scan.token::<usize>();
    let mut candies_sum = vec![0; n];

    for i in 0..n {
        candies_sum[i] = scan.token::<i64>();
    }

    let sum_total = candies_sum.iter().sum::<i64>() / 2;
    let sum_odd = candies_sum.iter().step_by(2).sum::<i64>();
    let candy_first = sum_odd - sum_total;

    let mut ret = vec![0; n];
    ret[0] = candy_first;

    for i in 1..n {
        ret[i] = candies_sum[i - 1] - ret[i - 1];
    }

    for candy in ret {
        writeln!(out, "{candy}").unwrap();
    }
}
