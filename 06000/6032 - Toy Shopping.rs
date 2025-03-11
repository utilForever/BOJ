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
    let mut metrics = vec![(0.0, 0, 0); n];

    for i in 0..n {
        let (j, p) = (scan.token::<i64>(), scan.token::<i64>());
        metrics[i] = (j as f64 / p as f64, p, i);
    }

    metrics.sort_by(|a, b| b.0.partial_cmp(&a.0).unwrap());

    writeln!(out, "{}", metrics[0].1 + metrics[1].1 + metrics[2].1).unwrap();

    for i in 0..3 {
        writeln!(out, "{} ", metrics[i].2 + 1).unwrap();
    }
}
