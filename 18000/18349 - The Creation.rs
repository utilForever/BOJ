use io::Write;
use std::{
    cmp::Ordering,
    collections::{BTreeMap, BinaryHeap, HashMap},
    io,
    ptr::null_mut,
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

/// Delaunay Triangulation and Voronoi Diagram implementation
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

impl std::fmt::Debug for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
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

/// KD-tree implemenation
#[derive(Clone, Copy)]
struct KDPoint {
    x: i64,
    y: i64,
    id: usize,
}

impl KDPoint {
    pub(crate) fn new(x: i64, y: i64, id: usize) -> Self {
        Self { x, y, id }
    }
}

#[derive(Clone, Copy)]
struct BBox {
    min: KDPoint,
    max: KDPoint,
}

impl BBox {
    fn new(min: KDPoint, max: KDPoint) -> Self {
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
}

#[derive(Clone)]
enum Dimension {
    X,
    Y,
}

pub struct KDTree {
    points: Vec<KDPoint>,
    bbox: Vec<BBox>,
    threshold: usize,
}

impl KDTree {
    pub(crate) fn new(points: Vec<KDPoint>, threshold: usize) -> Self {
        let n = points.len();
        let mut tree = KDTree {
            bbox: vec![BBox::new(KDPoint::new(0, 0, 0), KDPoint::new(0, 0, 0)); n],
            points,
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
            Dimension::X => a.x.cmp(&b.x).then(a.y.cmp(&b.y)).then(a.id.cmp(&b.id)),
            Dimension::Y => a.y.cmp(&b.y).then(a.x.cmp(&b.x)).then(a.id.cmp(&b.id)),
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
            let KDPoint {
                x: mut x_min,
                y: mut y_min,
                ..
            } = self.points[left];
            let (mut x_max, mut y_max) = (x_min, y_min);

            for &KDPoint { x, y, .. } in &self.points[left + 1..right] {
                x_min = x_min.min(x);
                x_max = x_max.max(x);
                y_min = y_min.min(y);
                y_max = y_max.max(y);
            }

            let bb = BBox::new(KDPoint::new(x_min, y_min, 0), KDPoint::new(x_max, y_max, 0));
            self.bbox[mid] = bb;
            return bb;
        }

        let bbox_left = self.fill_bbox(left, mid);
        let bbox_right = self.fill_bbox(mid + 1, right);
        let point_mid = self.points[mid];

        let merged = BBox::new(
            KDPoint::new(
                bbox_left.min.x.min(bbox_right.min.x).min(point_mid.x),
                bbox_left.min.y.min(bbox_right.min.y).min(point_mid.y),
                0,
            ),
            KDPoint::new(
                bbox_left.max.x.max(bbox_right.max.x).max(point_mid.x),
                bbox_left.max.y.max(bbox_right.max.y).max(point_mid.y),
                0,
            ),
        );

        self.bbox[mid] = merged;
        merged
    }

    pub fn nearest(&self, x: i64, y: i64) -> usize {
        let mut best_dist: i64 = i64::MAX;
        let mut best_idx: usize = usize::MAX;

        self.nearest_internal(
            &Dimension::X,
            &mut best_dist,
            &mut best_idx,
            0,
            self.points.len(),
            x,
            y,
        );

        best_idx
    }

    fn nearest_internal(
        &self,
        dim: &Dimension,
        best_dist: &mut i64,
        best_idx: &mut usize,
        left: usize,
        right: usize,
        x: i64,
        y: i64,
    ) {
        if left >= right {
            return;
        }

        let mid = (left + right) / 2;

        if self.bbox[mid].min_dist2(x, y) > *best_dist {
            return;
        }

        if right - left <= self.threshold {
            for i in left..right {
                let dx = self.points[i].x - x;
                let dy = self.points[i].y - y;
                let dist2 = dx * dx + dy * dy;

                if dist2 < *best_dist || (dist2 == *best_dist && self.points[i].id < *best_idx) {
                    *best_dist = dist2;
                    *best_idx = self.points[i].id;
                }
            }

            return;
        }

        {
            let dx = self.points[mid].x - x;
            let dy = self.points[mid].y - y;
            let dist2 = dx * dx + dy * dy;

            if dist2 < *best_dist || (dist2 == *best_dist && self.points[mid].id < *best_idx) {
                *best_dist = dist2;
                *best_idx = self.points[mid].id;
            }
        }

        let next = match dim {
            Dimension::X => Dimension::Y,
            Dimension::Y => Dimension::X,
        };

        let (val_query, val_pivot) = match dim {
            Dimension::X => (x, self.points[mid].x),
            Dimension::Y => (y, self.points[mid].y),
        };

        if val_query < val_pivot {
            self.nearest_internal(&next, best_dist, best_idx, left, mid, x, y);

            if mid + 1 < right {
                let mid2 = (mid + 1 + right) / 2;

                if self.bbox[mid2].min_dist2(x, y) <= *best_dist {
                    self.nearest_internal(&next, best_dist, best_idx, mid + 1, right, x, y);
                }
            }
        } else {
            if mid + 1 < right {
                self.nearest_internal(&next, best_dist, best_idx, mid + 1, right, x, y);
            }

            if left < mid {
                let mid2 = (left + mid) / 2;

                if self.bbox[mid2].min_dist2(x, y) <= *best_dist {
                    self.nearest_internal(&next, best_dist, best_idx, left, mid, x, y);
                }
            }
        }
    }
}

/// Link-Cut Tree implementation
#[derive(Debug)]
pub(crate) struct Node {
    left: *mut Node,
    right: *mut Node,
    parent: *mut Node,
    value: i64,
    max: i64,
    flip: bool,
}

impl Node {
    fn new(value: i64) -> *mut Self {
        Box::into_raw(Box::new(Self {
            left: null_mut(),
            right: null_mut(),
            parent: null_mut(),
            value,
            max: value,
            flip: false,
        }))
    }

    unsafe fn is_root(&self) -> bool {
        self.parent.is_null()
            || ((*self.parent).left != (self as *const _ as *mut _)
                && (*self.parent).right != (self as *const _ as *mut _))
    }

    unsafe fn is_left(&self) -> bool {
        !self.parent.is_null() && (*self.parent).left == (self as *const _ as *mut _)
    }

    unsafe fn update(&mut self) {
        let mut max = self.value;

        if !self.left.is_null() {
            max = max.max((*self.left).max);
        }

        if !self.right.is_null() {
            max = max.max((*self.right).max);
        }

        self.max = max;
    }

    unsafe fn push(&mut self) {
        if self.flip {
            let tmp = self.left;

            self.left = self.right;
            self.right = tmp;

            if !self.left.is_null() {
                (*self.left).flip ^= true;
            }

            if !self.right.is_null() {
                (*self.right).flip ^= true;
            }

            self.flip = false;
        }
    }

    unsafe fn rotate(&mut self) {
        let x = self as *mut Node;
        let p = (*x).parent;
        let g = (*p).parent;

        (*p).push();
        (*x).push();

        if (*x).is_left() {
            let b = (*x).right;

            (*p).left = b;

            if !b.is_null() {
                (*b).parent = p;
            }

            (*x).right = p;
            (*p).parent = x;
        } else {
            let b = (*x).left;

            (*p).right = b;

            if !b.is_null() {
                (*b).parent = p;
            }

            (*x).left = p;
            (*p).parent = x;
        }

        (*x).parent = g;

        if !g.is_null() {
            if (*g).left == p {
                (*g).left = x;
            } else if (*g).right == p {
                (*g).right = x;
            }
        }

        (*p).update();
        (*x).update();
    }
}

pub(crate) struct LinkCutTree {
    pub(crate) nodes: Vec<*mut Node>,
}

impl LinkCutTree {
    pub(crate) unsafe fn new(n: usize) -> Self {
        let mut nodes = vec![null_mut(); n + 1];

        for i in 1..=n {
            nodes[i] = Node::new(0);
        }

        Self { nodes }
    }

    pub(crate) unsafe fn push_to_root(&mut self, x: *mut Node) {
        let mut stack = Vec::new();
        let mut v = x;

        stack.push(v);

        while !(*v).is_root() {
            v = (*v).parent;
            stack.push(v);
        }

        while let Some(t) = stack.pop() {
            (*t).push();
        }
    }

    pub(crate) unsafe fn splay(&mut self, x: *mut Node) {
        self.push_to_root(x);

        while !(*x).is_root() {
            let p = (*x).parent;

            if !(*p).is_root() {
                let g = (*p).parent;
                let zigzig = ((*p).left == x) == ((*g).left == p);

                if zigzig {
                    (*p).rotate();
                } else {
                    (*x).rotate();
                }
            }

            (*x).rotate();
        }

        (*x).update();
    }

    pub(crate) unsafe fn access(&mut self, x: *mut Node) {
        let mut v = x;
        let mut last = null_mut();

        while !v.is_null() {
            self.splay(v);
            (*v).right = last;

            if !last.is_null() {
                (*last).parent = v;
            }

            (*v).update();
            last = v;
            v = (*v).parent;
        }

        self.splay(x);
    }

    pub(crate) unsafe fn make_root(&mut self, x: *mut Node) {
        self.access(x);
        (*x).flip ^= true;
    }

    pub(crate) unsafe fn link(&mut self, x: *mut Node, y: *mut Node) {
        self.make_root(x);
        self.access(y);
        (*x).parent = y;
    }

    pub(crate) unsafe fn cut(&mut self, x: *mut Node, y: *mut Node) {
        self.make_root(x);
        self.access(y);

        if (*y).left == x {
            (*y).left = null_mut();
            (*x).parent = null_mut();
            (*y).update();
        }
    }

    pub(crate) unsafe fn update_value(&mut self, x: *mut Node, value: i64) {
        self.access(x);
        (*x).value = value;
        (*x).update();
    }

    pub(crate) unsafe fn query_max(&mut self, x: *mut Node, y: *mut Node) -> i64 {
        self.make_root(x);
        self.access(y);
        (*y).max
    }
}

/// Suffix Automaton implementation
#[derive(Default, Clone)]
pub struct SAState {
    pub len: usize,
    pub link: i32,
    pub next: BTreeMap<char, usize>,
}

pub struct SuffixAutomaton {
    pub st: Vec<SAState>,
    pub sz: usize,
    pub last: usize,
}

impl SuffixAutomaton {
    pub fn from_str(s: &str) -> Self {
        let mut sa = Self::new(s.len());

        for ch in s.chars() {
            sa.add(ch);
        }

        sa
    }

    pub fn new(n: usize) -> Self {
        let mut sa = Self {
            st: vec![],
            sz: 1,
            last: 0,
        };

        for _ in 0..(2 * n) {
            sa.st.push(SAState::default());
        }

        sa.st[0].len = 0;
        sa.st[0].link = -1;

        sa
    }

    pub fn add(&mut self, c: char) {
        let cur = self.sz;

        self.sz += 1;
        self.st[cur].len = self.st[self.last].len + 1;

        let mut p = self.last as i32;

        while p != -1 && !self.st[p as usize].next.contains_key(&c) {
            self.st[p as usize].next.insert(c, cur);
            p = self.st[p as usize].link;
        }

        if p == -1 {
            self.st[cur].link = 0;
        } else {
            let pu = p as usize;
            let q = self.st[pu].next[&c];

            if self.st[pu].len + 1 == self.st[q].len {
                self.st[cur].link = q as i32;
            } else {
                let clone = self.sz;

                self.sz += 1;
                self.st[clone].len = self.st[pu].len + 1;
                self.st[clone].next = self.st[q].next.clone();
                self.st[clone].link = self.st[q].link;

                while p != -1 && *self.st[p as usize].next.get(&c).unwrap() == q {
                    self.st[p as usize].next.insert(c, clone);
                    p = self.st[p as usize].link;
                }

                self.st[cur].link = clone as i32;
                self.st[q].link = self.st[cur].link;
            }
        }

        self.last = cur;
    }

    pub fn lcs_len(&self, t: &Vec<char>) -> usize {
        let mut state = 0;
        let mut len = 0;
        let mut ret = 0;

        for &c in t {
            while state != 0 && !self.st[state].next.contains_key(&c) {
                state = self.st[state].link as usize;
                len = self.st[state].len;
            }

            if let Some(&to) = self.st[state].next.get(&c) {
                state = to;
                len += 1;
            } else {
                state = 0;
                len = 0;
            }

            ret = ret.max(len);
        }

        ret
    }
}

/// MST (Prim) implementation
#[derive(Clone, Copy, Eq, PartialEq)]
struct PrimCandidate {
    weight: i64,
    parent_star: usize,
    parent_relic: usize,
    child: usize,
}

impl Ord for PrimCandidate {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.weight.cmp(&other.weight) {
            Ordering::Equal => match self.parent_star.cmp(&other.parent_star) {
                Ordering::Equal => other.child.cmp(&self.child),
                ordering => ordering,
            },
            ordering => ordering,
        }
    }
}

impl PartialOrd for PrimCandidate {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

struct MST {
    a_star: Vec<usize>,
    b_relic: Vec<usize>,
    parent_star: Vec<usize>,
    strength_c: Vec<i64>,
}

impl MST {
    fn build(n: usize, edges: &Vec<Vec<(usize, i64)>>) -> Self {
        let mut in_tree = vec![false; n + 1];
        let mut best_weight = vec![-1; n + 1];
        let mut best_parent_star = vec![0; n + 1];
        let mut best_parent_relic = vec![0; n + 1];

        let mut a_star = vec![0; n + 1];
        let mut b_relic = vec![0; n + 1];
        let mut parent_star = vec![0; n + 1];
        let mut strength_c = vec![0; n + 1];

        a_star[1] = 1;
        b_relic[1] = 1;
        in_tree[1] = true;

        let mut priority_queue = BinaryHeap::new();

        for &(to, weight) in edges[1].iter() {
            if in_tree[to] {
                continue;
            }

            best_weight[to] = weight;
            best_parent_star[to] = 1;
            best_parent_relic[to] = 1;

            priority_queue.push(PrimCandidate {
                weight,
                parent_star: 1,
                child: to,
                parent_relic: 1,
            });
        }

        for node in 2..=n {
            let candidate = loop {
                let top = priority_queue
                    .pop()
                    .expect("Delaunay graph should be connected");

                if in_tree[top.child] {
                    continue;
                }

                if top.weight != best_weight[top.child]
                    || top.parent_star != best_parent_star[top.child]
                    || top.parent_relic != best_parent_relic[top.child]
                {
                    continue;
                }

                break top;
            };

            let child = candidate.child;

            a_star[node] = child;
            b_relic[child] = node;
            parent_star[node] = candidate.parent_star;
            strength_c[node] = candidate.weight;
            in_tree[child] = true;

            for &(to, weight) in edges[child].iter() {
                if in_tree[to] {
                    continue;
                }

                if weight > best_weight[to]
                    || (weight == best_weight[to] && node > best_parent_star[to])
                {
                    best_weight[to] = weight;
                    best_parent_star[to] = node;
                    best_parent_relic[to] = child;

                    priority_queue.push(PrimCandidate {
                        weight,
                        parent_star: node,
                        child: to,
                        parent_relic: child,
                    });
                }
            }
        }

        Self {
            a_star,
            b_relic,
            parent_star,
            strength_c,
        }
    }

    #[inline]
    fn relic_of_star(&self, star: usize) -> usize {
        self.a_star[star]
    }

    #[inline]
    fn star_of_relic(&self, relic: usize) -> usize {
        self.b_relic[relic]
    }

    fn write_initial_edges(&self, out: &mut String) {
        let n = self.a_star.len() - 1;

        for u in 2..=n {
            out.push_str(&format!(
                "{} {} {}\n",
                self.a_star[u], self.parent_star[u], self.strength_c[u]
            ));
        }
    }
}

fn pair_key(a: usize, b: usize) -> u64 {
    let (x, y) = if a < b { (a, b) } else { (b, a) };
    ((x as u64) << 32) | (y as u64)
}

fn get_lcs_cached(
    suffix_automatons: &Vec<SuffixAutomaton>,
    names: &Vec<String>,
    cache: &mut HashMap<u64, i64>,
    i: usize,
    j: usize,
) -> i64 {
    let key = pair_key(i, j);

    if let Some(&val) = cache.get(&key) {
        return val;
    }

    let val = if names[i].len() >= names[j].len() {
        let s = names[j].chars().collect::<Vec<_>>();
        suffix_automatons[i].lcs_len(&s) as i64
    } else {
        let s = names[i].chars().collect::<Vec<_>>();
        suffix_automatons[j].lcs_len(&s) as i64
    };

    cache.insert(key, val);
    val
}

fn build_adjacency_from_delaunay(
    points: &Vec<(i64, i64)>,
    suffix_automatons: &Vec<SuffixAutomaton>,
    names: &Vec<String>,
    cache: &mut HashMap<u64, i64>,
    n: usize,
) -> Vec<Vec<(usize, i64)>> {
    let edge_keys = delaunay_edge_keys(n, points);
    let mut edges = vec![Vec::new(); n + 1];

    for key in edge_keys {
        let u = (key >> 32) as usize;
        let v = (key & 0xFFFF_FFFF) as usize;
        let lcs = get_lcs_cached(suffix_automatons, names, cache, u, v);

        edges[u].push((v, lcs));
        edges[v].push((u, lcs));
    }

    edges
}

fn delaunay_edge_keys(n: usize, points: &Vec<(i64, i64)>) -> Vec<u64> {
    let mut points_delaunay = Vec::with_capacity(n);

    for i in 1..=n {
        points_delaunay.push(Point {
            x: points[i].0 as f64,
            y: points[i].1 as f64,
        });
    }

    if let Some(triangulation) = triangulate(&points_delaunay) {
        let mut edges = Vec::with_capacity(triangulation.triangles.len());

        for idx in 0..triangulation.triangles.len() {
            let a = triangulation.triangles[idx] + 1;
            let b = triangulation.triangles[next_halfedge(idx)] + 1;

            let (u, v) = if a < b { (a, b) } else { (b, a) };
            edges.push(pair_key(u, v));
        }

        edges.sort_unstable();
        edges.dedup();
        edges
    } else {
        let mut idxs = (1..=n).collect::<Vec<_>>();
        let (mut x_min, mut x_max, mut y_min, mut y_max) =
            (points[1].0, points[1].0, points[1].1, points[1].1);

        for i in 2..=n {
            x_min = x_min.min(points[i].0);
            x_max = x_max.max(points[i].0);
            y_min = y_min.min(points[i].1);
            y_max = y_max.max(points[i].1);
        }

        if x_max - x_min >= y_max - y_min {
            idxs.sort_unstable_by(|&a, &b| {
                points[a]
                    .0
                    .cmp(&points[b].0)
                    .then(points[a].1.cmp(&points[b].1))
                    .then(a.cmp(&b))
            });
        } else {
            idxs.sort_unstable_by(|&a, &b| {
                points[a]
                    .1
                    .cmp(&points[b].1)
                    .then(points[a].0.cmp(&points[b].0))
                    .then(a.cmp(&b))
            });
        }

        let mut edges = Vec::with_capacity(n - 1);

        for i in 0..n - 1 {
            let u = idxs[i];
            let v = idxs[i + 1];
            let (a, b) = if u < v { (u, v) } else { (v, u) };

            edges.push(pair_key(a, b));
        }

        edges
    }
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let (n, q1, q2) = (
        scan.token::<usize>(),
        scan.token::<usize>(),
        scan.token::<usize>(),
    );
    let mut points = vec![(0, 0); n + 1];
    let mut names = vec![String::new(); n + 1];

    for i in 1..=n {
        points[i] = (scan.token::<i64>(), scan.token::<i64>());
        names[i] = scan.token::<String>();
    }

    let mut suffix_automatons = Vec::with_capacity(n + 1);
    suffix_automatons.push(SuffixAutomaton::new(1));

    for i in 1..=n {
        suffix_automatons.push(SuffixAutomaton::from_str(&names[i]));
    }

    let mut cache = HashMap::new();
    let edges = build_adjacency_from_delaunay(&points, &suffix_automatons, &names, &mut cache, n);
    let mst = MST::build(n, &edges);

    let mut kd_points = Vec::with_capacity(n);

    for i in 1..=n {
        kd_points.push(KDPoint::new(points[i].0, points[i].1, i));
    }

    let kd_tree = KDTree::new(kd_points, 16);

    let mut ret = String::new();
    mst.write_initial_edges(&mut ret);

    let mut parent_curr = mst.parent_star.clone();

    unsafe {
        let mut lct = LinkCutTree::new(2 * n);

        for node in 2..=n {
            let parent = mst.parent_star[node];
            parent_curr[node] = parent;

            let node_fake = n + node;
            let relic_node = mst.relic_of_star(node);
            let relic_parent = mst.relic_of_star(parent);

            let dx = points[relic_node].0 - points[relic_parent].0;
            let dy = points[relic_node].1 - points[relic_parent].1;
            let dist2 = dx * dx + dy * dy;
            let w = dist2 * (mst.strength_c[node] as i64);

            lct.update_value(lct.nodes[node_fake], w);
            lct.link(lct.nodes[node], lct.nodes[node_fake]);
            lct.link(lct.nodes[node_fake], lct.nodes[parent]);
        }

        for _ in 0..q1 + q2 {
            let cmd = scan.token::<i32>();

            if cmd == 1 {
                let (xs, ys, xe, ye) = (
                    scan.token::<i64>(),
                    scan.token::<i64>(),
                    scan.token::<i64>(),
                    scan.token::<i64>(),
                );
                let relic_s = kd_tree.nearest(xs, ys);
                let relic_e = kd_tree.nearest(xe, ye);
                let star_s = mst.star_of_relic(relic_s);
                let star_e = mst.star_of_relic(relic_e);

                ret.push_str(&format!(
                    "{}\n",
                    lct.query_max(lct.nodes[star_s], lct.nodes[star_e])
                ));
            } else {
                let (u, p) = (scan.token::<usize>(), scan.token::<usize>());

                let old = parent_curr[u];
                let node_fake = n + u;

                lct.cut(lct.nodes[u], lct.nodes[node_fake]);
                lct.cut(lct.nodes[node_fake], lct.nodes[old]);

                let root_u = mst.relic_of_star(u);
                let root_p = mst.relic_of_star(p);
                let lcs = get_lcs_cached(&suffix_automatons, &names, &mut cache, root_u, root_p);

                let dx = points[root_u].0 - points[root_p].0;
                let dy = points[root_u].1 - points[root_p].1;
                let dist2 = dx * dx + dy * dy;
                let w = dist2 * lcs;

                lct.update_value(lct.nodes[node_fake], w);
                lct.link(lct.nodes[u], lct.nodes[node_fake]);
                lct.link(lct.nodes[node_fake], lct.nodes[p]);

                parent_curr[u] = p;
            }
        }
    }

    write!(out, "{ret}").unwrap();
}
