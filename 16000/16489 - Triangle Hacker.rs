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

    let (a, b, c) = (
        scan.token::<f64>(),
        scan.token::<f64>(),
        scan.token::<f64>(),
    );
    let s = (a + b + c) / 2.0;
    let s_area_square = s * (s - a) * (s - b) * (s - c);
    let r_outer_square = (a.powi(2) * b.powi(2) * c.powi(2)) / (16.0 * s_area_square);
    let r_inner_square = s_area_square / (s * s);
    let (r_outer, r_inner) = (r_outer_square.sqrt(), r_inner_square.sqrt());
    let d = if r_outer >= 2.0 * r_inner {
        (r_outer * (r_outer - 2.0 * r_inner)).sqrt()
    } else {
        0.0
    };

    let a2 = a * a;
    let b2 = b * b;
    let c2 = c * c;

    let cos_a = (b2 + c2 - a2) / (2.0 * b * c);
    let cos_b = (a2 + c2 - b2) / (2.0 * a * c);
    let cos_c = (a2 + b2 - c2) / (2.0 * a * b);
    let k = r_outer * (cos_a + cos_b + cos_c);

    writeln!(out, "{:.12}", s_area_square.sqrt()).unwrap();
    writeln!(out, "{:.12}", r_outer).unwrap();
    writeln!(out, "{:.12}", r_inner).unwrap();
    writeln!(out, "{:.12}", d).unwrap();
    writeln!(out, "{:.12}", k).unwrap();
}
