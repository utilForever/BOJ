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

#[derive(Copy, Clone)]
struct Point {
    x: f64,
    y: f64,
}

impl Point {
    fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }

    fn dist(&self, other: Self) -> f64 {
        ((self.x - other.x).powi(2) + (self.y - other.y).powi(2)).sqrt()
    }
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let a = Point::new(scan.token::<f64>(), scan.token::<f64>());
    let b = Point::new(scan.token::<f64>(), scan.token::<f64>());
    let c = Point::new(scan.token::<f64>(), scan.token::<f64>());
    let d = Point::new(scan.token::<f64>(), scan.token::<f64>());

    let mut ret = f64::MAX;
    let mut p_left = 0.0;
    let mut p_right = 100.0;

    loop {
        let p1 = (2.0 * p_left + p_right) / 3.0;
        let p2 = (p_left + 2.0 * p_right) / 3.0;

        let p1_ab = Point::new(
            a.x + (b.x - a.x) * p1 / 100.0,
            a.y + (b.y - a.y) * p1 / 100.0,
        );
        let p2_ab = Point::new(
            a.x + (b.x - a.x) * p2 / 100.0,
            a.y + (b.y - a.y) * p2 / 100.0,
        );
        let p1_cd = Point::new(
            c.x + (d.x - c.x) * p1 / 100.0,
            c.y + (d.y - c.y) * p1 / 100.0,
        );
        let p2_cd = Point::new(
            c.x + (d.x - c.x) * p2 / 100.0,
            c.y + (d.y - c.y) * p2 / 100.0,
        );

        let dist_p1 = p1_ab.dist(p1_cd);
        let dist_p2 = p2_ab.dist(p2_cd);
        ret = ret.min(dist_p1).min(dist_p2);

        if dist_p1 <= dist_p2 {
            p_right = p2;
        } else {
            p_left = p1;
        }

        if p_right - p_left <= 1e-9 {
            writeln!(out, "{:.6}", ret).unwrap();
            return;
        }
    }
}
