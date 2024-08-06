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
    let mut sequence = vec![0; n];
    let mut longest_increase = vec![1; n];
    let mut longest_decrease = vec![1; n];

    for i in 0..n {
        sequence[i] = scan.token::<i64>();
    }

    for i in 1..n {
        if sequence[i] >= sequence[i - 1] {
            longest_increase[i] = longest_increase[i].max(longest_increase[i - 1] + 1);
        }

        if sequence[i] <= sequence[i - 1] {
            longest_decrease[i] = longest_decrease[i].max(longest_decrease[i - 1] + 1);
        }
    }

    let max_increase = longest_increase.iter().max().unwrap();
    let max_decrease = longest_decrease.iter().max().unwrap();

    writeln!(out, "{}", max_increase.max(max_decrease)).unwrap();
}
