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

    let (h_lo, h_hi) = (scan.token::<f64>(), scan.token::<f64>());
    let (s_lo, s_hi) = (scan.token::<f64>(), scan.token::<f64>());
    let (v_lo, v_hi) = (scan.token::<f64>(), scan.token::<f64>());
    let (r, g, b) = (
        scan.token::<f64>(),
        scan.token::<f64>(),
        scan.token::<f64>(),
    );

    let max = r.max(g).max(b);
    let min = r.min(g).min(b);

    let v = max;
    let s = 255.0 * ((v - min) / v);
    let mut h = if max == r {
        (60.0 * (g - b)) / (v - min)
    } else if max == g {
        (60.0 * (b - r)) / (v - min) + 120.0
    } else {
        (60.0 * (r - g)) / (v - min) + 240.0
    };

    if h < 0.0 {
        h += 360.0;
    }

    writeln!(
        out,
        "{}",
        if h_lo <= h && h <= h_hi && s_lo <= s && s <= s_hi && v_lo <= v && v <= v_hi {
            "Lumi will like it."
        } else {
            "Lumi will not like it."
        }
    )
    .unwrap();
}
