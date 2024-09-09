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

fn calculate1(a: f64, b: f64, c: f64) -> f64 {
    (b.powi(2) + c.powi(2) - a.powi(2)) / (2.0 * b * c)
}

fn calculate2(t: f64, a: f64) -> f64 {
    a.powi(2) * (t - t.sin()) / 2.0
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let (a, b, c) = (
        scan.token::<f64>(),
        scan.token::<f64>(),
        scan.token::<f64>(),
    );
    let t1 = calculate1(a, b, c).acos();
    let t2 = calculate1(b, c, a).acos();

    let ret = if a.powi(2) + b.powi(2) > c.powi(2) {
        let val1 = calculate2(std::f64::consts::PI - 2.0 * t1, a)
            + calculate2(std::f64::consts::PI - 2.0 * t2, b)
            + calculate2(2.0 * t1 + 2.0 * t2 - std::f64::consts::PI, c);

        let aa = 2.0 * a * (std::f64::consts::FRAC_PI_2 - t1).sin();
        let bb = 2.0 * b * (std::f64::consts::FRAC_PI_2 - t2).sin();
        let cc = 2.0 * c * (t1 + t2 - std::f64::consts::FRAC_PI_2).sin();
        let s = (aa + bb + cc) / 2.0;

        let val2 = (s * (s - aa) * (s - bb) * (s - cc)).sqrt();

        val1 + val2
    } else {
        calculate2(2.0 * t1, b) + calculate2(2.0 * t2, a)
    };

    writeln!(out, "{:.9}", ret).unwrap();
}
