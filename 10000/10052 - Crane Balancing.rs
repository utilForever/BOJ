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

    pub fn all(&mut self) -> String {
        let mut input = String::new();
        self.reader.read_to_string(&mut input).expect("Failed read");
        input
    }

    pub fn line(&mut self) -> String {
        let mut input = String::new();
        self.reader.read_line(&mut input).expect("Failed read");
        input
    }
}

const EPS: f64 = 1e-9;
const INF: f64 = 1e100;

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let n = scan.token::<usize>();
    let mut points = vec![(0.0, 0.0); n];

    for i in 0..n {
        points[i] = (scan.token::<f64>(), scan.token::<f64>());
    }

    let mut sum_area = 0.0;
    let mut sum_center_of_mass = 0.0;

    for i in 0..n {
        let (x1, y1) = points[i];
        let (x2, y2) = points[(i + 1) % n];
        let cross = x1 * y2 - x2 * y1;

        sum_area += cross;
        sum_center_of_mass += (x1 + x2) * cross;
    }

    let area = sum_area.abs() * 0.5;
    let center_of_mass = sum_center_of_mass / (3.0 * sum_area);
    let moment = area * center_of_mass;

    let (mut left, mut right) = (f64::INFINITY, -f64::INFINITY);

    for &(x, y) in points.iter() {
        if y == 0.0 {
            left = left.min(x);
            right = right.max(x);
        }
    }

    let mut low = 0.0f64;
    let mut high = INF;

    'outer: {
        if (points[0].0 - left).abs() < EPS {
            if (left * area - moment) > EPS {
                writeln!(out, "unstable").unwrap();
                return;
            }

            break 'outer;
        }

        let cut = (left * area - moment) / (points[0].0 - left);

        if points[0].0 - left > 0.0 {
            low = low.max(cut);
        } else {
            high = high.min(cut);
        }
    }

    'outer: {
        if (points[0].0 - right).abs() < EPS {
            if right * area - moment < -EPS {
                writeln!(out, "unstable").unwrap();
                return;
            }

            break 'outer;
        }

        let cut = (right * area - moment) / (points[0].0 - right);

        if points[0].0 - right > 0.0 {
            high = high.min(cut);
        } else {
            low = low.max(cut);
        }
    }

    if low - high > EPS {
        writeln!(out, "unstable").unwrap();
        return;
    }

    let low = (low + EPS).floor() as i64;

    if high == INF {
        writeln!(out, "{low} .. inf").unwrap();
    } else {
        let high = (high - EPS).ceil() as i64;
        writeln!(out, "{low} .. {high}").unwrap();
    }
}
