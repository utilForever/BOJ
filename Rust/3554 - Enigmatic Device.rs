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
    let mut sequence = vec![0; n + 1];
    let mut sum = vec![0; n + 1];
    let mut index = 1;

    for i in 1..=n {
        sequence[i] = scan.token::<i64>();
    }

    let m = scan.token::<i64>();

    for _ in 0..m {
        let (k, l, r) = (
            scan.token::<i64>(),
            scan.token::<usize>(),
            scan.token::<usize>(),
        );

        if k == 1 {
            for val in &mut sequence[l..=r] {
                *val = (*val * *val) % 2010;
            }

            index = index.min(l);
        } else {
            if index - 1 < r {
                for (val, idx) in sequence.iter().take(r + 1).skip(index).zip(index..) {
                    sum[idx] = sum[idx - 1] + val;
                }

                index = r;
            }

            writeln!(out, "{}", sum[r] - sum[l - 1]).unwrap();
        }
    }
}
