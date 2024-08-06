use io::Write;
use std::{f64, fmt, io, str, usize};

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

pub const EPSILON: f64 = f64::EPSILON * 2.0;
pub const INVALID_INDEX: usize = usize::max_value();

pub trait Coord: Sync + Send + Clone {
    fn from_xy(x: f64, y: f64) -> Self;
    fn x(&self) -> f64;
    fn y(&self) -> f64;

    fn magnitude2(&self) -> f64 {
        self.x() * self.x() + self.y() * self.y()
    }
}

pub trait Vector<C: Coord> {
    fn vector(p: &C, q: &C) -> C {
        C::from_xy(q.x() - p.x(), q.y() - p.y())
    }

    fn determinant(p: &C, q: &C) -> f64 {
        p.x() * q.y() - p.y() * q.x()
    }

    fn dist2(p: &C, q: &C) -> f64 {
        let d = Self::vector(p, q);

        d.x() * d.x() + d.y() * d.y()
    }

    fn equals(p: &C, q: &C) -> bool {
        (p.x() - q.x()).abs() <= EPSILON && (p.y() - q.y()).abs() <= EPSILON
    }

    fn equals_with_span(p: &C, q: &C, span: f64) -> bool {
        let dist = Self::dist2(p, q) / span;
        dist < 1e-20
    }
}

#[derive(Copy, Clone, PartialEq, PartialOrd)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

impl Point {
    fn ccw(p1: &Point, p2: &Point, p3: &Point) -> f64 {
        let (x1, y1) = (p1.x, p1.y);
        let (x2, y2) = (p2.x, p2.y);
        let (x3, y3) = (p3.x, p3.y);

        (x2 - x1) * (y3 - y1) - (x3 - x1) * (y2 - y1)
    }
}

impl Coord for Point {
    #[inline(always)]
    fn from_xy(x: f64, y: f64) -> Self {
        Point { x, y }
    }

    #[inline(always)]
    fn x(&self) -> f64 {
        self.x
    }

    #[inline(always)]
    fn y(&self) -> f64 {
        self.y
    }
}

impl Vector<Point> for Point {}

impl fmt::Debug for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}, {}]", self.x, self.y)
    }
}

impl From<(f64, f64)> for Point {
    fn from((x, y): (f64, f64)) -> Self {
        Point { x, y }
    }
}

fn in_circle<C: Coord + Vector<C>>(p: &C, a: &C, b: &C, c: &C) -> bool {
    let d = C::vector(p, a);
    let e = C::vector(p, b);
    let f = C::vector(p, c);

    let ap = d.x() * d.x() + d.y() * d.y();
    let bp = e.x() * e.x() + e.y() * e.y();
    let cp = f.x() * f.x() + f.y() * f.y();

    #[rustfmt::skip]
    let res = d.x() * (e.y() * cp  - bp  * f.y()) -
                   d.y() * (e.x() * cp  - bp  * f.x()) +
                   ap  * (e.x() * f.y() - e.y() * f.x()) ;

    res < 0.0
}

#[rustfmt::skip]
fn circumradius<C: Coord + Vector<C>>(a: &C, b: &C, c: &C) -> f64 {
    let d = C::vector(a, b);
    let e = C::vector(a, c);

    let bl = d.magnitude2();
    let cl = e.magnitude2();
    let det = C::determinant(&d, &e);

    let x = (e.y() * bl - d.y() * cl) * (0.5 / det);
    let y = (d.x() * cl - e.x() * bl) * (0.5 / det);

    if (bl != 0.0) &&
       (cl != 0.0) &&
       (det != 0.0) {
        x * x + y * y
    } else {
        f64::MAX
    }
}

#[rustfmt::skip]
pub fn circumcenter<C: Coord + Vector<C>>(a: &C, b: &C, c: &C) -> Option<C> {
    let d = C::vector(a, b);
    let e = C::vector(a, c);

    let bl = d.magnitude2();
    let cl = e.magnitude2();
    let det = C::determinant(&d, &e);

    let x = (e.y() * bl - d.y() * cl) * (0.5 / det);
    let y = (d.x() * cl - e.x() * bl) * (0.5 / det);

    if (bl != 0.0) &&
       (cl != 0.0) &&
       (det != 0.0) {
        Some(C::from_xy(
            a.x() + x,
            a.y() + y)
        )
    } else {
        None
    }
}

fn counter_clockwise<C: Coord + Vector<C>>(p0: &C, p1: &C, p2: &C) -> bool {
    let v0 = C::vector(p0, p1);
    let v1 = C::vector(p0, p2);
    let det = C::determinant(&v0, &v1);
    let dist = v0.magnitude2() + v1.magnitude2();

    if det == 0. {
        return false;
    }

    let reldet = (dist / det).abs();

    if reldet > 1e14 {
        return false;
    }

    det > 0.
}

pub fn next_halfedge(i: usize) -> usize {
    if i % 3 == 2 {
        i - 2
    } else {
        i + 1
    }
}

pub fn prev_halfedge(i: usize) -> usize {
    if i % 3 == 0 {
        i + 2
    } else {
        i - 1
    }
}

pub fn edges_of_triangle(t: usize) -> [usize; 3] {
    [3 * t, 3 * t + 1, 3 * t + 2]
}

pub fn triangle_of_edge(e: usize) -> usize {
    ((e as f64) / 3.).floor() as usize
}

pub fn points_of_triangle(t: usize, delaunay: &Triangulation) -> Vec<usize> {
    let edges = edges_of_triangle(t);
    edges.iter().map(|e| delaunay.triangles[*e]).collect()
}

pub fn triangles_adjacent_to_triangle(t: usize, delaunay: &Triangulation) -> Vec<usize> {
    let mut adjacent_triangles: Vec<usize> = vec![];
    for e in edges_of_triangle(t).iter() {
        let opposite = delaunay.halfedges[*e];
        if opposite != INVALID_INDEX {
            adjacent_triangles.push(triangle_of_edge(opposite));
        }
    }
    adjacent_triangles
}

pub fn edges_around_point(start: usize, delaunay: &Triangulation) -> Vec<usize> {
    let mut result: Vec<usize> = vec![];

    if start == INVALID_INDEX {
        return result;
    }

    let mut incoming = start;
    loop {
        result.push(incoming);
        let outgoing = next_halfedge(incoming);
        incoming = delaunay.halfedges[outgoing];
        if incoming == INVALID_INDEX || incoming == start {
            break;
        }
    }
    result
}

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct Triangulation {
    pub triangles: Vec<usize>,
    pub halfedges: Vec<usize>,
    pub hull: Vec<usize>,
    pub inedges: Vec<usize>,
    pub outedges: Vec<usize>,
}

impl Triangulation {
    fn new(n: usize) -> Self {
        let max_triangles = 2 * n - 5;
        Self {
            triangles: Vec::with_capacity(max_triangles * 3),
            halfedges: Vec::with_capacity(max_triangles * 3),
            hull: Vec::new(),
            inedges: vec![INVALID_INDEX; n],
            outedges: vec![INVALID_INDEX; n],
        }
    }

    pub fn len(&self) -> usize {
        self.triangles.len() / 3
    }

    fn legalize<C: Coord + Vector<C>>(
        &mut self,
        p: usize,
        points: &[C],
        hull: &mut Hull<C>,
    ) -> usize {
        let mut i: usize = 0;
        let mut ar;
        let mut a = p;

        let mut edge_stack: Vec<usize> = Vec::new();

        loop {
            let b = self.halfedges[a];
            ar = prev_halfedge(a);

            if b == INVALID_INDEX {
                if i > 0 {
                    i -= 1;
                    a = edge_stack[i];
                    continue;
                } else {
                    break;
                }
            }

            let al = next_halfedge(a);
            let bl = prev_halfedge(b);

            let p0 = self.triangles[ar];
            let pr = self.triangles[a];
            let pl = self.triangles[al];
            let p1 = self.triangles[bl];

            let illegal = in_circle(&points[p1], &points[p0], &points[pr], &points[pl]);
            if illegal {
                self.triangles[a] = p1;
                self.triangles[b] = p0;

                let hbl = self.halfedges[bl];

                if hbl == INVALID_INDEX {
                    let mut e = hull.start;
                    loop {
                        if hull.tri[e] == bl {
                            hull.tri[e] = a;
                            break;
                        }

                        e = hull.prev[e];

                        if e == hull.start {
                            break;
                        }
                    }
                }

                self.link(a, hbl);
                self.link(b, self.halfedges[ar]);
                self.link(ar, bl);

                let br = next_halfedge(b);

                if i < edge_stack.len() {
                    edge_stack[i] = br;
                } else {
                    edge_stack.push(br);
                }

                i += 1;
            } else if i > 0 {
                i -= 1;
                a = edge_stack[i];
                continue;
            } else {
                break;
            }
        }

        ar
    }

    fn link(&mut self, a: usize, b: usize) {
        let s: usize = self.halfedges.len();

        if a == s {
            self.halfedges.push(b);
        } else if a < s {
            self.halfedges[a] = b;
        } else {
            panic!("Cannot link edge")
        }

        if b != INVALID_INDEX {
            let s2: usize = self.halfedges.len();
            if b == s2 {
                self.halfedges.push(a);
            } else if b < s2 {
                self.halfedges[b] = a;
            } else {
                panic!("Cannot link edge")
            }
        }
    }

    fn add_triangle(
        &mut self,
        i0: usize,
        i1: usize,
        i2: usize,
        a: usize,
        b: usize,
        c: usize,
    ) -> usize {
        let t: usize = self.triangles.len();

        self.triangles.push(i0);
        self.triangles.push(i1);
        self.triangles.push(i2);

        self.link(t, a);
        self.link(t + 1, b);
        self.link(t + 2, c);

        t
    }
}

fn fast_mod(i: usize, c: usize) -> usize {
    if i >= c {
        i % c
    } else {
        i
    }
}

fn pseudo_angle<C: Coord + Vector<C>>(d: &C) -> f64 {
    let p = d.x() / (d.x().abs() + d.y().abs());
    if d.y() > 0.0 {
        (3.0 - p) / 4.0
    } else {
        (1.0 + p) / 4.0
    }
}

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
struct Hull<C: Coord> {
    prev: Vec<usize>,
    next: Vec<usize>,
    tri: Vec<usize>,
    hash: Vec<usize>,
    start: usize,
    center: C,
}

impl<C: Coord + Vector<C>> Hull<C> {
    fn new(n: usize, center: &C, i0: usize, i1: usize, i2: usize, points: &[C]) -> Self {
        let hash_len = (n as f64).sqrt().ceil() as usize;

        let mut hull = Self {
            prev: vec![0; n],
            next: vec![0; n],
            tri: vec![0; n],
            hash: vec![INVALID_INDEX; hash_len],
            start: i0,
            center: center.clone(),
        };

        hull.next[i0] = i1;
        hull.prev[i2] = i1;
        hull.next[i1] = i2;
        hull.prev[i0] = i2;
        hull.next[i2] = i0;
        hull.prev[i1] = i0;

        hull.tri[i0] = 0;
        hull.tri[i1] = 1;
        hull.tri[i2] = 2;

        hull.hash_edge(&points[i0], i0);
        hull.hash_edge(&points[i1], i1);
        hull.hash_edge(&points[i2], i2);

        hull
    }

    fn hash_key(&self, p: &C) -> usize {
        let d = C::vector(&self.center, p);

        let angle: f64 = pseudo_angle(&d);
        let len = self.hash.len();

        fast_mod((angle * (len as f64)).floor() as usize, len)
    }

    fn hash_edge(&mut self, p: &C, i: usize) {
        let key = self.hash_key(p);
        self.hash[key] = i;
    }

    fn find_visible_edge(&self, p: &C, span: f64, points: &[C]) -> (usize, bool) {
        let mut start = 0;
        let key = self.hash_key(p);
        for j in 0..self.hash.len() {
            let index = fast_mod(key + j, self.hash.len());
            start = self.hash[index];
            if start != INVALID_INDEX && start != self.next[start] {
                break;
            }
        }

        if self.prev[start] == start || self.prev[start] == INVALID_INDEX {
            panic!("not in the hull");
        }

        start = self.prev[start];
        let mut e = start;
        let mut q: usize;

        loop {
            q = self.next[e];

            if C::equals_with_span(p, &points[e], span) || C::equals_with_span(p, &points[q], span)
            {
                e = INVALID_INDEX;
                break;
            }
            if counter_clockwise(p, &points[e], &points[q]) {
                break;
            }
            e = q;
            if e == start {
                e = INVALID_INDEX;
                break;
            }
        }

        (e, e == start)
    }
}

fn calculate_bbox_center<C: Coord + Vector<C>>(points: &[C]) -> (C, f64) {
    let mut max = Point {
        x: f64::NEG_INFINITY,
        y: f64::NEG_INFINITY,
    };
    let mut min = Point {
        x: f64::INFINITY,
        y: f64::INFINITY,
    };

    for point in points {
        min.x = min.x.min(point.x());
        min.y = min.y.min(point.y());
        max.x = max.x.max(point.x());
        max.y = max.y.max(point.y());
    }

    let width = max.x - min.x;
    let height = max.y - min.y;
    let span = width * width + height * height;

    (
        C::from_xy((min.x + max.x) / 2.0, (min.y + max.y) / 2.0),
        span,
    )
}

fn find_closest_point<C: Coord + Vector<C>>(points: &[C], p: &C) -> usize {
    let mut min_dist = f64::MAX;
    let mut k = INVALID_INDEX;

    for (i, q) in points.iter().enumerate() {
        if i != k {
            let d = C::dist2(&p, &q);

            if d < min_dist && d > 0.0 {
                k = i;
                min_dist = d;
            }
        }
    }

    k
}

fn find_seed_triangle<C: Coord + Vector<C>>(
    center: &C,
    points: &[C],
) -> Option<(usize, usize, usize)> {
    let i0 = find_closest_point(points, center);
    let p0 = &points[i0];

    let mut i1 = find_closest_point(points, &p0);
    let p1 = &points[i1];

    let mut min_radius = f64::MAX;
    let mut i2 = INVALID_INDEX;
    for (i, p) in points.iter().enumerate() {
        if i != i0 && i != i1 {
            let r = circumradius(p0, p1, p);

            if r < min_radius {
                i2 = i;
                min_radius = r;
            }
        }
    }

    if min_radius == f64::MAX {
        None
    } else {
        let p2 = &points[i2];

        if counter_clockwise(p0, p1, p2) {
            std::mem::swap(&mut i1, &mut i2)
        }

        Some((i0, i1, i2))
    }
}

fn to_points<C: Coord + Vector<C>>(coords: &[f64]) -> Vec<C> {
    coords
        .chunks(2)
        .map(|tuple| C::from_xy(tuple[0], tuple[1]))
        .collect()
}

pub fn triangulate_from_arr<C: Coord + Vector<C>>(
    coords: &[f64],
) -> Option<(Triangulation, Vec<C>)> {
    let n = coords.len();

    if n % 2 != 0 {
        return None;
    }

    let points = to_points(coords);
    let triangulation = triangulate(&points)?;

    Some((triangulation, points))
}

pub fn triangulate_from_tuple<C: Coord + Vector<C>>(
    coords: &[(f64, f64)],
) -> Option<(Triangulation, Vec<C>)> {
    let points: Vec<C> = coords.iter().map(|p| C::from_xy(p.0, p.1)).collect();

    let triangulation = triangulate(&points)?;

    Some((triangulation, points))
}

pub fn triangulate<C: Coord + Vector<C>>(points: &[C]) -> Option<Triangulation> {
    if points.len() < 3 {
        return None;
    }

    let (center_bbox, span) = calculate_bbox_center(points);
    let (i0, i1, i2) = find_seed_triangle(&center_bbox, &points)?;

    let p0 = &points[i0];
    let p1 = &points[i1];
    let p2 = &points[i2];

    let center = circumcenter(p0, p1, p2).unwrap();

    let mut dists: Vec<(usize, f64)> = points
        .iter()
        .enumerate()
        .map(|(i, _)| (i, C::dist2(&points[i], &center)))
        .collect();

    dists.sort_unstable_by(|(_, a), (_, b)| a.partial_cmp(&b).unwrap());

    let mut hull = Hull::new(points.len(), &center, i0, i1, i2, points);

    let mut triangulation = Triangulation::new(points.len());
    triangulation.add_triangle(i0, i1, i2, INVALID_INDEX, INVALID_INDEX, INVALID_INDEX);

    let mut pp = C::from_xy(f64::NAN, f64::NAN);

    for (k, &(i, _)) in dists.iter().enumerate() {
        let p = &points[i];

        if k > 0 && C::equals(p, &pp) {
            continue;
        }

        if i == i0 || i == i1 || i == i2 {
            continue;
        }

        pp = p.clone();

        let (mut e, backwards) = hull.find_visible_edge(p, span, points);
        if e == INVALID_INDEX {
            continue;
        }

        let mut t = triangulation.add_triangle(
            e,
            i,
            hull.next[e],
            INVALID_INDEX,
            INVALID_INDEX,
            hull.tri[e],
        );

        hull.tri[i] = triangulation.legalize(t + 2, points, &mut hull);
        hull.tri[e] = t;

        let mut next = hull.next[e];
        loop {
            let q = hull.next[next];
            if !counter_clockwise(p, &points[next], &points[q]) {
                break;
            }
            t = triangulation.add_triangle(next, i, q, hull.tri[i], INVALID_INDEX, hull.tri[next]);

            hull.tri[i] = triangulation.legalize(t + 2, points, &mut hull);
            hull.next[next] = next;
            next = q;
        }

        if backwards {
            loop {
                let q = hull.prev[e];
                if !counter_clockwise(p, &points[q], &points[e]) {
                    break;
                }
                t = triangulation.add_triangle(q, i, e, INVALID_INDEX, hull.tri[e], hull.tri[q]);
                triangulation.legalize(t + 2, points, &mut hull);
                hull.tri[q] = t;
                hull.next[e] = e;
                e = q;
            }
        }

        hull.prev[i] = e;
        hull.next[e] = i;
        hull.prev[next] = i;
        hull.next[i] = next;
        hull.start = e;

        hull.hash_edge(p, i);
        hull.hash_edge(&points[e], e);
    }

    for e in 0..triangulation.triangles.len() {
        let endpoint = triangulation.triangles[next_halfedge(e)];
        if triangulation.halfedges[e] == INVALID_INDEX
            || triangulation.inedges[endpoint] == INVALID_INDEX
        {
            triangulation.inedges[endpoint] = e;
        }
    }

    let mut vert0: usize;
    let mut vert1 = hull.start;
    loop {
        vert0 = vert1;
        vert1 = hull.next[vert1];
        triangulation.inedges[vert1] = hull.tri[vert0];
        triangulation.outedges[vert0] = hull.tri[vert1];
        if vert1 == hull.start {
            break;
        }
    }

    let mut e = hull.start;
    loop {
        triangulation.hull.push(e);
        e = hull.next[e];
        if e == hull.start {
            break;
        }
    }

    Some(triangulation)
}

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct Polygon<C: Coord> {
    pub(crate) points: Vec<C>,
}

impl<C: Coord> Polygon<C> {
    pub fn new() -> Self {
        Polygon { points: Vec::new() }
    }

    pub fn from_points(points: Vec<C>) -> Self {
        Polygon { points }
    }

    pub fn points(&self) -> &[C] {
        &self.points
    }
}

fn inside<C: Coord>(p: &C, p1: &C, p2: &C) -> bool {
    (p2.y() - p1.y()) * p.x() + (p1.x() - p2.x()) * p.y() + (p2.x() * p1.y() - p1.x() * p2.y())
        < 0.0
}

fn intersection<C: Coord>(cp1: &C, cp2: &C, s: &C, e: &C) -> C {
    let dc = C::from_xy(cp1.x() - cp2.x(), cp1.y() - cp2.y());
    let dp = C::from_xy(s.x() - e.x(), s.y() - e.y());

    let n1 = cp1.x() * cp2.y() - cp1.y() * cp2.x();
    let n2 = s.x() * e.y() - s.y() * e.x();

    let n3 = 1.0 / (dc.x() * dp.y() - dc.y() * dp.x());

    C::from_xy(
        (n1 * dp.x() - n2 * dc.x()) * n3,
        (n1 * dp.y() - n2 * dc.y()) * n3,
    )
}

pub fn sutherland_hodgman<C: Coord + Clone>(subject: &Polygon<C>, clip: &Polygon<C>) -> Polygon<C> {
    let mut output_polygon = Polygon::new();
    let mut input_polygon = Polygon::new();

    output_polygon.points.clone_from(&subject.points);

    let mut new_polygon_size = subject.points.len();

    for j in 0..clip.points.len() {
        input_polygon.points.clear();
        input_polygon.points.clone_from(&output_polygon.points);

        let mut counter = 0;
        output_polygon.points.clear();

        let cp1 = &clip.points[j];
        let cp2 = &clip.points[(j + 1) % clip.points.len()];

        for i in 0..new_polygon_size {
            let s = &input_polygon.points[i];
            let e = &input_polygon.points[(i + 1) % new_polygon_size];

            if inside(s, cp1, cp2) && inside(e, cp1, cp2) {
                output_polygon.points.push(e.clone());
                counter += 1;
            } else if !inside(s, cp1, cp2) && inside(e, cp1, cp2) {
                output_polygon.points.push(intersection(cp1, cp2, s, e));
                output_polygon.points.push(e.clone());

                counter += 1;
                counter += 1;
            } else if inside(s, cp1, cp2) && !inside(e, cp1, cp2) {
                output_polygon.points.push(intersection(cp1, cp2, s, e));
                counter += 1;
            }
        }

        new_polygon_size = counter;
    }

    output_polygon
}

fn helper_points<C: Coord>(polygon: &Polygon<C>) -> Vec<C> {
    let mut points = vec![];

    let mut min = Point {
        x: f64::MAX,
        y: f64::MAX,
    };
    let mut max = Point {
        x: f64::MIN,
        y: f64::MIN,
    };

    for point in polygon.points() {
        if point.x() < min.x() {
            min.x = point.x();
        }
        if point.x() > max.x() {
            max.x = point.x();
        }
        if point.y() < min.y() {
            min.y = point.y();
        }
        if point.y() > max.y() {
            max.y = point.y();
        }
    }

    let width = max.x() - min.x();
    let height = max.y() - min.y();

    points.push(C::from_xy(min.x() - width, min.y() + height / 2.0));
    points.push(C::from_xy(max.x() + width, min.y() + height / 2.0));
    points.push(C::from_xy(min.x() + width / 2.0, min.y() - height));
    points.push(C::from_xy(min.x() + width / 2.0, max.y() + height));

    points
}

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct VoronoiDiagram<C: Coord + Vector<C>> {
    pub sites: Vec<C>,
    pub delaunay: Triangulation,
    pub centers: Vec<C>,
    cells: Vec<Polygon<C>>,
    pub neighbors: Vec<Vec<usize>>,
    num_helper_points: usize,
}

impl<C: Coord + Vector<C>> VoronoiDiagram<C> {
    pub fn new(min: &C, max: &C, points: &[C]) -> Option<Self> {
        let clip_points = vec![
            C::from_xy(min.x(), min.y()),
            C::from_xy(max.x(), min.y()),
            C::from_xy(max.x(), max.y()),
            C::from_xy(min.x(), max.y()),
        ];
        let clip_polygon = Polygon::from_points(clip_points);

        VoronoiDiagram::with_bounding_polygon(points.to_vec(), &clip_polygon)
    }

    pub fn with_bounding_polygon(mut points: Vec<C>, clip_polygon: &Polygon<C>) -> Option<Self> {
        let mut helper_points = helper_points(&clip_polygon);
        let num_helper_points = helper_points.len();
        points.append(&mut helper_points);

        VoronoiDiagram::with_helper_points(points, clip_polygon, num_helper_points)
    }

    fn with_helper_points(
        points: Vec<C>,
        clip_polygon: &Polygon<C>,
        num_helper_points: usize,
    ) -> Option<Self> {
        let delaunay = triangulate(&points)?;
        let centers = calculate_circumcenters(&points, &delaunay);
        let cells = VoronoiDiagram::calculate_polygons(&points, &centers, &delaunay, &clip_polygon);
        let neighbors = calculate_neighbors(&points, &delaunay);

        Some(VoronoiDiagram {
            sites: points,
            delaunay,
            centers,
            cells,
            neighbors,
            num_helper_points,
        })
    }

    pub fn from_tuple(min: &(f64, f64), max: &(f64, f64), coords: &[(f64, f64)]) -> Option<Self> {
        let points: Vec<C> = coords.iter().map(|p| C::from_xy(p.0, p.1)).collect();

        let clip_points = vec![
            C::from_xy(min.0, min.1),
            C::from_xy(max.0, min.1),
            C::from_xy(max.0, max.1),
            C::from_xy(min.0, max.1),
        ];

        let clip_polygon = Polygon::from_points(clip_points);

        VoronoiDiagram::with_bounding_polygon(points, &clip_polygon)
    }

    pub fn cells(&self) -> &[Polygon<C>] {
        &self.cells[..self.cells.len() - self.num_helper_points]
    }

    fn calculate_polygons(
        points: &[C],
        centers: &[C],
        delaunay: &Triangulation,
        clip_polygon: &Polygon<C>,
    ) -> Vec<Polygon<C>> {
        points
            .iter()
            .enumerate()
            .map(|(t, _point)| {
                let incoming = delaunay.inedges[t];
                let edges = edges_around_point(incoming, delaunay);
                let triangles: Vec<usize> = edges.into_iter().map(triangle_of_edge).collect();
                let polygon: Vec<C> = triangles.into_iter().map(|t| centers[t].clone()).collect();

                let polygon = Polygon::from_points(polygon);
                let polygon = sutherland_hodgman(&polygon, &clip_polygon);

                polygon
            })
            .collect()
    }
}

fn calculate_circumcenters<C: Coord + Vector<C>>(points: &[C], delaunay: &Triangulation) -> Vec<C> {
    let mut ret = Vec::new();

    for t in 0..delaunay.len() {
        let v: Vec<C> = points_of_triangle(t, delaunay)
            .into_iter()
            .map(|p| points[p].clone())
            .collect();

        ret.push(match circumcenter(&v[0], &v[1], &v[2]) {
            Some(c) => c,
            None => C::from_xy(0., 0.),
        });
    }

    ret
}

fn calculate_neighbors<C: Coord + Vector<C>>(
    points: &[C],
    delaunay: &Triangulation,
) -> Vec<Vec<usize>> {
    points
        .iter()
        .enumerate()
        .map(|(t, _point)| {
            let mut neighbours: Vec<usize> = vec![];

            let e0 = delaunay.inedges[t];
            if e0 != INVALID_INDEX {
                let mut e = e0;
                loop {
                    neighbours.push(delaunay.triangles[e]);
                    e = next_halfedge(e);
                    if delaunay.triangles[e] != t {
                        break;
                    }
                    e = delaunay.halfedges[e];
                    if e == INVALID_INDEX {
                        neighbours.push(delaunay.triangles[delaunay.outedges[t]]);
                        break;
                    }
                    if e == e0 {
                        break;
                    }
                }
            }

            neighbours
        })
        .collect()
}

fn is_inside(point: &Point, polygon: &Vec<Point>) -> bool {
    let mut crosses = 0;

    for i in 0..polygon.len() {
        let j = (i + 1) % polygon.len();

        if (polygon[i].y > point.y) != (polygon[j].y > point.y) {
            let at_x = (polygon[j].x - polygon[i].x) * (point.y - polygon[i].y)
                / (polygon[j].y - polygon[i].y)
                + polygon[i].x;

            if point.x < at_x {
                crosses += 1;
            }
        }
    }

    crosses % 2 == 1
}

fn calculate_area(points: &Vec<Point>) -> f64 {
    let mut area = 0.0;

    for i in 1..points.len() - 1 {
        area += Point::ccw(&points[0], &points[i], &points[i + 1]) / 2.0;
    }

    area.abs()
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let (n, border) = (scan.token::<usize>(), scan.token::<i64>() as f64);
    let mut data = Vec::with_capacity(n);
    let mut points = Vec::with_capacity(n);

    for _ in 0..n {
        let (mut x, mut y, mark) = (
            scan.token::<f64>(),
            scan.token::<f64>(),
            scan.token::<i64>(),
        );

        if x == 0.0 {
            x += 0.0001;
        }

        if y == 0.0 {
            y += 0.0001;
        }

        if x == border {
            x -= 0.0001;
        }

        if y == border {
            y -= 0.0001;
        }

        data.push((Point { x, y }, mark));
        points.push(Point { x, y });
    }

    if n == 1 {
        if data[0].1 == 0 {
            writeln!(out, "{:.1} 0.0", border * border).unwrap();
        } else {
            writeln!(out, "0.0 {:.1}", border * border).unwrap();
        }

        return;
    }

    let diagram = VoronoiDiagram::new(
        &Point { x: 0.0, y: 0.0 },
        &Point {
            x: border,
            y: border,
        },
        &points,
    )
    .unwrap();

    let cells = diagram.cells();
    let mut area_player = 0.0;
    let mut area_opponent = 0.0;

    for cell in cells {
        let mut mark_to_add = -1;

        for (point, mark) in data.iter() {
            if is_inside(point, &cell.points) {
                mark_to_add = *mark;
                break;
            }
        }

        if mark_to_add == 0 {
            area_player += calculate_area(&cell.points);
        } else if mark_to_add == 1 {
            area_opponent += calculate_area(&cell.points);
        }
    }

    writeln!(out, "{:.1} {:.1}", area_player, area_opponent).unwrap();
}
