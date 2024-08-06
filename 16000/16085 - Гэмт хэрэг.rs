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

    for _ in 0..n {
        let (t, w, c) = (
            scan.token::<i64>(),
            scan.token::<i64>(),
            scan.token::<i64>(),
        );

        if w >= c {
            writeln!(
                out,
                "{:.3}",
                std::f64::consts::PI * ((w * t) as f64).powi(2)
            )
            .unwrap();
        } else {
            let base = (((c * t) as f64).powi(2) - ((w * t) as f64).powi(2)).sqrt();
            let mut ret = std::f64::consts::PI * ((w * t) as f64).powi(2);

            ret += base * (w * t) as f64 * 2.0;
            ret -= ((w * t) as f64 / (c * t) as f64).acos() * ((w * t) as f64).powi(2) * 2.0;

            writeln!(out, "{:.3}", ret).unwrap();
        }
    }
}
