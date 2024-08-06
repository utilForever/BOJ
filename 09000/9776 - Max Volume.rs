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

    let n = scan.token::<i64>();
    let mut ret = 0.0_f64;

    for _ in 0..n {
        let c = scan.token::<String>();

        match c.as_str() {
            "C" => {
                let (r, h) = (scan.token::<f64>(), scan.token::<f64>());
                let volume = std::f64::consts::PI * r * r * h;
                ret = ret.max(volume);
            }
            "L" => {
                let (r, h) = (scan.token::<f64>(), scan.token::<f64>());
                let volume = std::f64::consts::PI * r * r * h / 3.0;
                ret = ret.max(volume);
            }
            "S" => {
                let r = scan.token::<f64>();
                let volume = 4.0 * std::f64::consts::PI * r * r * r / 3.0;
                ret = ret.max(volume);
            }
            _ => unreachable!(),
        }
    }

    writeln!(out, "{:.3}", ret).unwrap();
}
