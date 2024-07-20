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

    let mut nums = Vec::new();

    for i in 1..=9 {
        for j in 1..=9 {
            for k in 1..=9 {
                for l in 1..=9 {
                    let mut nums_local = Vec::new();
                    nums_local.push(vec![i, j, k, l]);
                    nums_local.push(vec![j, k, l, i]);
                    nums_local.push(vec![k, l, i, j]);
                    nums_local.push(vec![l, i, j, k]);

                    nums_local.sort_unstable();
                    nums.push(nums_local[0].clone());
                }
            }
        }
    }

    nums.sort_unstable();
    nums.dedup();

    let (a, b, c, d) = (
        scan.token::<i64>(),
        scan.token::<i64>(),
        scan.token::<i64>(),
        scan.token::<i64>(),
    );

    let mut vals = Vec::new();
    vals.push(vec![a, b, c, d]);
    vals.push(vec![b, c, d, a]);
    vals.push(vec![c, d, a, b]);
    vals.push(vec![d, a, b, c]);

    vals.sort_unstable();

    writeln!(
        out,
        "{}",
        nums.iter().position(|x| *x == vals[0]).unwrap() + 1
    )
    .unwrap();
}
