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

    let (n, r) = (scan.token::<i64>(), scan.token::<f64>());
    let mut ret = 0;

    for _ in 0..n {
        let mut points = [(0, 0); 4];

        for i in 0..4 {
            points[i] = (scan.token::<i64>(), scan.token::<i64>());
        }

        let (x1, y1, x2, y2, x3, y3, x4, y4) = (
            points[0].0 as f64,
            points[0].1 as f64,
            points[1].0 as f64,
            points[1].1 as f64,
            points[2].0 as f64,
            points[2].1 as f64,
            points[3].0 as f64,
            points[3].1 as f64,
        );
        let (center_x, center_y) = ((x1 + x2 + x3 + x4) / 4.0, (y1 + y2 + y3 + y4) / 4.0);
        let dist_center = (center_x * center_x + center_y * center_y).sqrt();
        let mut radius_rect = 0.0;

        for &(x, y) in vec![(x1, y1), (x2, y2), (x3, y3), (x4, y4)].iter() {
            let (dx, dy) = (x - center_x, y - center_y);
            let dist = (dx * dx + dy * dy).sqrt();

            if dist > radius_rect {
                radius_rect = dist;
            }
        }

        if dist_center - radius_rect <= r {
            ret += 1;
        }
    }

    writeln!(out, "{ret}").unwrap();
}
