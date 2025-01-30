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
    let mut snacks = vec![0; n];

    for i in 0..n {
        snacks[i] = scan.token::<usize>();
    }

    let mut expectation = vec![0.0; n + 1];
    let mut prefix_sum = vec![0.0; n + 1];

    for i in 1..=n {
        let snack = snacks[i - 1];
        let idx = if i >= snack { i - snack } else { 0 };
        let expect = if idx == 0 {
            prefix_sum[i - 1]
        } else {
            prefix_sum[i - 1] - prefix_sum[idx - 1]
        };

        expectation[i] = 1.0 + expect / snack as f64;
        prefix_sum[i] = prefix_sum[i - 1] + expectation[i];
    }

    writeln!(out, "{:.9}", expectation[n]).unwrap();
}
