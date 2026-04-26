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

#[derive(Clone, Copy, Debug, Default)]
struct Point {
    x: f64,
    y: f64,
}

impl Point {
    #[inline(always)]
    fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }

    #[inline(always)]
    fn dot(self, other: &Self) -> f64 {
        self.x * other.x + self.y * other.y
    }

    #[inline(always)]
    fn cross(&self, other: &Point) -> f64 {
        self.x * other.y - self.y * other.x
    }

    #[inline(always)]
    fn ccw(p1: Point, p2: Point, p3: Point) -> f64 {
        (p2 - p1).cross(&(p3 - p1))
    }

    #[inline(always)]
    fn norm(&self) -> f64 {
        self.dot(self).sqrt()
    }

    #[inline(always)]
    fn rotate(self, cos: f64, sin: f64) -> Self {
        Self::new(cos * self.x - sin * self.y, sin * self.x + cos * self.y)
    }

    #[inline(always)]
    fn normal_left(self) -> Self {
        Self::new(-self.y, self.x)
    }
}

impl std::ops::Add for Point {
    type Output = Point;

    fn add(self, rhs: Point) -> Point {
        Point::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl std::ops::Sub for Point {
    type Output = Point;

    fn sub(self, rhs: Point) -> Point {
        Point::new(self.x - rhs.x, self.y - rhs.y)
    }
}

impl std::ops::Mul<f64> for Point {
    type Output = Point;

    fn mul(self, rhs: f64) -> Point {
        Point::new(self.x * rhs, self.y * rhs)
    }
}

impl std::ops::Neg for Point {
    type Output = Point;

    fn neg(self) -> Point {
        Point::new(-self.x, -self.y)
    }
}

#[derive(Clone, Copy, Debug)]
struct Edge {
    from: Point,
    to: Point,
    dir: Point,
    len: f64,
    unit: Point,
}

impl Edge {
    fn new(from: Point, to: Point) -> Self {
        let dir = to - from;
        let len = dir.norm();

        Self {
            from,
            to,
            dir,
            len,
            unit: dir * (1.0 / len),
        }
    }

    fn interval_on(&self, axis: &Point) -> (f64, f64) {
        let a = self.from.dot(axis);
        let b = self.to.dot(axis);

        if a < b {
            (a, b)
        } else {
            (b, a)
        }
    }
}

const EPS: f64 = 1e-9;

mod triangulation {
    use crate::{Point, Polygon, EPS};

    pub(crate) type Triangle = [usize; 3];

    fn orientation(polygon: &Polygon) -> f64 {
        if polygon.area2() >= 0.0 {
            1.0
        } else {
            -1.0
        }
    }

    fn convex_in_polygon_order(a: Point, b: Point, c: Point, orient: f64) -> bool {
        orient * Point::ccw(a, b, c) > EPS
    }

    fn in_triangle_or_on_boundary(a: Point, b: Point, c: Point, p: Point, orient: f64) -> bool {
        orient * Point::ccw(a, b, p) >= -EPS
            && orient * Point::ccw(b, c, p) >= -EPS
            && orient * Point::ccw(c, a, p) >= -EPS
    }

    fn is_ear(polygon: &Polygon, alive: &[usize], pos: usize, orient: f64) -> bool {
        let m = alive.len();

        let idx_a = alive[(pos + m - 1) % m];
        let idx_b = alive[pos];
        let idx_c = alive[(pos + 1) % m];

        let a = polygon.vertices[idx_a];
        let b = polygon.vertices[idx_b];
        let c = polygon.vertices[idx_c];

        if !convex_in_polygon_order(a, b, c, orient) {
            return false;
        }

        for &idx in alive {
            if idx == idx_a || idx == idx_b || idx == idx_c {
                continue;
            }

            let p = polygon.vertices[idx];

            if in_triangle_or_on_boundary(a, b, c, p, orient) {
                return false;
            }
        }

        true
    }

    pub(crate) fn triangulate(polygon: &Polygon) -> Vec<Triangle> {
        let n = polygon.len();

        if n == 3 {
            return vec![[0, 1, 2]];
        }

        let orient = orientation(polygon);
        let mut alive = (0..n).collect::<Vec<_>>();
        let mut triangles = Vec::with_capacity(n - 2);

        while alive.len() > 3 {
            let mut pos = None;

            for i in 0..alive.len() {
                if is_ear(polygon, &alive, i, orient) {
                    pos = Some(i);
                    break;
                }
            }

            let Some(pos) = pos else {
                panic!("Triangulation failed");
            };

            let m = alive.len();

            let idx_a = alive[(pos + m - 1) % m];
            let idx_b = alive[pos];
            let idx_c = alive[(pos + 1) % m];

            triangles.push([idx_a, idx_b, idx_c]);
            alive.remove(pos);
        }

        triangles.push([alive[0], alive[1], alive[2]]);
        triangles
    }
}

#[derive(Clone, Debug)]
struct Polygon {
    vertices: Vec<Point>,
    edges: Vec<Edge>,
}

impl Polygon {
    fn new(vertices: Vec<Point>) -> Self {
        let edges = Self::build_edges(&vertices);

        Self { vertices, edges }
    }

    fn build_edges(vertices: &Vec<Point>) -> Vec<Edge> {
        let n = vertices.len();

        (0..n)
            .map(|i| Edge::new(vertices[i], vertices[(i + 1) % n]))
            .collect()
    }

    fn len(&self) -> usize {
        self.vertices.len()
    }

    fn area2(&self) -> f64 {
        let mut area = 0.0;

        for i in 0..self.len() {
            area += self.vertices[i].cross(&self.vertices[(i + 1) % self.len()]);
        }

        area
    }

    fn rotated(&self, cos: f64, sin: f64) -> Polygon {
        let vertices = self
            .vertices
            .iter()
            .map(|&p| p.rotate(cos, sin))
            .collect::<Vec<_>>();
        Self::new(vertices)
    }
}

#[derive(Clone, Copy, Debug)]
struct Interval {
    left: f64,
    right: f64,
}

impl Interval {
    fn new(left: f64, right: f64) -> Option<Self> {
        if right - left > EPS {
            Some(Self { left, right })
        } else {
            None
        }
    }

    fn contains_open(self, x: f64) -> bool {
        x > self.left + EPS && x < self.right - EPS
    }

    fn merge(mut intervals: Vec<Interval>) -> Vec<Interval> {
        intervals.sort_unstable_by(|a, b| {
            a.left
                .partial_cmp(&b.left)
                .unwrap_or(std::cmp::Ordering::Equal)
        });

        let mut merged: Vec<Interval> = Vec::new();

        for interval in intervals {
            if let Some(last) = merged.last_mut() {
                if interval.left < last.right - EPS {
                    last.right = last.right.max(interval.right);
                } else {
                    merged.push(interval);
                }
            } else {
                merged.push(interval);
            }
        }

        merged
    }
}

fn forbidden_interval(
    triangle_a: [Point; 3],
    triangle_b: [Point; 3],
    base: Point,
    axis: Point,
) -> Option<Interval> {
    let normal = axis.normal_left();
    let mut dists = [0.0; 9];
    let mut params = [0.0; 9];
    let mut cnt = 0;

    let mut dist_min = f64::MAX;
    let mut dist_max = f64::MIN;

    for &p in triangle_a.iter() {
        for &q in triangle_b.iter() {
            let h = p - q - base;
            let dist = normal.dot(&h);
            let param = axis.dot(&h);

            dists[cnt] = dist;
            params[cnt] = param;
            cnt += 1;

            dist_min = dist_min.min(dist);
            dist_max = dist_max.max(dist);
        }
    }

    if dist_max <= EPS || dist_min >= -EPS {
        return None;
    }

    let mut left = f64::MAX;
    let mut right = f64::MIN;
    let mut found = false;

    for i in 0..9 {
        if dists[i].abs() <= EPS {
            left = left.min(params[i]);
            right = right.max(params[i]);
            found = true;
        }
    }

    for i in 0..9 {
        for j in i + 1..9 {
            if (dists[i] > EPS && dists[j] < -EPS) || (dists[i] < -EPS && dists[j] > EPS) {
                let alpha = -dists[i] / (dists[j] - dists[i]);
                let s = params[i] + alpha * (params[j] - params[i]);

                left = left.min(s);
                right = right.max(s);
                found = true;
            }
        }
    }

    if found {
        Interval::new(left, right)
    } else {
        None
    }
}

fn forbidden_intervals(
    a: &Polygon,
    b: &Polygon,
    b_rotated: &Polygon,
    base: Point,
    axis: Point,
) -> Vec<Interval> {
    let (triangles_a, triangles_b) = (triangulation::triangulate(a), triangulation::triangulate(b));
    let mut intervals = Vec::with_capacity(triangles_a.len() * triangles_b.len());

    for &idx_a in triangles_a.iter() {
        let triangle_a = [
            a.vertices[idx_a[0]],
            a.vertices[idx_a[1]],
            a.vertices[idx_a[2]],
        ];

        for &idx_b in triangles_b.iter() {
            let triangle_b = [
                b_rotated.vertices[idx_b[0]],
                b_rotated.vertices[idx_b[1]],
                b_rotated.vertices[idx_b[2]],
            ];

            if let Some(interval) = forbidden_interval(triangle_a, triangle_b, base, axis) {
                intervals.push(interval);
            }
        }
    }

    Interval::merge(intervals)
}

#[derive(Clone, Copy, Debug)]
struct Event {
    x: f64,
    delta: f64,
}

impl Event {
    fn new(x: f64, delta: f64) -> Self {
        Self { x, delta }
    }

    fn merge(mut events: Vec<Event>) -> Vec<Event> {
        events.sort_unstable_by(|a, b| a.x.partial_cmp(&b.x).unwrap_or(std::cmp::Ordering::Equal));

        let mut merged: Vec<Event> = Vec::new();

        for event in events {
            if let Some(last) = merged.last_mut() {
                if (event.x - last.x).abs() <= EPS {
                    last.delta += event.delta;
                } else {
                    merged.push(event);
                }
            } else {
                merged.push(event);
            }
        }

        merged
    }
}

#[derive(Default)]
struct ContactEvents {
    slope_events: Vec<Event>,
    point_events: Vec<Event>,
    candidates: Vec<f64>,
}

impl ContactEvents {
    fn add_candidate(&mut self, x: f64) {
        self.candidates.push(x);
    }

    fn add_slope_event(&mut self, x: f64, delta: f64) {
        self.slope_events.push(Event::new(x, delta));
        self.add_candidate(x);
    }

    fn add_point_event(&mut self, x: f64, value: f64) {
        if value > EPS {
            self.point_events.push(Event::new(x, value));
            self.add_candidate(x);
        }
    }

    fn add_forbidden_endpoints(&mut self, intervals: &[Interval]) {
        for interval in intervals {
            self.add_candidate(interval.left);
            self.add_candidate(interval.right);
        }
    }

    fn is_inside_forbidden(x: f64, intervals: &[Interval]) -> bool {
        let mut left = 0;
        let mut right = intervals.len();

        while left < right {
            let mid = (left + right) / 2;

            if intervals[mid].left < x {
                left = mid + 1;
            } else {
                right = mid;
            }
        }

        left > 0 && intervals[left - 1].contains_open(x)
    }

    fn sweep(self, forbidden: &Vec<Interval>) -> f64 {
        let slope_events = Event::merge(self.slope_events);
        let point_events = Event::merge(self.point_events);
        let candidates = {
            let mut candidates = self.candidates;
            candidates
                .sort_unstable_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));

            let mut ret: Vec<f64> = Vec::new();

            for candidate in candidates {
                if ret
                    .last()
                    .map_or(true, |&last| (candidate - last).abs() > EPS)
                {
                    ret.push(candidate);
                }
            }

            ret
        };

        if candidates.is_empty() {
            return 0.0;
        }

        let mut prev = candidates[0];
        let mut slope = 0.0f64;
        let mut val = 0.0f64;
        let mut ret = 0.0f64;

        let mut idx_slope = 0;
        let mut idx_point = 0;

        for &candidate in candidates.iter() {
            val += slope * (candidate - prev);

            if val < 0.0 && val > -EPS {
                val = 0.0;
            }

            let mut delta_sum = 0.0;

            while idx_point < point_events.len() && point_events[idx_point].x < candidate - EPS {
                idx_point += 1;
            }

            let mut idx = idx_point;

            while idx < point_events.len() && (point_events[idx].x - candidate).abs() <= EPS {
                delta_sum += point_events[idx].delta;
                idx += 1;
            }

            if !Self::is_inside_forbidden(candidate, forbidden) {
                ret = ret.max(val + delta_sum);
            }

            while idx_slope < slope_events.len() && slope_events[idx_slope].x < candidate - EPS {
                slope += slope_events[idx_slope].delta;
                idx_slope += 1;
            }

            while idx_slope < slope_events.len()
                && (slope_events[idx_slope].x - candidate).abs() <= EPS
            {
                slope += slope_events[idx_slope].delta;
                idx_slope += 1;
            }

            idx_point = idx;
            prev = candidate;
        }

        ret
    }
}

fn add_events(events: &mut ContactEvents, a: Edge, b: Edge, base: Point, axis: Point) {
    let are_oppositely_parallel = a.dir.cross(&b.dir).abs() <= EPS * a.len * b.len
        && a.dir.dot(&b.dir) < -EPS * a.len * b.len;

    if !are_oppositely_parallel {
        return;
    }

    let val_line_curr = a.dir.cross(&(b.from + base - a.from));
    let val_line_speed = a.dir.cross(&axis);

    if val_line_speed.abs() <= EPS * a.len {
        if val_line_curr.abs() > EPS * a.len {
            return;
        }

        add_parallel_contact_events(events, a, b, base, axis);
    } else {
        let slope = -val_line_curr / val_line_speed;
        add_point_contact_event(events, a, b, base, axis, slope);
    }
}

fn add_parallel_contact_events(
    events: &mut ContactEvents,
    a: Edge,
    b: Edge,
    base: Point,
    axis: Point,
) {
    let (left, right) = a.interval_on(&a.unit);
    let q0 = (b.from + base).dot(&a.unit);
    let q1 = (b.to + base).dot(&a.unit);
    let (q_left, q_right) = if q0 < q1 { (q0, q1) } else { (q1, q0) };
    let speed = axis.dot(&a.unit);
    let sign = if speed >= 0.0 { 1.0 } else { -1.0 };

    let changes = [
        (left - q_right, 1.0),
        (left - q_left, -1.0),
        (right - q_right, -1.0),
        (right - q_left, 1.0),
    ];

    for &(x, delta) in changes.iter() {
        events.add_slope_event(x / sign, delta);
    }
}

fn add_point_contact_event(
    events: &mut ContactEvents,
    a: Edge,
    b: Edge,
    base: Point,
    axis: Point,
    slope: f64,
) {
    let translation = base + axis * slope;
    let (left, right) = a.interval_on(&a.unit);
    let q0 = (b.from + translation).dot(&a.unit);
    let q1 = (b.to + translation).dot(&a.unit);
    let (q_left, q_right) = if q0 < q1 { (q0, q1) } else { (q1, q0) };
    let overlap = {
        let left = left.max(q_left);
        let right = right.min(q_right);
        (right - left).max(0.0)
    };

    events.add_point_event(slope, overlap);
}

fn calculate(a: &Polygon, b: &Polygon, b_rotated: &Polygon, base: Point, axis: Point) -> f64 {
    let forbidden = forbidden_intervals(a, b, b_rotated, base, axis);
    let mut events = ContactEvents::default();

    events.add_forbidden_endpoints(&forbidden);

    for &edge_a in a.edges.iter() {
        for &edge_b in b_rotated.edges.iter() {
            add_events(&mut events, edge_a, edge_b, base, axis);
        }
    }

    events.sweep(&forbidden)
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let a = {
        let n = scan.token::<usize>();
        let mut points = Vec::with_capacity(n);

        for _ in 0..n {
            let (x, y) = (scan.token::<f64>(), scan.token::<f64>());
            points.push(Point::new(x, y));
        }

        Polygon::new(points)
    };
    let b = {
        let n = scan.token::<usize>();
        let mut points = Vec::with_capacity(n);

        for _ in 0..n {
            let (x, y) = (scan.token::<f64>(), scan.token::<f64>());
            points.push(Point::new(x, y));
        }

        Polygon::new(points)
    };
    let mut ret = 0.0f64;

    for i in 0..a.len() {
        let edge_a = a.edges[i];

        for j in 0..b.len() {
            let edge_b = b.edges[j];
            let (cos, sin) = (
                edge_b.unit.dot(&(-edge_a.unit)),
                edge_b.unit.cross(&(-edge_a.unit)),
            );
            let b_rotated = b.rotated(cos, sin);
            let base = edge_a.from - b_rotated.vertices[(j + 1) % b.len()];

            let val = calculate(&a, &b, &b_rotated, base, edge_a.unit);
            ret = ret.max(val);
        }
    }

    writeln!(out, "{:.12}", ret).unwrap();
}
