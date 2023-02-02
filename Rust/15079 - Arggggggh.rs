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
    let (mut x, mut y) = (scan.token::<f64>(), scan.token::<f64>());

    for _ in 0..n - 1 {
        let (dir, d) = (scan.token::<String>(), scan.token::<f64>());

        match dir.as_str() {
            "N" => y += d,
            "NE" => {
                x += d / 2.0_f64.sqrt();
                y += d / 2.0_f64.sqrt();
            }
            "E" => x += d,
            "SE" => {
                x += d / 2.0_f64.sqrt();
                y -= d / 2.0_f64.sqrt();
            }
            "S" => y -= d,
            "SW" => {
                x -= d / 2.0_f64.sqrt();
                y -= d / 2.0_f64.sqrt();
            }
            "W" => x -= d,
            "NW" => {
                x -= d / 2.0_f64.sqrt();
                y += d / 2.0_f64.sqrt();
            }
            _ => unreachable!(),
        }
    }

    writeln!(out, "{:.6} {:.6}", x, y).unwrap();
}
