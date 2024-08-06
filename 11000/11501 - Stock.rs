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

    let t = scan.token::<i64>();

    for _ in 0..t {
        let n = scan.token::<usize>();
        let mut stocks = vec![0; n];
        let mut stocks_max = vec![0; n];

        for i in 0..n {
            stocks[i] = scan.token::<i64>();
        }

        stocks_max[n - 1] = stocks[n - 1];

        for i in (0..n - 1).rev() {
            stocks_max[i] = stocks[i].max(stocks_max[i + 1]);
        }

        let mut ret = 0;

        for i in 0..n {
            if stocks[i] < stocks_max[i] {
                ret += stocks_max[i] - stocks[i];
            }
        }

        writeln!(out, "{ret}").unwrap();
    }
}
