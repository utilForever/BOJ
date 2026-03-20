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

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let n = scan.token::<usize>();
    let mut points = vec![(0, 0, 0); n];

    for i in 0..n {
        points[i] = (scan.token::<i64>(), scan.token::<i64>(), i);
    }

    points.sort_unstable_by(|a, b| a.0.cmp(&b.0));

    let mut slope_max = (points[1].0 - points[0].0, (points[1].1 - points[0].1).abs());

    for i in 1..n - 1 {
        let (dx, dy) = (
            points[i + 1].0 - points[i].0,
            (points[i + 1].1 - points[i].1).abs(),
        );

        if slope_max.0 * dy > slope_max.1 * dx {
            slope_max = (dx, dy);
        }
    }

    let mut idx = 0;
    let mut ret = (usize::MAX, usize::MAX);

    while idx < n - 1 {
        let (dx, dy) = (
            points[idx + 1].0 - points[idx].0,
            (points[idx + 1].1 - points[idx].1).abs(),
        );

        if slope_max.0 * dy != slope_max.1 * dx {
            idx += 1;
            continue;
        }

        let sign = if points[idx + 1].1 > points[idx].1 {
            1
        } else {
            -1
        };
        let left = idx;
        let mut right = idx + 1;

        while right < n - 1 {
            let (dx2, dy2) = (
                points[right + 1].0 - points[right].0,
                (points[right + 1].1 - points[right].1).abs(),
            );

            if slope_max.0 * dy2 != slope_max.1 * dx2 {
                break;
            }

            let sign2 = if points[right + 1].1 > points[right].1 {
                1
            } else {
                -1
            };

            if sign != sign2 {
                break;
            }

            right += 1;
        }

        let mut candidate = (usize::MAX, usize::MAX);

        for i in left..=right {
            if points[i].2 < candidate.0 {
                candidate.1 = candidate.0;
                candidate.0 = points[i].2;
            } else if points[i].2 < candidate.1 {
                candidate.1 = points[i].2;
            }
        }

        ret = ret.min(candidate);
        idx = right;
    }

    writeln!(out, "{} {}", ret.0 + 1, ret.1 + 1).unwrap();
}
