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

    let (w, h, n) = (
        scan.token::<usize>(),
        scan.token::<usize>(),
        scan.token::<usize>(),
    );
    let ratio = h as f64 / w as f64;
    let mut a = vec![0.0; n + 2];
    let mut k = vec![0.0; n + 2];
    let mut x = vec![0.0; n + 2];

    // f(x) = a1(x1 - k1x2)^2 + a2(x2 - k2x3)^2 + ... + an(xn - knw)^2 + C
    // where a1 = 0.5, ki = 0.25 * (1 - ratio^2) / ai, ai+1 = 0.5 - aiki^2
    a[1] = 0.5;
    for i in 1..=n {
        k[i] = 0.25 * (1.0 - ratio.powi(2)) / a[i];
        a[i + 1] = 0.5 - a[i] * k[i].powi(2);
    }

    // The minimum overall excavation cost is C
    let ret = (a[n + 1] - 0.5 + 0.25 * (ratio + 1.0).powi(2)) * (w as f64).powi(2);

    x[n + 1] = w as f64;
    for i in (1..=n).rev() {
        x[i] = x[i + 1] * k[i];
    }

    writeln!(out, "{}", ret).unwrap();

    for i in 1..=n {
        if i > 10 {
            break;
        }

        writeln!(out, "{}", x[i]).unwrap();
    }
}
