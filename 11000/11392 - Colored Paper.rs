use io::Write;
use std::{
    io,
    ops::{Add, Div, Mul, Sub},
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

#[derive(Debug, Copy, Clone)]
struct Point {
    x: f64,
    y: f64,
}

impl Point {
    #[inline(always)]
    pub fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }

    #[inline(always)]
    pub fn dot(&self, other: &Point) -> f64 {
        self.x * other.x + self.y * other.y
    }

    #[inline(always)]
    pub fn cross(&self, other: &Point) -> f64 {
        self.x * other.y - self.y * other.x
    }

    #[inline(always)]
    pub fn norm2(&self) -> f64 {
        self.dot(self)
    }

    #[inline(always)]
    pub fn norm(&self) -> f64 {
        self.norm2().sqrt()
    }
}

impl Add for Point {
    type Output = Point;
    fn add(self, other: Point) -> Point {
        Point::new(self.x + other.x, self.y + other.y)
    }
}

impl Sub for Point {
    type Output = Point;
    fn sub(self, other: Point) -> Point {
        Point::new(self.x - other.x, self.y - other.y)
    }
}

impl Mul<f64> for Point {
    type Output = Point;

    fn mul(self, rhs: f64) -> Self::Output {
        Point::new(self.x * rhs, self.y * rhs)
    }
}

impl Div<f64> for Point {
    type Output = Point;

    fn div(self, rhs: f64) -> Self::Output {
        Point::new(self.x / rhs, self.y / rhs)
    }
}

#[derive(Clone, Copy, PartialEq)]
enum PointLocation {
    Outside,
    OnBoundary,
    Inside,
}

struct Circle {
    center: Point,
    radius: f64,
    angles: Vec<f64>,
}

impl Circle {
    fn new(center: Point, radius: f64, angles: Vec<f64>) -> Self {
        Self {
            center,
            radius,
            angles,
        }
    }
}

struct Triangle {
    vertices: [Point; 3],
    t_list: [Vec<f64>; 3],
}

impl Triangle {
    fn new(vertices: [Point; 3], t_list: [Vec<f64>; 3]) -> Self {
        Self { vertices, t_list }
    }
}

enum Shape {
    Circle(Circle),
    Triangle(Triangle),
}

const EPS: f64 = 1e-9;

fn intersect_circle_and_circle(c1: &Circle, c2: &Circle) -> Vec<Point> {
    let center_dist = (c2.center - c1.center).norm();

    if center_dist < EPS && (c1.radius - c2.radius).abs() < EPS {
        return Vec::new();
    }

    if center_dist > c1.radius + c2.radius + EPS {
        return Vec::new();
    }

    if center_dist < (c1.radius - c2.radius).abs() - EPS {
        return Vec::new();
    }

    if center_dist < EPS {
        return Vec::new();
    }

    let a = (c1.radius * c1.radius - c2.radius * c2.radius + center_dist * center_dist)
        / (2.0 * center_dist);
    let mut h2 = c1.radius * c1.radius - a * a;

    if h2 < -EPS {
        return Vec::new();
    }

    h2 = h2.max(0.0);

    let unit_ab = Point::new(
        (c2.center.x - c1.center.x) / center_dist,
        (c2.center.y - c1.center.y) / center_dist,
    );
    let center_chord = c1.center + unit_ab * a;
    let unit_perp = Point::new(-unit_ab.y, unit_ab.x);

    if h2.sqrt() < EPS {
        vec![center_chord]
    } else {
        let offset = unit_perp * h2.sqrt();
        vec![center_chord + offset, center_chord - offset]
    }
}

fn intersect_circle_and_segment(c: &Circle, p1: &Point, p2: &Point) -> Vec<(Point, f64)> {
    let segment_dir = *p2 - *p1;
    let from_center = *p1 - c.center;

    let a = segment_dir.dot(&segment_dir);

    if a <= EPS {
        return Vec::new();
    }

    let b = 2.0 * from_center.dot(&segment_dir);
    let c = from_center.dot(&from_center) - c.radius * c.radius;
    let d = b * b - 4.0 * a * c;

    if d < -EPS {
        return Vec::new();
    }

    let mut ret = Vec::new();

    if d.abs() <= EPS {
        let t = -b / (2.0 * a);

        if t >= -EPS && t <= 1.0 + EPS {
            let t = t.clamp(0.0, 1.0);
            ret.push((*p1 + segment_dir * t, t));
        }
    } else {
        let d_sqrt = d.max(0.0).sqrt();
        let t1 = (-b - d_sqrt) / (2.0 * a);
        let t2 = (-b + d_sqrt) / (2.0 * a);

        for t in [t1, t2] {
            if t >= -EPS && t <= 1.0 + EPS {
                let t = t.clamp(0.0, 1.0);
                ret.push((*p1 + segment_dir * t, t));
            }
        }
    }

    if ret.len() == 2 && (ret[0].0 - ret[1].0).norm() < EPS {
        ret.pop();
    }

    ret
}

fn intersect_segment_and_segment(p1: &Point, p2: &Point, p3: &Point, p4: &Point) -> Vec<Point> {
    let r = *p2 - *p1;
    let s = *p4 - *p3;

    let r_cross_s = r.cross(&s);
    let q_sub_p = *p3 - *p1;
    let q_sub_p_cross_r = q_sub_p.cross(&r);

    if r_cross_s.abs() < EPS && q_sub_p_cross_r.abs() < EPS {
        let r_dot_r = r.dot(&r);

        if r_dot_r < EPS {
            return Vec::new();
        }

        let t0 = q_sub_p.dot(&r) / r_dot_r;
        let t1 = (*p4 - *p1).dot(&r) / r_dot_r;

        let (t_min, t_max) = if t0 < t1 { (t0, t1) } else { (t1, t0) };
        let t_start = t_min.clamp(0.0, 1.0);
        let t_end = t_max.clamp(0.0, 1.0);

        if t_end < t_start - EPS {
            return Vec::new();
        }

        if (t_end - t_start).abs() <= EPS {
            return vec![*p1 + r * t_start];
        }

        return vec![*p1 + r * t_start, *p1 + r * t_end];
    }

    if r_cross_s.abs() < EPS {
        return Vec::new();
    }

    let t = q_sub_p.cross(&s) / r_cross_s;
    let u = q_sub_p.cross(&r) / r_cross_s;

    if t >= -EPS && t <= 1.0 + EPS && u >= -EPS && u <= 1.0 + EPS {
        let t = t.clamp(0.0, 1.0);
        vec![*p1 + r * t]
    } else {
        Vec::new()
    }
}

fn segment_parameter(a: Point, b: Point, p: Point) -> f64 {
    let d = b - a;
    let denom = d.dot(&d);

    if denom <= EPS {
        return 0.0;
    }

    (p.sub(a).dot(&d) / denom).clamp(0.0, 1.0)
}

fn normalize_angle(mut a: f64) -> f64 {
    let tau = 2.0 * std::f64::consts::PI;

    a = a % tau;

    if a < 0.0 {
        a += tau;
    }

    a
}

#[derive(Debug)]
struct Piece {
    owner: usize,
    area: f64,
    point_on: Point,
    point_out: Point,
}

fn locate_in_circle(c: &Circle, p: Point) -> PointLocation {
    let d2 = p.sub(c.center).norm2();
    let r2 = c.radius * c.radius;

    if d2 < r2 - EPS {
        PointLocation::Inside
    } else if (d2 - r2).abs() <= EPS {
        PointLocation::OnBoundary
    } else {
        PointLocation::Outside
    }
}

fn locate_in_triangle(t: &Triangle, p: Point) -> PointLocation {
    let mut on_boundary = false;

    for i in 0..3 {
        let a = t.vertices[i];
        let b = t.vertices[(i + 1) % 3];
        let cross = (b - a).cross(&(p - a));

        if cross < -EPS {
            return PointLocation::Outside;
        }

        if cross.abs() <= EPS {
            on_boundary = true;
        }
    }

    if on_boundary {
        PointLocation::OnBoundary
    } else {
        PointLocation::Inside
    }
}

fn locate_in_shape(shape: &Shape, p: Point) -> PointLocation {
    match shape {
        Shape::Circle(c) => locate_in_circle(c, p),
        Shape::Triangle(t) => locate_in_triangle(t, p),
    }
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let n = scan.token::<usize>();
    let mut shapes = Vec::with_capacity(n);

    for _ in 0..n {
        let t = scan.token::<i64>();

        if t == 1 {
            let (x1, y1, x2, y2, x3, y3) = (
                scan.token::<f64>(),
                scan.token::<f64>(),
                scan.token::<f64>(),
                scan.token::<f64>(),
                scan.token::<f64>(),
                scan.token::<f64>(),
            );
            let mut points = [Point::new(x1, y1), Point::new(x2, y2), Point::new(x3, y3)];
            let t_list = [vec![0.0, 1.0], vec![0.0, 1.0], vec![0.0, 1.0]];
            let area2 = (points[1] - points[0]).cross(&(points[2] - points[0]));

            if area2 < 0.0 {
                points.swap(1, 2);
            }

            shapes.push(Shape::Triangle(Triangle::new(points, t_list)));
        } else {
            let (x, y, r) = (
                scan.token::<f64>(),
                scan.token::<f64>(),
                scan.token::<f64>(),
            );
            shapes.push(Shape::Circle(Circle::new(Point::new(x, y), r, Vec::new())));
        }
    }

    for i in 0..n {
        for j in i + 1..n {
            let (left, right) = shapes.split_at_mut(j);

            match (&mut left[i], &mut right[0]) {
                (Shape::Triangle(t1), Shape::Triangle(t2)) => {
                    for e1 in 0..3 {
                        let a1 = t1.vertices[e1];
                        let a2 = t1.vertices[(e1 + 1) % 3];

                        for e2 in 0..3 {
                            let b1 = t2.vertices[e2];
                            let b2 = t2.vertices[(e2 + 1) % 3];
                            let points_intersect =
                                intersect_segment_and_segment(&a1, &a2, &b1, &b2);

                            for point in points_intersect {
                                t1.t_list[e1].push(segment_parameter(a1, a2, point));
                                t2.t_list[e2].push(segment_parameter(b1, b2, point));
                            }
                        }
                    }
                }
                (Shape::Triangle(t), Shape::Circle(c)) | (Shape::Circle(c), Shape::Triangle(t)) => {
                    for e in 0..3 {
                        let a = t.vertices[e];
                        let b = t.vertices[(e + 1) % 3];
                        let points_intersect = intersect_circle_and_segment(c, &a, &b);

                        for (point, t_param) in points_intersect {
                            let angle =
                                normalize_angle((point.y - c.center.y).atan2(point.x - c.center.x));
                            c.angles.push(angle);
                            t.t_list[e].push(t_param);
                        }
                    }
                }
                (Shape::Circle(c1), Shape::Circle(c2)) => {
                    let points_intersect = intersect_circle_and_circle(c1, c2);

                    for point in points_intersect {
                        let angle1 =
                            normalize_angle((point.y - c1.center.y).atan2(point.x - c1.center.x));
                        let angle2 =
                            normalize_angle((point.y - c2.center.y).atan2(point.x - c2.center.x));
                        c1.angles.push(angle1);
                        c2.angles.push(angle2);
                    }
                }
            }
        }
    }

    let mut pieces = Vec::new();

    for (idx, shape) in shapes.iter().enumerate() {
        match shape {
            Shape::Triangle(t) => {
                for e in 0..3 {
                    let a = t.vertices[e];
                    let b = t.vertices[(e + 1) % 3];
                    let mut t_list = t.t_list[e].clone();

                    t_list.sort_unstable_by(|x, y| x.partial_cmp(y).unwrap());
                    t_list.dedup_by(|x, y| (*x - *y).abs() <= EPS);

                    if t_list.len() < 2 {
                        continue;
                    }

                    for w in 0..t_list.len() - 1 {
                        let t1 = t_list[w];
                        let t2 = t_list[w + 1];

                        if t2 - t1 < EPS {
                            continue;
                        }

                        let p = a + (b - a) * t1;
                        let q = a + (b - a) * t2;
                        let point_on = Point::new((p.x + q.x) / 2.0, (p.y + q.y) / 2.0);

                        let d = q - p;
                        let len = d.norm();

                        if len < EPS {
                            continue;
                        }

                        let normal_out = Point::new(d.y / len, -d.x / len);
                        let point_out = point_on + normal_out * EPS;

                        pieces.push(Piece {
                            owner: idx + 1,
                            area: 0.5 * p.cross(&q),
                            point_on,
                            point_out,
                        })
                    }
                }
            }
            Shape::Circle(c) => {
                let mut angles = c.angles.clone();

                for angle in angles.iter_mut() {
                    *angle = normalize_angle(*angle);
                }

                angles.sort_unstable_by(|x, y| x.partial_cmp(y).unwrap());
                angles.dedup_by(|x, y| (*x - *y).abs() <= EPS);

                if angles.len() > 1 {
                    let first = angles[0];
                    let last = angles[angles.len() - 1];

                    if first < EPS && (2.0 * std::f64::consts::PI - last) < EPS {
                        angles.pop();
                    }
                }

                if angles.is_empty() {
                    angles.push(0.0);
                }

                let m = angles.len();

                for i in 0..m {
                    let a = angles[i];
                    let mut b = if i + 1 < m {
                        angles[i + 1]
                    } else {
                        angles[0] + 2.0 * std::f64::consts::PI
                    };

                    if b - a <= EPS {
                        b += 2.0 * std::f64::consts::PI;
                    }

                    if b - a < EPS {
                        continue;
                    }

                    let mid = (a + b) / 2.0;
                    let point_on = Point::new(
                        c.center.x + c.radius * mid.cos(),
                        c.center.y + c.radius * mid.sin(),
                    );
                    let point_out = Point::new(
                        c.center.x + (c.radius + EPS) * mid.cos(),
                        c.center.y + (c.radius + EPS) * mid.sin(),
                    );

                    pieces.push(Piece {
                        owner: idx + 1,
                        area: 0.5
                            * c.radius
                            * (c.center.x * (b.sin() - a.sin()) - c.center.y * (b.cos() - a.cos())
                                + c.radius * (b - a)),
                        point_on,
                        point_out,
                    });
                }
            }
        }
    }

    let mut diff = vec![vec![0.0; n + 3]; n + 3];

    for piece in pieces.iter() {
        let mut lower_cover_max = 0;
        let mut upper_cover_min = n + 1;

        for (idx, shape) in shapes.iter().enumerate() {
            if idx + 1 == piece.owner {
                continue;
            }

            let location_out = locate_in_shape(shape, piece.point_out);

            if location_out == PointLocation::Outside {
                if idx + 1 > piece.owner
                    && locate_in_shape(shape, piece.point_on) == PointLocation::OnBoundary
                {
                    upper_cover_min = upper_cover_min.min(idx + 1);
                }
            } else {
                if idx + 1 < piece.owner {
                    lower_cover_max = lower_cover_max.max(idx + 1);
                } else {
                    upper_cover_min = upper_cover_min.min(idx + 1);
                }
            }
        }

        let l1 = lower_cover_max + 1;
        let l2 = piece.owner;
        let r1 = piece.owner;
        let r2 = upper_cover_min - 1;

        if l1 <= l2 && r1 <= r2 {
            diff[l1][r1] += piece.area;
            diff[l1][r2 + 1] -= piece.area;
            diff[l2 + 1][r1] -= piece.area;
            diff[l2 + 1][r2 + 1] += piece.area;
        }
    }

    for i in 1..=n {
        for j in 1..=n {
            let val = diff[i][j] + diff[i - 1][j] + diff[i][j - 1] - diff[i - 1][j - 1];
            diff[i][j] = val;
        }
    }

    for i in 1..=n {
        for j in 1..=i {
            let mut ret = diff[j][i];

            if i > j {
                ret -= diff[j + 1][i];
            }

            write!(out, "{:.12} ", ret).unwrap();
        }

        writeln!(out).unwrap();
    }
}
