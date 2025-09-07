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

    let t = scan.token::<i64>();

    for _ in 0..t {
        let (xa, ya, xc, yc, v) = (
            scan.token::<i64>(),
            scan.token::<i64>(),
            scan.token::<i64>(),
            scan.token::<i64>(),
            scan.token::<i64>(),
        );

        if xa == xc && ya == yc {
            writeln!(out, "{:.9}", 0).unwrap();
            continue;
        }

        if v == 1 {
            writeln!(out, "{:.9}", -1).unwrap();
            continue;
        }

        let d = (((xa - xc).pow(2) + (ya - yc).pow(2)) as f64).sqrt();
        let denominator = (v as f64).powi(2) - 1.0;
        let theta = (1.0 / v as f64).acos();
        let apollo = ((v as f64).powi(2) * d.powi(2) / denominator.powi(2))
            * (theta - theta.sin() * theta.cos());
        let spiral = d.powi(2) * ((std::f64::consts::PI / denominator.sqrt()).exp() - 1.0)
            / (2.0 * denominator.sqrt());
        let ret = apollo + spiral;

        writeln!(out, "{:.9}", ret).unwrap();
    }
}
