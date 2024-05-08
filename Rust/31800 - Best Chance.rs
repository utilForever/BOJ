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
    let mut profits = vec![0; n];
    let mut prices = vec![0; n];

    let mut max_first = 0;
    let mut max_second = 0;

    for i in 0..n {
        prices[i] = scan.token::<i64>();

        if prices[i] > max_first {
            max_second = max_first;
            max_first = prices[i];
        } else if prices[i] > max_second {
            max_second = prices[i];
        }
    }

    if max_second == 0 {
        max_second = max_first;
    }

    for i in 0..n {
        profits[i] = scan.token::<i64>();
    }

    for i in 0..n {
        write!(
            out,
            "{} ",
            if prices[i] == max_first {
                prices[i] - max_second
            } else {
                prices[i] - max_first
            }
        )
        .unwrap();
    }

    writeln!(out).unwrap();
}
