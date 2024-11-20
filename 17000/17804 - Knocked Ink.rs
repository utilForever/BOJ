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

#[inline]
fn sign(x: f64) -> i64 {
    if x < -1e-9 {
        -1
    } else if x > 1e-9 {
        1
    } else {
        0
    }
}

#[inline]
fn dist(x1: f64, y1: f64, x2: f64, y2: f64) -> f64 {
    ((x1 - x2) * (x1 - x2) + (y1 - y2) * (y1 - y2)).sqrt()
}

#[inline]
fn angle(a: f64, b: f64, c: f64) -> f64 {
    let mut val = (a * a + b * b - c * c) / (2.0 * a * b);
    val = val.clamp(-1.0, 1.0);

    val.acos()
}

struct CircleUnion {
    n: usize,
    x: Vec<f64>,
    y: Vec<f64>,
    r: Vec<f64>,
    segments: Vec<(f64, f64)>,
    arc: f64,
    polygon: f64,
}

impl CircleUnion {
    fn new(n: usize) -> Self {
        Self {
            n,
            x: Vec::with_capacity(n),
            y: Vec::with_capacity(n),
            r: Vec::with_capacity(n),
            segments: Vec::with_capacity(n * 2),
            arc: 0.0,
            polygon: 0.0,
        }
    }

    pub fn add(&mut self, x: f64, y: f64, r: f64) {
        self.x.push(x);
        self.y.push(y);
        self.r.push(r);
    }

    pub fn area(&mut self, idx: usize, left: f64, right: f64) {
        self.arc += 0.5 * self.r[idx] * self.r[idx] * (right - left - (right - left).sin());

        let x1 = self.x[idx] + self.r[idx] * left.cos();
        let y1 = self.y[idx] + self.r[idx] * left.sin();
        let x2 = self.x[idx] + self.r[idx] * right.cos();
        let y2 = self.y[idx] + self.r[idx] * right.sin();

        self.polygon += x1 * y2 - x2 * y1;
    }

    pub fn union(&mut self, idx_except: usize) -> f64 {
        self.arc = 0.0;
        self.polygon = 0.0;

        let mut r = self.r.clone();
        let mut is_covered = vec![false; self.n];

        if idx_except != self.n {
            r[idx_except] = 0.0;
        }

        for i in 0..self.n {
            for j in 0..i {
                if sign(self.x[i] - self.x[j]) == 0
                    && sign(self.y[i] - self.y[j]) == 0
                    && sign(r[i] - r[j]) == 0
                {
                    r[i] = 0.0;
                    break;
                }
            }
        }

        for i in 0..self.n {
            for j in 0..self.n {
                if i == j {
                    continue;
                }

                if sign(r[j] - r[i]) >= 0
                    && sign(dist(self.x[i], self.y[i], self.x[j], self.y[j]) - (r[j] - r[i])) <= 0
                {
                    is_covered[i] = true;
                    break;
                }
            }
        }

        for i in 0..self.n {
            if sign(r[i]) == 0 || is_covered[i] {
                continue;
            }

            self.segments.clear();

            for j in 0..self.n {
                if i == j {
                    continue;
                }

                let d = dist(self.x[i], self.y[i], self.x[j], self.y[j]);

                if sign(d - (r[i] + r[j])) >= 0 {
                    continue;
                }

                if sign(d - (r[j] - r[i]).abs()) <= 0 {
                    continue;
                }

                let alpha = (self.y[j] - self.y[i]).atan2(self.x[j] - self.x[i]);
                let beta = angle(r[i], d, r[j]);
                let temp = (alpha - beta, alpha + beta);

                if sign(temp.0) <= 0 && sign(temp.1) <= 0 {
                    self.segments.push((
                        2.0 * std::f64::consts::PI + temp.0,
                        2.0 * std::f64::consts::PI + temp.1,
                    ));
                } else if sign(temp.0) < 0 {
                    self.segments.push((
                        2.0 * std::f64::consts::PI + temp.0,
                        2.0 * std::f64::consts::PI,
                    ));
                    self.segments.push((0.0, temp.1));
                } else {
                    self.segments.push(temp);
                }
            }

            self.segments.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());

            let mut right = 0.0;

            for j in 0..self.segments.len() {
                let (a, b) = self.segments[j];

                if sign(right - a) >= 0 {
                    right = right.max(b);
                } else {
                    self.area(i, right, a);
                    right = b;
                }
            }

            if sign(right) == 0 {
                self.arc += r[i] * r[i] * std::f64::consts::PI;
            } else {
                self.area(i, right, 2.0 * std::f64::consts::PI);
            }
        }

        self.polygon / 2.0 + self.arc
    }
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let (n, a) = (scan.token::<usize>(), scan.token::<f64>());
    let mut circles = vec![(0.0, 0.0, 0.0); n];

    for i in 0..n {
        circles[i] = (
            scan.token::<f64>(),
            scan.token::<f64>(),
            scan.token::<f64>(),
        );
    }

    let mut left = 0.0;
    let mut right = 1_000_000_000.0;

    while right - left > 1e-9 {
        let mid = (left + right) / 2.0;
        let mut circle_union = CircleUnion::new(n);

        for i in 0..n {
            circle_union.add(circles[i].0, circles[i].1, (mid - circles[i].2).max(0.0));
        }

        let area_total = circle_union.union(n);

        if area_total >= a {
            right = mid - 1e-9;
        } else {
            left = mid + 1e-9;
        }
    }

    writeln!(out, "{:.9}", left).unwrap();
}
