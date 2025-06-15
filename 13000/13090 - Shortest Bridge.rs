use io::Write;
use std::{
    cmp::Ordering,
    collections::BinaryHeap,
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

const EPS: f64 = 1e-9;

#[inline(always)]
fn sign(x: f64) -> i64 {
    if x < -EPS {
        -1
    } else if x > EPS {
        1
    } else {
        0
    }
}

#[derive(Debug, Copy, Clone, PartialOrd)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

impl Point {
    #[inline(always)]
    fn new(x: f64, y: f64) -> Self {
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
    pub fn cross2(&self, p1: &Point, p2: &Point) -> f64 {
        (*p1 - *self).cross(&(*p2 - *self))
    }

    #[inline(always)]
    pub fn dist(&self) -> f64 {
        (self.x * self.x + self.y * self.y).sqrt()
    }

    #[inline(always)]
    pub fn dist2(&self) -> f64 {
        self.x * self.x + self.y * self.y
    }

    #[inline(always)]
    pub fn normalize(&self) -> Point {
        let d = self.dist();

        Point {
            x: self.x / d,
            y: self.y / d,
        }
    }

    #[inline(always)]
    pub fn perp(&self) -> Point {
        Point {
            x: -self.y,
            y: self.x,
        }
    }

    #[inline(always)]
    fn intersect_segment(a: &Point, b: &Point, c: &Point, d: &Point) -> Option<Point> {
        let oa = c.cross2(d, a);
        let ob = c.cross2(d, b);
        let oc = a.cross2(b, c);
        let od = a.cross2(b, d);

        if sign(oa) * sign(ob) < 0 && sign(oc) * sign(od) < 0 {
            Some((*a * ob - *b * oa) / (ob - oa))
        } else {
            None
        }
    }

    #[inline(always)]
    fn is_on_segment(a: &Point, b: &Point, p: &Point) -> bool {
        p.cross2(a, b).abs() < EPS && (*a - *p).dot(&(*b - *p)) < EPS
    }

    #[inline(always)]
    fn project_on_segment(a: &Point, b: &Point, p: &Point) -> Option<Point> {
        let ab = *b - *a;
        let t = ab.dot(&(*p - *a)) / ab.dist2();

        if t < -EPS || t > 1.0 + EPS {
            return None;
        }

        Some(*a + ab * t)
    }
}

impl Add for Point {
    type Output = Point;

    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Sub for Point {
    type Output = Point;

    fn sub(self, other: Point) -> Point {
        Point {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl Mul<f64> for Point {
    type Output = Point;

    fn mul(self, rhs: f64) -> Self::Output {
        Point {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

impl Div<f64> for Point {
    type Output = Point;

    fn div(self, rhs: f64) -> Self::Output {
        Point {
            x: self.x / rhs,
            y: self.y / rhs,
        }
    }
}

impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        sign(self.x - other.x) == 0 && sign(self.y - other.y) == 0
    }
}

#[derive(Clone)]
pub struct Polygon {
    points: Vec<Point>,
}

impl Polygon {
    fn new(points: Vec<Point>) -> Self {
        Self { points }
    }

    fn len(&self) -> usize {
        self.points.len()
    }

    fn is_inside(&self, p: &Point, strict: bool) -> bool {
        let n = self.len();
        let mut cnt = 0;

        for i in 0..n {
            let q = self.points[(i + 1) % n];

            if Point::is_on_segment(&self.points[i], &q, p) {
                return !strict;
            }

            let cond1 = if p.y < self.points[i].y { 1 } else { 0 };
            let cond2 = if p.y < q.y { 1 } else { 0 };

            if (cond1 - cond2) as f64 * p.cross2(&self.points[i], &q) > 0.0 {
                cnt += 1;
            }
        }

        cnt % 2 == 1
    }

    fn is_visible(&self, a: &Point, b: &Point) -> bool {
        let mut ts = vec![0.0f64, 1.0f64];

        for i in 0..self.len() {
            let c = &self.points[i];
            let d = &self.points[(i + 1) % self.len()];

            if Point::intersect_segment(a, b, c, d).is_some() {
                return false;
            }

            if Point::is_on_segment(a, b, c) {
                let t = if (b.x - a.x).abs() > (b.y - a.y).abs() {
                    (c.x - a.x) / (b.x - a.x)
                } else {
                    (c.y - a.y) / (b.y - a.y)
                };

                if t > -EPS && t < 1.0 + EPS {
                    ts.push(t);
                }
            }

            if Point::is_on_segment(a, b, d) {
                let t = if (b.x - a.x).abs() > (b.y - a.y).abs() {
                    (d.x - a.x) / (b.x - a.x)
                } else {
                    (d.y - a.y) / (b.y - a.y)
                };

                if t > -EPS && t < 1.0 + EPS {
                    ts.push(t);
                }
            }
        }

        ts.sort_unstable_by(|a, b| a.partial_cmp(b).unwrap());
        ts.dedup_by(|a, b| (*a - *b).abs() < EPS);

        for i in 0..ts.len() - 1 {
            let mid = (ts[i] + ts[i + 1]) / 2.0;
            let p = *a + (*b - *a) * mid;

            if !self.is_inside(&p, false) {
                return false;
            }
        }

        true
    }
}

#[derive(PartialEq, Clone, Copy)]
struct MinNonNan(f64);

impl Eq for MinNonNan {}

impl PartialOrd for MinNonNan {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.0.partial_cmp(&other.0)
    }
}

impl Ord for MinNonNan {
    fn cmp(&self, other: &MinNonNan) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

fn construct_visibility_graph(
    polygon: &Polygon,
    start: &Point,
    end: &Point,
) -> Vec<Vec<(usize, MinNonNan)>> {
    let mut vertices = polygon.points.clone();
    vertices.push(*start);
    vertices.push(*end);

    let mut ret: Vec<Vec<(usize, MinNonNan)>> = vec![Vec::new(); vertices.len()];

    for i in 0..vertices.len() {
        for j in i + 1..vertices.len() {
            if polygon.is_visible(&vertices[i], &vertices[j]) {
                let dist = (vertices[i] - vertices[j]).dist();
                ret[i].push((j, MinNonNan(dist)));
                ret[j].push((i, MinNonNan(dist)));
            }
        }
    }

    ret
}

fn process_dijkstra(graph: &Vec<Vec<(usize, MinNonNan)>>, from: usize, to: usize) -> f64 {
    let mut ret = vec![MinNonNan(f64::MAX); graph.len()];
    ret[from] = MinNonNan(0.0);

    let mut queue = BinaryHeap::new();
    queue.push((MinNonNan(0.0), from));

    while !queue.is_empty() {
        let (mut cost_curr, vertex_curr) = queue.pop().unwrap();
        cost_curr.0 *= -1.0;

        if ret[vertex_curr] < cost_curr {
            continue;
        }

        for info in graph[vertex_curr].iter() {
            let (vertex_next, mut cost_next) = *info;
            cost_next.0 += cost_curr.0;

            if ret[vertex_next] > cost_next {
                ret[vertex_next] = cost_next;
                queue.push((MinNonNan(-cost_next.0), vertex_next));
            }
        }
    }

    ret[to].0
}

fn check_dist_shortest(
    riverside_west: &Vec<Point>,
    riverside_east: &Vec<Point>,
    s: &Point,
    t: &Point,
    point_west: &Point,
    point_east: &Point,
    ret_bridge: &mut f64,
    ret_highway: &mut f64,
) {
    let dist_bridge = (*point_west - *point_east).dist();

    let graph_west =
        construct_visibility_graph(&Polygon::new(riverside_west.clone()), s, point_west);
    let dist_west = process_dijkstra(&graph_west, riverside_west.len(), riverside_west.len() + 1);

    let graph_east =
        construct_visibility_graph(&Polygon::new(riverside_east.clone()), point_east, t);
    let dist_east = process_dijkstra(&graph_east, riverside_east.len(), riverside_east.len() + 1);

    let dist_total = dist_bridge + dist_west + dist_east;

    if (*ret_bridge - dist_bridge).abs() < 1e-15 {
        if *ret_highway > dist_total {
            *ret_highway = dist_total;
        }
    } else if *ret_bridge > dist_bridge {
        *ret_bridge = dist_bridge;
        *ret_highway = dist_total;
    }
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let (sx, sy, tx, ty) = (
        scan.token::<f64>(),
        scan.token::<f64>(),
        scan.token::<f64>(),
        scan.token::<f64>(),
    );
    let s = Point::new(sx, sy);
    let t = Point::new(tx, ty);

    let n = scan.token::<usize>();
    let mut riverside_west = Vec::with_capacity(n);

    for _ in 0..n {
        let (wx, wy) = (scan.token::<f64>(), scan.token::<f64>());
        riverside_west.push(Point::new(wx, wy));
    }

    let m = scan.token::<usize>();
    let mut riverside_east = Vec::with_capacity(m);

    for _ in 0..m {
        let (ex, ey) = (scan.token::<f64>(), scan.token::<f64>());
        riverside_east.push(Point::new(ex, ey));
    }

    riverside_west.insert(0, Point::new(0.0, 0.0));
    riverside_west.push(Point::new(0.0, 1000.0));

    riverside_east.insert(0, Point::new(1000.0, 0.0));
    riverside_east.push(Point::new(1000.0, 1000.0));

    let mut ret_bridge = f64::MAX / 4.0;
    let mut ret_highway = f64::MAX / 4.0;

    for i in 0..riverside_west.len() - 1 {
        let pa1 = riverside_west[i];
        let pa2 = riverside_west[i + 1];

        for j in 0..riverside_east.len() - 1 {
            let pb1 = riverside_east[j];
            let pb2 = riverside_east[j + 1];

            // 1. Project endpoint on the segment and calculate distance
            if let Some(p) = Point::project_on_segment(&pb1, &pb2, &pa1) {
                check_dist_shortest(
                    &riverside_west,
                    &riverside_east,
                    &s,
                    &t,
                    &pa1,
                    &p,
                    &mut ret_bridge,
                    &mut ret_highway,
                );
            }

            if let Some(p) = Point::project_on_segment(&pb1, &pb2, &pa2) {
                check_dist_shortest(
                    &riverside_west,
                    &riverside_east,
                    &s,
                    &t,
                    &pa2,
                    &p,
                    &mut ret_bridge,
                    &mut ret_highway,
                );
            }

            if let Some(p) = Point::project_on_segment(&pa1, &pa2, &pb1) {
                check_dist_shortest(
                    &riverside_west,
                    &riverside_east,
                    &s,
                    &t,
                    &p,
                    &pb1,
                    &mut ret_bridge,
                    &mut ret_highway,
                );
            }

            if let Some(p) = Point::project_on_segment(&pa1, &pa2, &pb2) {
                check_dist_shortest(
                    &riverside_west,
                    &riverside_east,
                    &s,
                    &t,
                    &p,
                    &pb2,
                    &mut ret_bridge,
                    &mut ret_highway,
                );
            }

            // 2. Calculate distance from one endpoint to other
            check_dist_shortest(
                &riverside_west,
                &riverside_east,
                &s,
                &t,
                &pa1,
                &pb1,
                &mut ret_bridge,
                &mut ret_highway,
            );
            check_dist_shortest(
                &riverside_west,
                &riverside_east,
                &s,
                &t,
                &pa1,
                &pb2,
                &mut ret_bridge,
                &mut ret_highway,
            );
            check_dist_shortest(
                &riverside_west,
                &riverside_east,
                &s,
                &t,
                &pa2,
                &pb1,
                &mut ret_bridge,
                &mut ret_highway,
            );
            check_dist_shortest(
                &riverside_west,
                &riverside_east,
                &s,
                &t,
                &pa2,
                &pb2,
                &mut ret_bridge,
                &mut ret_highway,
            );

            // 3. Check parallel or overlapping segments
            if sign(((pa2 - pa1).cross(&(pb2 - pb1))).abs()) == 0 {
                let mut dir = pb2 - pb1;
                let len = dir.dist();

                if len < EPS {
                    continue;
                }

                dir = dir.normalize();

                let sa1 = pa1.dot(&dir);
                let sa2 = pa2.dot(&dir);
                let sb1 = pb1.dot(&dir);
                let sb2 = pb2.dot(&dir);

                let mut left = (sa1.min(sa2)).max(sb1.min(sb2));
                let mut right = (sa1.max(sa2)).min(sb1.max(sb2));

                if sign(left - right) >= 0 {
                    continue;
                }

                let sa = pa1 + dir * (left - sa1);
                let sb = pb1 + dir * (left - sb1);
                let dist_bridge = (sa - sb).dist();

                if dist_bridge > ret_bridge + EPS {
                    continue;
                }

                let check = |smid: f64| -> f64 {
                    let pw = pa1 + dir * (smid - sa1);
                    let pe = pb1 + dir * (smid - sb1);

                    let graph_west =
                        construct_visibility_graph(&Polygon::new(riverside_west.clone()), &s, &pw);
                    let dist_west = process_dijkstra(
                        &graph_west,
                        riverside_west.len(),
                        riverside_west.len() + 1,
                    );

                    let graph_east =
                        construct_visibility_graph(&Polygon::new(riverside_east.clone()), &pe, &t);
                    let dist_east = process_dijkstra(
                        &graph_east,
                        riverside_east.len(),
                        riverside_east.len() + 1,
                    );

                    dist_west + dist_east
                };

                for _ in 0..40 {
                    let mid1 = (left * 2.0 + right) / 3.0;
                    let mid2 = (left + right * 2.0) / 3.0;

                    let dist1 = check(mid1);
                    let dist2 = check(mid2);

                    if dist1 < dist2 {
                        right = mid2;
                    } else {
                        left = mid1;
                    }
                }

                let mid = (left + right) / 2.0;
                let pw = pa1 + dir * (mid - sa1);
                let pe = pb1 + dir * (mid - sb1);

                check_dist_shortest(
                    &riverside_west,
                    &riverside_east,
                    &s,
                    &t,
                    &pw,
                    &pe,
                    &mut ret_bridge,
                    &mut ret_highway,
                );
            }
        }
    }

    writeln!(out, "{:.12} {:.12}", ret_bridge, ret_highway).unwrap();
}
