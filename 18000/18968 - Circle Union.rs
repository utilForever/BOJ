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

    pub fn all(&mut self) -> String {
        let mut input = String::new();
        self.reader.read_to_string(&mut input).expect("Failed read");
        input
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
    let mut radii = vec![0.0; n];
    let mut radius_max = 0.0;

    for i in 0..n {
        radii[i] = scan.token::<f64>();

        if radii[i] > radius_max {
            radius_max = radii[i];
        }
    }

    let mut h = 0.0;

    if n >= 3 {
        let mut left = 0.0;
        let mut right = radius_max;

        for _ in 0..100 {
            let mid = (left + right) / 2.0;
            let mut sum = 0.0;

            for &radius in radii.iter() {
                if radius > mid {
                    sum += (mid / radius).acos();
                }
            }

            if sum > std::f64::consts::PI {
                left = mid;
            } else {
                right = mid;
            }
        }

        h = (left + right) / 2.0;
    }

    let mut ret = 0.0;

    for &radius in radii.iter() {
        if radius > h {
            let x = (h / radius).clamp(0.0, 1.0);
            let a = x.acos();
            let s = (1.0 - x * x).max(0.0).sqrt();

            ret += 2.0 * radius * radius * (a + x * s);
        }
    }

    writeln!(out, "{:.12}", ret).unwrap();
}
