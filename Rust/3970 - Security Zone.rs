use io::Write;
use std::{cmp::Ordering, io, ops::Sub, str};
use Ordering::{Equal, Greater, Less};

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

#[derive(Clone, Debug)]
struct Disc {
    center: (f64, f64),
    radius: f64,
}

impl Disc {
    pub fn new(center: (f64, f64), radius: f64) -> Self {
        Self { center, radius }
    }

    pub fn exist_common_support_line(&self, other: &Disc) -> bool {
        let (x1, y1, r1) = (self.center.0, self.center.1, self.radius);
        let (x2, y2, r2) = (other.center.0, other.center.1, other.radius);
        let square = (x1 - x2).powi(2) + (y1 - y2).powi(2);

        square > (r1 - r2).powi(2)
    }

    pub fn get_leftmost_point(&self) -> (f64, f64) {
        (self.center.0 - self.radius, self.center.1)
    }

    pub fn compute_common_support_line_to(&self, other: &Disc) -> Edge {
        let (x1, y1, r1) = (self.center.0, self.center.1, self.radius);
        let (x2, y2, r2) = (other.center.0, other.center.1, other.radius);

        let square = (x1 - x2).powi(2) + (y1 - y2).powi(2);
        let d = square.sqrt();
        let vx = (x2 - x1) / d;
        let vy = (y2 - y1) / d;

        let c = (r1 - r2) / d;
        let h = (1.0 - c.powi(2)).sqrt();

        let nx = vx * c - vy * h;
        let ny = vx * h + vy * c;

        Edge {
            p1: Point {
                x: x1 + r1 * nx,
                y: y1 + r1 * ny,
            },
            p2: Point {
                x: x2 + r2 * nx,
                y: y2 + r2 * ny,
            },
        }
    }

    pub fn compute_parallel_support_line(&self, line: &Edge) -> Edge {
        let line_unit = line.normalize().unit();
        let p = Point {
            x: self.center.0 - (line_unit.p2.x - line_unit.p1.x),
            y: self.center.1 - (line_unit.p2.y - line_unit.p1.y),
        };
        let od = Edge {
            p1: Point { x: p.x, y: p.y },
            p2: Point {
                x: self.center.0,
                y: self.center.1,
            },
        }
        .normalize();

        Edge {
            p1: od.p2,
            p2: od.p1,
        }
    }
}

impl PartialEq for Disc {
    fn eq(&self, other: &Self) -> bool {
        self.center == other.center && self.radius == other.radius
    }
}

impl PartialOrd for Disc {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.radius < other.radius {
            Some(Less)
        } else if self.radius > other.radius {
            Some(Greater)
        } else {
            Some(Equal)
        }
    }
}

#[derive(Copy, Clone, Debug, Default)]
struct Point {
    x: f64,
    y: f64,
}

impl Point {
    fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }
}

impl Sub for Point {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

fn is_ccw(p1: &Point, p2: &Point, p3: &Point) -> bool {
    let area = (p2.x - p1.x) * (p3.y - p1.y) - (p2.y - p1.y) * (p3.x - p1.x);
    area > 0.0
}

fn convex_hull(mut points: Vec<Point>) -> Vec<Point> {
    let n = points.len();

    if n <= 1 {
        return points;
    }

    points.sort_by(|a, b| {
        if a.y == b.y {
            a.x.partial_cmp(&b.x).unwrap()
        } else {
            a.y.partial_cmp(&b.y).unwrap()
        }
    });

    let mut hull = Vec::with_capacity(n);

    for i in 0..n {
        while hull.len() >= 2 {
            let m = hull.len() - 1;
            if is_ccw(&hull[m - 1], &hull[m], &points[i]) {
                break;
            }
            hull.pop();
        }
        hull.push(points[i]);
    }

    let lower_hull_len = hull.len();

    for i in (0..(n - 1)).rev() {
        while hull.len() >= lower_hull_len + 1 {
            let m = hull.len() - 1;
            if is_ccw(&hull[m - 1], &hull[m], &points[i]) {
                break;
            }
            hull.pop();
        }
        hull.push(points[i]);
    }

    hull.pop();

    hull
}

#[derive(Clone, Debug, Default)]
struct Edge {
    p1: Point,
    p2: Point,
}

fn radians_to_degrees(radians: f64) -> f64 {
    radians * 180.0 / std::f64::consts::PI
}

impl Edge {
    fn normalize(&self) -> Self {
        let (x1, y1) = (self.p1.x, self.p1.y);
        let (x2, y2) = (self.p2.x, self.p2.y);
        let dx = x2 - x1;
        let dy = y2 - y1;
        let d = (dx * dx + dy * dy).sqrt();

        Self {
            p1: Point {
                x: x1 / d,
                y: y1 / d,
            },
            p2: Point {
                x: x2 / d,
                y: y2 / d,
            },
        }
    }

    fn unit(&self) -> Self {
        let (x1, y1) = (self.p1.x, self.p1.y);
        let (x2, y2) = (self.p2.x, self.p2.y);
        let dx = x2 - x1;
        let dy = y2 - y1;
        let d = (dx * dx + dy * dy).sqrt();

        Self {
            p1: Point { x: 0.0, y: 0.0 },
            p2: Point {
                x: dx / d,
                y: dy / d,
            },
        }
    }

    fn angle(&self) -> f64 {
        let dx = self.p2.x - self.p1.x;
        let dy = self.p2.y - self.p1.y;
        let theta = radians_to_degrees(-dy.atan2(dx));
        let theta_norm = if theta < 0.0 { theta + 360.0 } else { theta };

        if (theta_norm - 360.0).abs() < 1e-6 {
            0.0
        } else {
            theta_norm
        }
    }

    fn angle_to(&self, other: &Self) -> f64 {
        let a1 = self.angle();
        let a2 = other.angle();
        let delta = a2 - a1;
        let delta_norm = if delta < 0.0 { delta + 360.0 } else { delta };

        if (delta_norm - 360.0).abs() < 1e-6 {
            0.0
        } else {
            delta_norm
        }
    }
}

#[derive(Debug)]
struct ConvexHull {
    discs: Vec<Disc>,
    edges: Vec<Edge>,
}

impl ConvexHull {
    fn length(&self) -> f64 {
        let mut points = Vec::new();

        for disc in &self.discs {
            let mut angle = 0.0_f64;

            for _ in 0..=5000 {
                points.push(Point::new(
                    disc.center.0 + disc.radius * angle.cos(),
                    disc.center.1 + disc.radius * angle.sin(),
                ));

                angle += std::f64::consts::PI / 2500.0;
            }
        }

        let hull = convex_hull(points);
        let mut ret = 0.0;

        for i in 0..hull.len() {
            let j = (i + 1) % hull.len();
            ret += (hull[i].x - hull[j].x).hypot(hull[i].y - hull[j].y);
        }

        ret
    }
}

#[derive(Default, Debug)]
struct ConvexHullAlgorithm {
    discs: Vec<Disc>,
}

impl ConvexHullAlgorithm {
    pub fn add_disc(&mut self, disc: Disc) {
        self.discs.push(disc);
    }

    pub fn find(&self, start: usize, end: usize) -> ConvexHull {
        let len = end - start + 1;

        if len == 1 {
            return ConvexHull {
                discs: self.discs.clone(),
                edges: Vec::new(),
            };
        }

        if len == 2 {
            return self.merge_two_discs(&self.discs[0], &self.discs[1]);
        }

        let mid = (start + end) / 2;
        let p = self.find(start, mid);
        let q = self.find(mid + 1, end);

        if p.discs.len() == 1 && q.discs.len() == 1 {
            return self.merge_two_discs(&p.discs[0], &q.discs[0]);
        }

        self.merge(&p, &q)
    }
}

impl ConvexHullAlgorithm {
    fn merge_two_discs(&self, disc1: &Disc, disc2: &Disc) -> ConvexHull {
        let x = disc1.clone();
        let y = disc2.clone();

        if !x.exist_common_support_line(&y) {
            if x.partial_cmp(&y) == Some(Less) {
                return ConvexHull {
                    discs: vec![x],
                    edges: Vec::new(),
                };
            } else {
                return ConvexHull {
                    discs: vec![y],
                    edges: Vec::new(),
                };
            }
        }

        let p1 = x.get_leftmost_point().0;
        let p2 = y.get_leftmost_point().0;
        let e1 = x.compute_common_support_line_to(&y);
        let e2 = y.compute_common_support_line_to(&x);

        if p1 < p2 {
            ConvexHull {
                discs: vec![x.clone(), y, x],
                edges: vec![e1, e2],
            }
        } else if p1 > p2 {
            ConvexHull {
                discs: vec![y.clone(), x, y],
                edges: vec![e2, e1],
            }
        } else {
            if x.center.1 < y.center.1 {
                ConvexHull {
                    discs: vec![x.clone(), y, x],
                    edges: vec![e1, e2],
                }
            } else {
                ConvexHull {
                    discs: vec![y.clone(), x, y],
                    edges: vec![e2, e1],
                }
            }
        }
    }

    fn triangle_orientation(&self, p: &Point, q: &Point, r: &Point) -> f64 {
        (q.x - p.x) * (r.y - p.y) - (q.y - p.y) * (r.x - p.x)
    }

    fn dom(&self, line_p: &Edge, line_q: &Edge) -> bool {
        let to = self.triangle_orientation(&line_p.p1, &line_q.p1, &line_p.p2);

        if to < 0.0 {
            false
        } else if to > 0.0 {
            true
        } else {
            let direction = line_p.p2.y - line_p.p1.y;

            if direction > 0.0 {
                if line_p.p1.y > line_q.p2.y {
                    true
                } else {
                    false
                }
            } else if direction < 0.0 {
                if line_p.p1.y > line_q.p2.y {
                    false
                } else {
                    true
                }
            } else {
                let direction_x = line_p.p2.x - line_p.p1.x;

                if direction_x > 0.0 {
                    if line_p.p1.x > line_q.p2.x {
                        true
                    } else {
                        false
                    }
                } else {
                    if line_p.p1.x > line_q.p2.x {
                        false
                    } else {
                        true
                    }
                }
            }
        }
    }

    fn add(&self, list: &mut Vec<Disc>, disc: &Disc) -> bool {
        if list.is_empty() || list.last().unwrap() != disc {
            list.push(disc.clone());
            true
        } else {
            false
        }
    }

    fn succ(&self, list: &Vec<Disc>, idx: usize) -> usize {
        if idx + 1 < list.len() {
            idx + 1
        } else if list.len() > 1 {
            1
        } else {
            0
        }
    }

    fn merge(&self, lch: &ConvexHull, rch: &ConvexHull) -> ConvexHull {
        let mut list = Vec::new();
        let mut edges = Vec::new();
        let end_p = lch.discs.len();
        let end_q = rch.discs.len();
        let mut disc_idx_p = 0;
        let mut disc_idx_q = 0;
        let mut i = 0;
        let mut j = 0;

        let mut line_star = Edge {
            p1: Point { x: 0.0, y: 0.0 },
            p2: Point { x: 0.0, y: 100.0 },
        };
        let mut line_p = lch.discs[disc_idx_p].compute_parallel_support_line(&line_star);
        let mut line_q = rch.discs[disc_idx_q].compute_parallel_support_line(&line_star);

        while i < end_p || j < end_q {
            if self.dom(&line_p, &line_q) {
                if self.add(&mut list, &lch.discs[disc_idx_p]) && list.len() != 1 {
                    let s = list[list.len() - 2].clone();
                    let e = s.compute_common_support_line_to(&lch.discs[disc_idx_p]);
                    edges.push(e);
                }

                self.advance(
                    &mut line_star,
                    &mut i,
                    &mut disc_idx_p,
                    &mut j,
                    &mut disc_idx_q,
                    &mut list,
                    &mut edges,
                    &lch.discs,
                    &rch.discs,
                );
            } else {
                if self.add(&mut list, &rch.discs[disc_idx_q]) && list.len() != 1 {
                    let s = list[list.len() - 2].clone();
                    let e = s.compute_common_support_line_to(&rch.discs[disc_idx_q]);
                    edges.push(e);
                }

                self.advance(
                    &mut line_star,
                    &mut j,
                    &mut disc_idx_q,
                    &mut i,
                    &mut disc_idx_p,
                    &mut list,
                    &mut edges,
                    &rch.discs,
                    &lch.discs,
                );
            }

            line_p = lch.discs[disc_idx_p].compute_parallel_support_line(&line_star);
            line_q = rch.discs[disc_idx_q].compute_parallel_support_line(&line_star);
        }

        ConvexHull { discs: list, edges }
    }

    fn advance(
        &self,
        line_star: &mut Edge,
        idx_x: &mut usize,
        idx_disc_x: &mut usize,
        idx_y: &mut usize,
        idx_disc_y: &mut usize,
        list: &mut Vec<Disc>,
        edges: &mut Vec<Edge>,
        list_x: &Vec<Disc>,
        list_y: &Vec<Disc>,
    ) {
        let mut line_xy = Edge::default();
        let mut line_yx = Edge::default();
        let mut line_x_succ = Edge::default();
        let mut line_y_succ = Edge::default();

        let mut a1 = 361.0;
        let mut a2 = 361.0;
        let mut a3 = 361.0;
        let mut a4 = 361.0;

        if list_x[*idx_disc_x].exist_common_support_line(&list_y[*idx_disc_y]) {
            line_xy = list_x[*idx_disc_x].compute_common_support_line_to(&list_y[*idx_disc_y]);
            line_yx = list_y[*idx_disc_y].compute_common_support_line_to(&list_x[*idx_disc_x]);
            a1 = line_star.angle_to(&line_xy);
            a4 = line_star.angle_to(&line_yx);
        }

        let x_succ = list_x[self.succ(list_x, *idx_x)].clone();
        let y_succ = list_y[self.succ(list_y, *idx_y)].clone();

        if list_x.len() == 1 && *idx_x == 0 {
            *idx_x += 1;
        } else if *idx_x < list_x.len() {
            line_x_succ = list_x[*idx_disc_x].compute_common_support_line_to(&x_succ);
            a2 = line_star.angle_to(&line_x_succ);
        }

        if list_y.len() == 1 && *idx_y == 0 {
            *idx_y += 1;
        } else if *idx_y < list_y.len() {
            line_y_succ = list_y[*idx_disc_y].compute_common_support_line_to(&y_succ);
            a3 = line_star.angle_to(&line_y_succ);
        }

        if a1 == a1.min(a2.min(a3)) && a1 != 361.0 {
            let disc_x = list_x[*idx_disc_x].clone();
            let disc_y = list_y[*idx_disc_y].clone();

            if self.add(list, &disc_y) {
                edges.push(line_xy);
            }

            if a4 == a4.min(a2.min(a3)) && self.add(list, &disc_x) {
                edges.push(line_yx);
            }
        }

        if a2 < a3 {
            *line_star = line_x_succ.clone();

            if *idx_x < list_x.len() {
                *idx_x += 1;
                *idx_disc_x = *idx_x % list_x.len();
            }
        } else if a2 > a3 || (a2 == a3 && a2 != 361.0) {
            *line_star = line_y_succ.clone();

            if *idx_y < list_y.len() {
                *idx_y += 1;
                *idx_disc_y = *idx_y % list_y.len();
            }
        }
    }
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let c = scan.token::<i64>();

    for _ in 0..c {
        let n = scan.token::<usize>();
        let mut convex_hull_algorithm = ConvexHullAlgorithm::default();

        for _ in 0..n {
            let (x, y, r) = (
                scan.token::<f64>(),
                scan.token::<f64>(),
                scan.token::<f64>() + 10.0,
            );
            convex_hull_algorithm.add_disc(Disc::new((x, y), r));
        }

        writeln!(out, "{:.7}", convex_hull_algorithm.find(1, n).length()).unwrap();
    }
}
