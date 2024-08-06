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

    let (n, m, t) = (
        scan.token::<usize>(),
        scan.token::<i64>(),
        scan.token::<f64>(),
    );
    let mut betting_amounts = vec![0.0; n];

    for i in 0..n {
        betting_amounts[i] = scan.token::<f64>();
    }

    if m == 1 {
        let mut x = 0.0f64;
        let mut sum = 0.0;

        for i in 0..n {
            x = x.max(betting_amounts[i].powi(2));
            sum += betting_amounts[i];
        }

        writeln!(out, "{:.9}", (sum * sum - x * t) / sum).unwrap();
    } else {
        let mut x = 0.0f64;
        let mut sum = 0.0;

        for i in 0..n {
            x += betting_amounts[i].powi(2).powf(m as f64 / (m - 1) as f64);
            sum += betting_amounts[i];
        }

        writeln!(
            out,
            "{:.9}",
            (sum * sum - t.powf(1.0 / m as f64) * x.powf((m - 1) as f64 / m as f64)) / sum
        )
        .unwrap();
    }
}
