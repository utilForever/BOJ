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
    let mut wealths = vec![0; n];
    let mut sum = 0;

    for i in 0..n {
        wealths[i] = scan.token::<i64>();
        sum += wealths[i];
    }

    wealths.sort_unstable_by(|a, b| b.cmp(a));

    let mut sum_local = 0;
    let mut ret = 0.0f64;

    for i in 0..n {
        sum_local += wealths[i];

        let x = (i + 1) as f64 / n as f64;
        let y = sum_local as f64 / sum as f64;

        ret = ret.max(y - x);
    }

    writeln!(out, "{:.9}", ret * 100.0).unwrap();
}
