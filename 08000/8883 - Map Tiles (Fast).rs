use io::Write;
use std::{
    io,
    ops::{Add, Sub},
    str,
};

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

#[derive(Debug, Default, Copy, Clone)]
struct Point {
    x: f64,
    y: f64,
}

impl Add for Point {
    type Output = Self;

    #[inline(always)]
    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Sub for Point {
    type Output = Self;

    #[inline(always)]
    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl Point {
    #[inline(always)]
    fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }

    #[inline(always)]
    fn cross(&self, other: &Self) -> f64 {
        self.x * other.y - self.y * other.x
    }

    #[inline(always)]
    fn cross2(&self, p1: &Self, p2: &Self) -> f64 {
        (p1.x - self.x) * (p2.y - self.y) - (p1.y - self.y) * (p2.x - self.x)
    }
}

mod geometry {
    use crate::Point;

    #[inline]
    fn floor_div_pos(a: i64, b: i64) -> i64 {
        let q = a / b;
        let r = a % b;

        if r < 0 {
            q - 1
        } else {
            q
        }
    }

    pub(crate) fn congruence_near(residue: i64, base: i64, step: i64) -> i64 {
        let k = floor_div_pos(base - residue, step);
        residue + k * step
    }

    pub(crate) fn intersect_lines(p1: Point, p2: Point, p3: Point, p4: Point) -> Point {
        let a = p3.cross2(&p4, &p1);
        let b = -p3.cross2(&p4, &p2);
        let denom = a + b;

        Point::new((p1.x * b + p2.x * a) / denom, (p1.y * b + p2.y * a) / denom)
    }
}

mod polygon {
    use super::Point;

    fn area(polygon: &Vec<Point>) -> f64 {
        let n = polygon.len();
        let mut area = 0.0;

        for i in 0..n {
            let p = &polygon[i];
            let q = &polygon[(i + 1) % n];

            area += p.x * q.y - p.y * q.x;
        }

        area / 2.0
    }

    pub(crate) fn normalize(mut polygon: Vec<Point>) -> Vec<Point> {
        if area(&polygon) < 0.0 {
            polygon.reverse();
        }

        let n = polygon.len();
        let mut idx = 0;

        for i in 1..n {
            if (polygon[i].x, polygon[i].y) < (polygon[idx].x, polygon[idx].y) {
                idx = i;
            }
        }

        let mut ret = Vec::with_capacity(n);
        ret.extend_from_slice(&polygon[idx..]);
        ret.extend_from_slice(&polygon[..idx]);

        ret
    }

    pub(crate) fn bounding_box(polygon: &Vec<Point>) -> (f64, f64, f64, f64) {
        let mut y_min = f64::MAX;
        let mut y_max = f64::MIN;
        let mut x_min = f64::MAX;
        let mut x_max = f64::MIN;

        for &point in polygon.iter() {
            y_min = y_min.min(point.y);
            y_max = y_max.max(point.y);
            x_min = x_min.min(point.x);
            x_max = x_max.max(point.x);
        }

        (y_min, y_max, x_min, x_max)
    }
}

mod offset {
    use std::collections::HashSet;

    use crate::{geometry, Point, EPS};

    const SCALE: f64 = 1_000_000.0;

    fn normalize(mut point: Point, width: f64, height: f64) -> Point {
        point.x = point.x % width;

        if point.x < 0.0 {
            point.x += width;
        }

        if point.x >= width {
            point.x -= width;
        }

        point.y = point.y % height;

        if point.y < 0.0 {
            point.y += height;
        }

        if point.y >= height {
            point.y -= height;
        }

        point
    }

    fn push(
        visited: &mut HashSet<(i64, i64)>,
        ret: &mut Vec<Point>,
        width: f64,
        height: f64,
        offset: Point,
    ) {
        let normalized = normalize(offset, width, height);
        let key = (
            (normalized.x * SCALE).round() as i64,
            (normalized.y * SCALE).round() as i64,
        );

        if visited.insert(key) {
            ret.push(normalized);
        }
    }

    pub(crate) fn generate(points: &Vec<Point>, width: f64, height: f64) -> Vec<Point> {
        let n = points.len();
        let anchor = points[0];

        let mut visited = HashSet::new();
        let mut ret = Vec::new();

        // (vertex, vertex)
        for i in 0..n {
            for j in 0..n {
                push(
                    &mut visited,
                    &mut ret,
                    width,
                    height,
                    Point::new(anchor.x - points[i].x, anchor.y - points[j].y),
                );
                push(
                    &mut visited,
                    &mut ret,
                    width,
                    height,
                    Point::new(anchor.x - points[j].x, anchor.y - points[i].y),
                );
            }
        }

        // (vertex, edge)
        for i in 0..n {
            for j in 0..n {
                let start = points[j];
                let end = points[(j + 1) % n];

                // vertical grid lines
                if start.x != end.x {
                    let (mut x1, mut y1) = (start.x, start.y);
                    let (mut x2, mut y2) = (end.x, end.y);

                    if x1 > x2 {
                        std::mem::swap(&mut x1, &mut x2);
                        std::mem::swap(&mut y1, &mut y2);
                    }

                    let x_start =
                        geometry::congruence_near(points[i].x as i64, x1 as i64, width as i64);
                    let x_end =
                        geometry::congruence_near(points[i].x as i64, x2 as i64, width as i64);
                    let mut x = x_start;

                    while x <= x_end {
                        if x >= x1 as i64 && x <= x2 as i64 {
                            let y = y1 + (x as f64 - x1) * (y2 - y1) / (x2 - x1);

                            push(
                                &mut visited,
                                &mut ret,
                                width,
                                height,
                                Point::new(anchor.x - points[i].x, anchor.y - y),
                            );
                        }

                        x += width as i64;
                    }
                }

                // horizontal grid lines
                if start.y != end.y {
                    let (mut x1, mut y1) = (start.x, start.y);
                    let (mut x2, mut y2) = (end.x, end.y);

                    if y1 > y2 {
                        std::mem::swap(&mut x1, &mut x2);
                        std::mem::swap(&mut y1, &mut y2);
                    }

                    let y_start =
                        geometry::congruence_near(points[i].y as i64, y1 as i64, height as i64);
                    let y_end =
                        geometry::congruence_near(points[i].y as i64, y2 as i64, height as i64);
                    let mut y = y_start;

                    while y <= y_end {
                        if y >= y1 as i64 && y <= y2 as i64 {
                            let x = x1 + (y as f64 - y1) * (x2 - x1) / (y2 - y1);

                            push(
                                &mut visited,
                                &mut ret,
                                width,
                                height,
                                Point::new(anchor.x - x, anchor.y - points[i].y),
                            );
                        }

                        y += height as i64;
                    }
                }
            }
        }

        // (edge, edge)
        for i in 0..n {
            let a = points[i];
            let b = points[(i + 1) % n];
            let ab = b - a;

            for j in 0..n {
                if i == j {
                    continue;
                }

                let c = points[j];
                let d = points[(j + 1) % n];

                if (b - a).cross(&(d - c)).abs() < EPS {
                    continue;
                }

                let corner1 = c;
                let corner2 = Point::new(c.x - ab.x, c.y - ab.y);
                let corner3 = d;
                let corner4 = Point::new(d.x - ab.x, d.y - ab.y);

                let x_min = corner1.x.min(corner2.x).min(corner3.x).min(corner4.x);
                let x_max = corner1.x.max(corner2.x).max(corner3.x).max(corner4.x);
                let y_min = corner1.y.min(corner2.y).min(corner3.y).min(corner4.y);
                let y_max = corner1.y.max(corner2.y).max(corner3.y).max(corner4.y);

                let u = d - c;
                let v = Point::new(-ab.x, -ab.y);
                let denom = u.cross(&v);

                if denom.abs() < EPS {
                    continue;
                }

                let mut grid_x = geometry::congruence_near(a.x as i64, x_min as i64, width as i64);

                while grid_x <= x_max as i64 {
                    let mut grid_y =
                        geometry::congruence_near(a.y as i64, y_min as i64, height as i64);

                    while grid_y <= y_max as i64 {
                        let p = Point::new(grid_x as f64, grid_y as f64);
                        let w = p - c;

                        let cross1 = u.cross(&w);
                        let cross2 = w.cross(&v);
                        let is_inside = if denom > 0.0 {
                            cross1 >= -EPS
                                && cross1 <= denom + EPS
                                && cross2 >= -EPS
                                && cross2 <= denom + EPS
                        } else {
                            cross1 <= EPS
                                && cross1 >= denom - EPS
                                && cross2 <= EPS
                                && cross2 >= denom - EPS
                        };

                        if is_inside {
                            let intersect = geometry::intersect_lines(
                                c,
                                d,
                                Point::new(grid_x as f64, grid_y as f64),
                                Point::new(grid_x as f64 + ab.x, grid_y as f64 + ab.y),
                            );
                            push(
                                &mut visited,
                                &mut ret,
                                width,
                                height,
                                Point::new(anchor.x - intersect.x, anchor.y - intersect.y),
                            );
                        }

                        grid_y += height as i64;
                    }

                    grid_x += width as i64;
                }
            }
        }

        ret
    }
}

fn is_point_inside_polygon(point: Point, polygon: &[Point]) -> bool {
    let n = polygon.len();
    let mut inside = false;

    for i in 0..n {
        let a = polygon[i];
        let b = polygon[(i + 1) % n];

        if (a.y > point.y) != (b.y > point.y) {
            let x_intersect = (b.x - a.x) * (point.y - a.y) / (b.y - a.y) + a.x;

            if x_intersect > point.x {
                inside = !inside;
            }
        }
    }

    inside
}

#[derive(Copy, Clone)]
struct Edge {
    ax: f64,
    ay: f64,
    dx: f64,
    dy: f64,
    y_min: f64,
    y_max: f64,
    x_min: f64,
    x_max: f64,
}

#[inline(always)]
fn build_edges(polygon: &[Point]) -> Vec<Edge> {
    let n = polygon.len();
    let mut edges = Vec::with_capacity(n);

    for i in 0..n {
        let a = polygon[i];
        let b = polygon[(i + 1) % n];

        edges.push(Edge {
            ax: a.x,
            ay: a.y,
            dx: b.x - a.x,
            dy: b.y - a.y,
            y_min: a.y.min(b.y),
            y_max: a.y.max(b.y),
            x_min: a.x.min(b.x),
            x_max: a.x.max(b.x),
        });
    }

    edges
}

fn intersect_segment_open_rectangle(edge: &Edge, y1: f64, y2: f64, x1: f64, x2: f64) -> bool {
    let y_min = y1 + EPS;
    let y_max = y2 - EPS;
    let x_min = x1 + EPS;
    let x_max = x2 - EPS;

    if y_min >= y_max || x_min >= x_max {
        return false;
    }

    if edge.y_max <= y_min || edge.y_min >= y_max || edge.x_max <= x_min || edge.x_min >= x_max {
        return false;
    }

    let mut t0 = 0.0;
    let mut t1 = 1.0;

    let mut clip = |p: f64, q: f64| -> bool {
        if p.abs() < EPS {
            return q >= 0.0;
        }

        let r = q / p;

        if p < 0.0 {
            if r > t1 {
                return false;
            }

            if r > t0 {
                t0 = r;
            }
        } else {
            if r < t0 {
                return false;
            }

            if r < t1 {
                t1 = r;
            }
        }

        true
    };

    if !clip(-edge.dy, edge.ay - y_min) {
        return false;
    }

    if !clip(edge.dy, y_max - edge.ay) {
        return false;
    }

    if !clip(-edge.dx, edge.ax - x_min) {
        return false;
    }

    if !clip(edge.dx, x_max - edge.ax) {
        return false;
    }

    t0 <= t1
}

fn check(polygon: &Vec<Point>, edges: &Vec<Edge>, y1: f64, y2: f64, x1: f64, x2: f64) -> bool {
    let y_min = y1 + EPS;
    let y_max = y2 - EPS;
    let x_min = x1 + EPS;
    let x_max = x2 - EPS;

    for &point in polygon.iter() {
        if point.x > x_min && point.x < x_max && point.y > y_min && point.y < y_max {
            return true;
        }
    }

    let center = Point::new((x1 + x2) / 2.0, (y1 + y2) / 2.0);

    if is_point_inside_polygon(center, polygon) {
        return true;
    }

    for edge in edges.iter() {
        if intersect_segment_open_rectangle(edge, y1, y2, x1, x2) {
            return true;
        }
    }

    false
}

fn calculate(polygon: &Vec<Point>, cnt_min: i64, width: f64, height: f64) -> i64 {
    let (y_min, y_max, x_min, x_max) = polygon::bounding_box(polygon);
    let row_min = (y_min / height).floor() as i64 - 1;
    let row_max = (y_max / height).ceil() as i64 + 1;
    let col_min = (x_min / width).floor() as i64 - 1;
    let col_max = (x_max / width).ceil() as i64 + 1;

    let edges = build_edges(polygon);
    let mut cnt = 0;

    for row in row_min..=row_max {
        let y1 = row as f64 * height;
        let y2 = y1 + height;

        for col in col_min..=col_max {
            let x1 = col as f64 * width;
            let x2 = x1 + width;

            if check(polygon, &edges, y1, y2, x1, x2) {
                cnt += 1;

                if cnt >= cnt_min {
                    return cnt;
                }
            }
        }
    }

    cnt
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let (n, w, h) = (
        scan.token::<usize>(),
        scan.token::<f64>(),
        scan.token::<f64>(),
    );
    let mut points = Vec::with_capacity(n);

    for _ in 0..n {
        let (x, y) = (scan.token::<f64>(), scan.token::<f64>());
        points.push(Point::new(x, y));
    }

    let polygon = polygon::normalize(points);
    let offsets_candidate = offset::generate(&polygon, w, h);

    let mut translated = vec![Point::new(0.0, 0.0); n];
    let mut ret = i64::MAX;

    for offset in offsets_candidate {
        let dx = offset.x - polygon[0].x;
        let dy = offset.y - polygon[0].y;

        for i in 0..n {
            translated[i] = Point::new(polygon[i].x + dx, polygon[i].y + dy);
        }

        let cnt = calculate(&translated, ret, w, h);
        ret = ret.min(cnt);
    }

    writeln!(out, "{ret}").unwrap();
}
