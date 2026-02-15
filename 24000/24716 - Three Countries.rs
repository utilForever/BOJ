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
struct Point {
    x: f64,
    y: f64,
}

impl Point {
    fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }

    fn dist(&self, other: &Point) -> f64 {
        ((self.x - other.x).powi(2) + (self.y - other.y).powi(2)).sqrt()
    }

    fn cross(&self, other: &Point) -> f64 {
        self.x * other.y - self.y * other.x
    }
}

#[derive(Default, Debug, Clone, Copy)]
struct Circle {
    center: Point,
    radius: f64,
}

impl Circle {
    fn new(center: Point, radius: f64) -> Self {
        Self { center, radius }
    }
}

#[derive(Clone, Copy)]
struct Segment {
    idx: usize,
    angle_start: f64,
    angle_end: f64,
}

impl Segment {
    fn new(idx: usize, angle_start: f64, angle_end: f64) -> Self {
        Self {
            idx,
            angle_start,
            angle_end,
        }
    }
}

const EPS: f64 = 1e-9;

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let t = scan.token::<i64>();

    for _ in 0..t {
        let mut circles = vec![Circle::default(); 3];

        for i in 0..3 {
            let x = scan.token::<f64>();
            let y = scan.token::<f64>();
            let r = scan.token::<f64>();

            circles[i] = Circle::new(Point::new(x, y), r);
        }

        let mut angles = Vec::with_capacity(8);
        angles.push(0.0);

        for i in 0..3 {
            for j in i + 1..3 {
                let diff = circles[i].center.dist(&circles[j].center);
                let dx = circles[j].center.x - circles[i].center.x;
                let dy = circles[j].center.y - circles[i].center.y;
                let dr = circles[i].radius - circles[j].radius;

                if diff < EPS {
                    continue;
                }

                let angle = ((dr / diff).clamp(-1.0, 1.0)).acos();
                let phi = dy.atan2(dx);
                let theta1 = (phi + angle).rem_euclid(std::f64::consts::TAU);
                let theta2 = (phi - angle).rem_euclid(std::f64::consts::TAU);

                angles.push(theta1);
                angles.push(theta2);
            }
        }

        angles.sort_unstable_by(|a, b| a.partial_cmp(b).unwrap());
        angles.dedup_by(|a, b| (*a - *b).abs() < EPS);

        let mut segments: Vec<Segment> = Vec::with_capacity(angles.len());

        for i in 0..angles.len() {
            let a = angles[i];
            let b = if i + 1 < angles.len() {
                angles[i + 1]
            } else {
                std::f64::consts::TAU
            };

            if b - a < EPS {
                continue;
            }

            let mid = (a + b) / 2.0;
            let cos = mid.cos();
            let sin = mid.sin();

            let mut idx = 0;
            let mut best = f64::MIN;

            for j in 0..3 {
                let val = circles[j].center.x * cos + circles[j].center.y * sin + circles[j].radius;

                if val > best {
                    best = val;
                    idx = j;
                }
            }

            if let Some(last) = segments.last() {
                if last.idx == idx {
                    segments.last_mut().unwrap().angle_end = b;
                    continue;
                }
            }

            segments.push(Segment::new(idx, a, b));
        }

        let mut area = 0.0;

        for segment in segments.iter() {
            let a_sin = segment.angle_start.sin();
            let a_cos = segment.angle_start.cos();
            let b_sin = segment.angle_end.sin();
            let b_cos = segment.angle_end.cos();
            let circle = circles[segment.idx];

            area += (circle.radius
                * (circle.center.x * (b_sin - a_sin) - circle.center.y * (b_cos - a_cos))
                + circle.radius.powi(2) * (segment.angle_end - segment.angle_start))
                / 2.0;
        }

        for i in 0..segments.len() {
            let seg1 = segments[i];
            let seg2 = segments[(i + 1) % segments.len()];

            if seg1.idx == seg2.idx {
                continue;
            }

            let theta = (seg1.angle_end).rem_euclid(std::f64::consts::TAU);
            let cos = theta.cos();
            let sin = theta.sin();

            let circle1 = circles[seg1.idx];
            let circle2 = circles[seg2.idx];

            let p = Point::new(
                cos * circle1.radius + circle1.center.x,
                sin * circle1.radius + circle1.center.y,
            );
            let q = Point::new(
                cos * circle2.radius + circle2.center.x,
                sin * circle2.radius + circle2.center.y,
            );

            area += p.cross(&q) / 2.0;
        }

        writeln!(out, "{:.12}", area.abs()).unwrap();
    }
}
