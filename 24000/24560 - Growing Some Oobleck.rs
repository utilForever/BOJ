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

#[derive(Default, Debug, Clone, Copy)]
struct Circle {
    x: f64,
    y: f64,
    r: f64,
    s: f64,
}

impl Circle {
    fn new(x: f64, y: f64, r: f64, s: f64) -> Self {
        Self { x, y, r, s }
    }
}

const EPS: f64 = 1e-9;

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let n = scan.token::<usize>();
    let mut circles = vec![Circle::default(); n];

    for i in 0..n {
        let (x, y, r, s) = (
            scan.token::<f64>(),
            scan.token::<f64>(),
            scan.token::<f64>(),
            scan.token::<f64>(),
        );
        circles[i] = Circle::new(x, y, r, s);
    }

    while circles.len() > 1 {
        let mut min_dt = f64::MAX;
        let mut min_idx1 = 0;
        let mut min_idx2 = 0;

        for i in 0..circles.len() {
            for j in i + 1..circles.len() {
                let dist = ((circles[i].x - circles[j].x).powi(2)
                    + (circles[i].y - circles[j].y).powi(2))
                .sqrt();
                let dt = (dist - (circles[i].r + circles[j].r)) / (circles[i].s + circles[j].s);

                if dt < min_dt {
                    min_dt = dt;
                    min_idx1 = i;
                    min_idx2 = j;
                }
            }
        }

        for circle in circles.iter_mut() {
            circle.r += circle.s * min_dt;
        }

        let a = circles[min_idx1];
        let b = circles[min_idx2];
        let mut c = Circle::new(
            (a.x + b.x) / 2.0,
            (a.y + b.y) / 2.0,
            (a.r * a.r + b.r * b.r).sqrt(),
            a.s.max(b.s),
        );

        let mut pool = Vec::with_capacity(circles.len() - 1);

        for (idx, circle) in circles.iter().enumerate() {
            if idx != min_idx1 && idx != min_idx2 {
                pool.push(*circle);
            }
        }

        circles = pool;

        loop {
            let mut merged = vec![false; circles.len()];
            let mut cnt = 0;

            let mut sum_x = c.x;
            let mut sum_y = c.y;
            let mut sum_r2 = c.r * c.r;
            let mut max_s = c.s;

            for (idx, circle) in circles.iter().enumerate() {
                let dx = circle.x - c.x;
                let dy = circle.y - c.y;
                let dist = (dx * dx + dy * dy).sqrt();

                if dist <= c.r + circle.r + EPS {
                    merged[idx] = true;
                    cnt += 1;

                    sum_x += circle.x;
                    sum_y += circle.y;
                    sum_r2 += circle.r * circle.r;
                    max_s = max_s.max(circle.s);
                }
            }

            if cnt == 0 {
                break;
            }

            let mut pool = Vec::with_capacity(circles.len() - cnt);

            for (idx, circle) in circles.iter().enumerate() {
                if !merged[idx] {
                    pool.push(*circle);
                }
            }

            circles = pool;
            c = Circle::new(
                sum_x / (cnt as f64 + 1.0),
                sum_y / (cnt as f64 + 1.0),
                sum_r2.sqrt(),
                max_s,
            );
        }

        circles.push(c);
    }

    writeln!(out, "{:.12} {:.12}", circles[0].x, circles[0].y).unwrap();
    writeln!(out, "{:.12}", circles[0].r).unwrap();
}
