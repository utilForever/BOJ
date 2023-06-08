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

    let mut points = [(0, 0); 3];

    for i in 0..3 {
        points[i] = (scan.token::<i64>(), scan.token::<i64>());
    }

    if (points[1].0 - points[0].0) * (points[2].1 - points[0].1)
        == (points[2].0 - points[0].0) * (points[1].1 - points[0].1)
    {
        writeln!(out, "{:.9}", -1.0).unwrap();
        return;
    }

    let len_ab = ((points[1].0 - points[0].0) as f64).hypot((points[1].1 - points[0].1) as f64);
    let len_bc = ((points[2].0 - points[1].0) as f64).hypot((points[2].1 - points[1].1) as f64);
    let len_ca = ((points[0].0 - points[2].0) as f64).hypot((points[0].1 - points[2].1) as f64);

    let perimeter1 = (len_ab + len_bc) * 2.0;
    let perimeter2 = (len_bc + len_ca) * 2.0;
    let perimeter3 = (len_ca + len_ab) * 2.0;

    writeln!(
        out,
        "{:.9}",
        perimeter1.max(perimeter2).max(perimeter3) - perimeter1.min(perimeter2).min(perimeter3)
    )
    .unwrap();
}
