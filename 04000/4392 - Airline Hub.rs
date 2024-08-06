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

fn distance(latitude1: f64, longitude1: f64, latitude2: f64, longitude2: f64) -> f64 {
    let r = 6371.0;
    let p = std::f64::consts::PI / 180.0;
    let a = 0.5 - ((latitude2 - latitude1) * p).cos() / 2.0
        + ((latitude1 * p).cos()
            * (latitude2 * p).cos()
            * (1.0 - ((longitude2 - longitude1) * p).cos()))
            / 2.0;

    2.0 * r * a.sqrt().asin()
}

// Reference: https://stackoverflow.com/questions/27928/calculate-distance-between-two-latitude-longitude-points-haversine-formula
fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let n = scan.token::<usize>();
    let mut locations = vec![(0.0, 0.0); n];

    for i in 0..n {
        locations[i] = (scan.token::<f64>(), scan.token::<f64>());
    }

    let mut ret = (0, f64::MAX);

    for i in 0..n {
        let (latitude, longitude) = locations[i];
        let mut dist_max = 0.0f64;

        for j in 0..n {
            if i == j {
                continue;
            }

            let dist = distance(latitude, longitude, locations[j].0, locations[j].1);
            dist_max = dist_max.max(dist);
        }

        if dist_max < ret.1 {
            ret = (i, dist_max);
        }
    }

    writeln!(out, "{:.2} {:.2}", locations[ret.0].0, locations[ret.0].1).unwrap();
}
