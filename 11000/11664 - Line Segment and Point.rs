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
    z: f64,
}

impl Point {
    fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }

    fn dist(&self, other: Self) -> f64 {
        ((self.x - other.x).powi(2) + (self.y - other.y).powi(2) + (self.z - other.z).powi(2))
            .sqrt()
    }
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let mut a = Point::new(
        scan.token::<f64>(),
        scan.token::<f64>(),
        scan.token::<f64>(),
    );
    let mut b = Point::new(
        scan.token::<f64>(),
        scan.token::<f64>(),
        scan.token::<f64>(),
    );
    let c = Point::new(
        scan.token::<f64>(),
        scan.token::<f64>(),
        scan.token::<f64>(),
    );

    let mut ret = f64::MAX;
    let mut dist_a = a.dist(c);
    let mut dist_b = b.dist(c);

    loop {
        let mid = Point::new(
            (a.x + b.x) / 2.0,
            (a.y + b.y) / 2.0,
            (a.z + b.z) / 2.0,
        );
        let dist = mid.dist(c);

        if (dist - ret).abs() < 1e-9 {
            writeln!(out, "{:.6}", ret).unwrap();
            return;
        }

        ret = ret.min(dist);

        if dist_a < dist_b {
            b = mid;
            dist_b = dist;
        } else {
            a = mid;
            dist_a = dist;
        }       
    }
}
