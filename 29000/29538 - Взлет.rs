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

    let (m, n, a) = (
        scan.token::<i64>(),
        scan.token::<usize>(),
        scan.token::<i64>(),
    );
    let mut weights = vec![0; n];

    for i in 0..n {
        weights[i] = scan.token::<i64>();
    }

    if a == 1000 {
        writeln!(out, "Impossible").unwrap();
        return;
    }

    let ret = ((m + weights.iter().sum::<i64>()) as f64) / (1000.0 / a as f64 - 1.0);

    writeln!(out, "{:.9}", ret).unwrap();
}
