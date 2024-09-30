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

    let (n, u, d) = (
        scan.token::<usize>(),
        scan.token::<i64>(),
        scan.token::<i64>(),
    );
    let mut a = vec![0; n];
    let mut b = vec![0; n];

    for i in 0..n {
        a[i] = scan.token::<i64>();
    }

    for i in 0..n {
        b[i] = scan.token::<i64>();
    }

    let mut prefix_sum_a = vec![0; n + 1];
    let mut prefix_sum_b = vec![0; n + 1];

    for i in 0..n {
        let mut idx = i;

        // Calculate the number of steps after which choosing B_i becomes advantageous
        if a[i] <= b[i] {
            let delta = b[i] - a[i];
            let steps = ((delta + u + d - 1) / (u + d)) as usize;

            idx = (idx + steps).min(n);

            prefix_sum_a[i] += u;
            prefix_sum_a[idx] -= u;

            prefix_sum_b[i] += a[i] - (i as i64) * u;
            prefix_sum_b[idx] -= a[i] - (i as i64) * u;
        }

        prefix_sum_a[idx] -= d;
        prefix_sum_b[idx] += b[i] + (i as i64) * d;
    }

    for i in 0..n {
        if i > 0 {
            prefix_sum_a[i] += prefix_sum_a[i - 1];
            prefix_sum_b[i] += prefix_sum_b[i - 1];
        }

        let ret = prefix_sum_a[i] * (i as i64) + prefix_sum_b[i];

        writeln!(out, "{ret}").unwrap();
    }
}
