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

    let ostap = (scan.token::<f64>(), scan.token::<f64>());
    let skumbriewitz = (scan.token::<f64>(), scan.token::<f64>());
    let berlaga = (scan.token::<f64>(), scan.token::<f64>());

    let a = (ostap.0 - berlaga.0, ostap.1 - berlaga.1);
    let b = (skumbriewitz.0 - berlaga.0, skumbriewitz.1 - berlaga.1);

    let r1 = (a.0 * a.0 + a.1 * a.1).sqrt();
    let r2 = (b.0 * b.0 + b.1 * b.1).sqrt();
    let dot = a.0 * b.0 + a.1 * b.1;
    let cross = a.0 * b.1 - a.1 * b.0;
    let delta = cross.abs().atan2(dot);

    let l = ((r2 - r1) / r1).ln_1p();
    let ret = if l.abs() < 1e-12 {
        delta * r1
    } else {
        ((r2 - r1) / l).abs() * delta.hypot(l)
    };

    writeln!(out, "{:.12}", ret).unwrap();
}
