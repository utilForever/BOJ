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

#[derive(Clone, Copy)]
struct Point {
    x: i64,
    y: i64,
}

impl Point {
    fn new(x: i64, y: i64) -> Self {
        Self { x, y }
    }
}

#[derive(Clone, Copy)]
struct BBox {
    min: Point,
    max: Point,
}

impl BBox {
    fn new(min: Point, max: Point) -> Self {
        Self { min, max }
    }

    fn min_dist2(&self, x: i64, y: i64) -> i64 {
        let dx = if x < self.min.x {
            self.min.x - x
        } else if x > self.max.x {
            x - self.max.x
        } else {
            0
        };
        let dy = if y < self.min.y {
            self.min.y - y
        } else if y > self.max.y {
            y - self.max.y
        } else {
            0
        };

        dx * dx + dy * dy
    }

    fn max_dist2(&self, x: i64, y: i64) -> i64 {
        let dx = (self.min.x - x).abs().max((self.max.x - x).abs());
        let dy = (self.min.y - y).abs().max((self.max.y - y).abs());

        dx * dx + dy * dy
    }
}

#[derive(Clone)]
enum Dimension {
    X,
    Y,
}

pub struct KDTree {
    points: Vec<Point>,
    bbox: Vec<BBox>,
    threshold: usize,
}

impl KDTree {
    fn new(points: Vec<Point>, threshold: usize) -> Self {
        let n = points.len();
        let mut tree = KDTree {
            points,
            bbox: vec![BBox::new(Point::new(0, 0), Point::new(0, 0)); n],
            threshold,
        };

        tree.construct(Dimension::X, 0, n);
        tree.fill_bbox(0, n);

        tree
    }

    fn construct(&mut self, dim: Dimension, left: usize, right: usize) {
        if right - left <= self.threshold {
            return;
        }

        let mid = (left + right) / 2;
        self.points[left..right].select_nth_unstable_by(mid - left, |a, b| match dim {
            Dimension::X => a.x.cmp(&b.x),
            Dimension::Y => a.y.cmp(&b.y),
        });

        let next = match dim {
            Dimension::X => Dimension::Y,
            Dimension::Y => Dimension::X,
        };

        self.construct(next.clone(), left, mid);
        self.construct(next, mid + 1, right);
    }

    fn fill_bbox(&mut self, left: usize, right: usize) -> BBox {
        let mid = (left + right) / 2;

        if right - left <= self.threshold {
            let Point {
                x: mut x_min,
                y: mut y_min,
            } = self.points[left];
            let (mut x_max, mut y_max) = (x_min, y_min);

            for &Point { x, y } in &self.points[left + 1..right] {
                x_min = x_min.min(x);
                x_max = x_max.max(x);
                y_min = y_min.min(y);
                y_max = y_max.max(y);
            }

            self.bbox[mid] = BBox::new(Point::new(x_min, y_min), Point::new(x_max, y_max));
            return self.bbox[mid];
        }

        let bbox_left = self.fill_bbox(left, mid);
        let bbox_right = self.fill_bbox(mid + 1, right);
        let point_mid = self.points[mid];

        let merged = BBox::new(
            Point::new(
                bbox_left.min.x.min(bbox_right.min.x).min(point_mid.x),
                bbox_left.min.y.min(bbox_right.min.y).min(point_mid.y),
            ),
            Point::new(
                bbox_left.max.x.max(bbox_right.max.x).max(point_mid.x),
                bbox_left.max.y.max(bbox_right.max.y).max(point_mid.y),
            ),
        );

        self.bbox[mid] = merged;
        merged
    }

    pub fn count_in_circle(&self, x: i64, y: i64, r: i64) -> usize {
        self.count_in_circle_internal(x, y, r * r, 0, self.points.len())
    }

    fn count_in_circle_internal(
        &self,
        x: i64,
        y: i64,
        r2: i64,
        left: usize,
        right: usize,
    ) -> usize {
        let mid = (left + right) / 2;
        let bbox = &self.bbox[mid];

        if bbox.min_dist2(x, y) > r2 {
            return 0;
        }

        if bbox.max_dist2(x, y) <= r2 {
            return right - left;
        }

        if right - left <= self.threshold {
            let mut cnt = 0;

            for i in left..right {
                let dx = self.points[i].x - x;
                let dy = self.points[i].y - y;

                if dx * dx + dy * dy <= r2 {
                    cnt += 1;
                }
            }

            return cnt;
        }

        let mut ret = 0;

        let dx = self.points[mid].x - x;
        let dy = self.points[mid].y - y;

        if dx * dx + dy * dy <= r2 {
            ret += 1;
        }

        ret += self.count_in_circle_internal(x, y, r2, left, mid);
        ret += self.count_in_circle_internal(x, y, r2, mid + 1, right);

        ret
    }
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let n = scan.token::<usize>();
    let mut points = Vec::with_capacity(n);

    for _ in 0..n {
        points.push(Point::new(scan.token::<i64>(), scan.token::<i64>()));
    }

    let m = scan.token::<i64>();
    let tree = KDTree::new(points, 64);

    for _ in 0..m {
        let (x, y, r) = (
            scan.token::<i64>(),
            scan.token::<i64>(),
            scan.token::<i64>(),
        );
        writeln!(out, "{}", tree.count_in_circle(x, y, r)).unwrap();
    }
}
