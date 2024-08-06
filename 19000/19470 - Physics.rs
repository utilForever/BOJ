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

    let (n, c) = (scan.token::<usize>(), scan.token::<i64>() as f64);
    let mut velocities = vec![0; n];

    for i in 0..n {
        velocities[i] = scan.token::<i64>();
        _ = scan.token::<i64>();
        _ = scan.token::<i64>();
    }

    velocities.sort();

    let q = scan.token::<i64>();

    for _ in 0..q {
        let (t, k) = (scan.token::<i64>() as f64, scan.token::<usize>());

        writeln!(
            out,
            "{:.10}",
            (2.0 * c * t + (velocities[k - 1] * velocities[k - 1]) as f64).sqrt()
        )
        .unwrap();
    }
}
