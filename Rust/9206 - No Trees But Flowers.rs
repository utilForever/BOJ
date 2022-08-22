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

fn process_simpsons_rule(a: f64, b: f64, left: f64, right: f64) -> f64 {
    let fx = |x: f64| -> f64 {
        let val = a * (-1.0 * x * x).exp() + b * x.sqrt();
        std::f64::consts::PI * val * val
    };

    let approx = |left: f64, right: f64| -> f64 {
        let mid = (left + right) / 2.0;
        (fx(left) + 4.0 * fx(mid) + fx(right)) * (right - left) / 6.0
    };

    let sum = approx(left, right);
    let mid = (left + right) / 2.0;
    let sum_left = approx(left, mid);
    let sum_right = approx(mid, right);

    if (sum - sum_left - sum_right).abs() < 1e-5 {
        sum
    } else {
        process_simpsons_rule(a, b, left, mid) + process_simpsons_rule(a, b, mid, right)
    }
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let (v, n) = (scan.token::<f64>(), scan.token::<usize>());
    let mut flower_pots = vec![0.0; n];

    for i in 0..n {
        let (a, b, h) = (
            scan.token::<f64>(),
            scan.token::<f64>(),
            scan.token::<f64>(),
        );
        flower_pots[i] = process_simpsons_rule(a, b, 0.0, h);
    }

    let mut diff = f64::MAX;
    let mut ret = 0;

    for i in 0..n {
        if (v - flower_pots[i]).abs() < diff {
            diff = (v - flower_pots[i]).abs();
            ret = i;
        }
    }

    writeln!(out, "{}", ret).unwrap();
}
