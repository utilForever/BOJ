use io::Write;
use std::{
    cmp::Ordering,
    collections::{hash_map::RandomState, VecDeque},
    hash::{BuildHasher, Hasher},
    io,
    iter::repeat_with,
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

#[derive(Clone, Debug)]
pub struct Xorshift {
    y: u64,
}

impl Xorshift {
    pub fn new_with_seed(seed: u64) -> Self {
        Xorshift { y: seed }
    }

    pub fn new() -> Self {
        Xorshift::new_with_seed(RandomState::new().build_hasher().finish())
    }

    pub fn rand64(&mut self) -> u64 {
        self.y ^= self.y << 5;
        self.y ^= self.y >> 17;
        self.y ^= self.y << 11;
        self.y
    }

    pub fn rand(&mut self, k: u64) -> u64 {
        self.rand64() % k
    }

    pub fn rands(&mut self, k: u64, n: usize) -> Vec<u64> {
        repeat_with(|| self.rand(k)).take(n).collect()
    }

    pub fn randf(&mut self) -> f64 {
        const UPPER_MASK: u64 = 0x3FF0_0000_0000_0000;
        const LOWER_MASK: u64 = 0x000F_FFFF_FFFF_FFFF;
        let x = self.rand64();
        let tmp = UPPER_MASK | (x & LOWER_MASK);
        let result: f64 = f64::from_bits(tmp);
        f64::from_bits(f64::to_bits(result - 1.0) ^ (x >> 63))
    }

    pub fn gen_bool(&mut self, p: f64) -> bool {
        self.randf() < p
    }

    pub fn shuffle<T>(&mut self, slice: &mut [T]) {
        let mut n = slice.len();
        while n > 1 {
            let i = self.rand(n as _) as usize;
            n -= 1;
            slice.swap(i, n);
        }
    }
}

impl Default for Xorshift {
    fn default() -> Self {
        Xorshift::new_with_seed(0x2b99_2ddf_a232_49d6)
    }
}

const EPS: f64 = 1e-5;

fn gcd(first: i128, second: i128) -> i128 {
    let mut max = first;
    let mut min = second;

    if min == 0 && max == 0 {
        return 0;
    } else if min == 0 {
        return max;
    } else if max == 0 {
        return min;
    }

    if min > max {
        std::mem::swap(&mut min, &mut max);
    }

    loop {
        let res = max % min;

        if res == 0 {
            return min;
        }

        max = min;
        min = res;
    }
}

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

#[derive(Debug, Default, Copy, Clone)]
pub struct Point {
    pub x: f64,
    pub y: f64,
    pub idx: usize,
}

impl Point {
    #[inline(always)]
    pub fn new(x: f64, y: f64, idx: usize) -> Self {
        Self { x, y, idx }
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
    pub fn dist(&self, other: &Point) -> f64 {
        let dx = self.x - other.x;
        let dy = self.y - other.y;

        (dx * dx + dy * dy).sqrt()
    }

    #[inline(always)]
    pub fn rotate90(&self) -> Point {
        Point::new(-self.y, self.x, 0)
    }

    #[inline(always)]
    fn is_upper_half(self) -> bool {
        sign(self.y) > 0 || (sign(self.y) == 0 && sign(self.x) >= 0)
    }

    fn cmp_polar_direction(self, other: &Point) -> Ordering {
        let half_a = self.is_upper_half();
        let half_b = other.is_upper_half();

        if half_a != half_b {
            return if half_a {
                Ordering::Less
            } else {
                Ordering::Greater
            };
        }

        match sign(self.cross(other)) {
            1 => Ordering::Less,
            -1 => Ordering::Greater,
            _ => Ordering::Equal,
        }
    }
}

impl Add for Point {
    type Output = Point;
    fn add(self, other: Point) -> Point {
        Point::new(self.x + other.x, self.y + other.y, 0)
    }
}

impl Sub for Point {
    type Output = Point;
    fn sub(self, other: Point) -> Point {
        Point::new(self.x - other.x, self.y - other.y, 0)
    }
}

impl Mul<f64> for Point {
    type Output = Point;

    fn mul(self, rhs: f64) -> Self::Output {
        Point::new(self.x * rhs, self.y * rhs, 0)
    }
}

impl Div<f64> for Point {
    type Output = Point;

    fn div(self, rhs: f64) -> Self::Output {
        Point::new(self.x / rhs, self.y / rhs, 0)
    }
}

impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        sign(self.x - other.x) == 0 && sign(self.y - other.y) == 0
    }
}

impl PartialOrd for Point {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if (self.x - other.x).abs() < EPS {
            Some(self.y.partial_cmp(&other.y).unwrap())
        } else {
            Some(self.x.partial_cmp(&other.x).unwrap())
        }
    }
}

#[derive(Debug, Clone)]
struct Line {
    a: Point,
    b: Point,
    idx: usize,
}

impl Line {
    #[inline(always)]
    fn new(a: Point, b: Point, idx: usize) -> Self {
        Self { a, b, idx }
    }

    #[inline(always)]
    fn direction(&self) -> Point {
        self.b - self.a
    }

    #[inline(always)]
    fn contain_left(&self, p: &Point) -> bool {
        sign(self.direction().cross(&(*p - self.a))) > 0
    }

    #[inline(always)]
    fn side_sign(&self, p: Point) -> i64 {
        sign(self.direction().cross(&(p - self.a)))
    }

    #[inline(always)]
    fn side_sign_of_intersection(&self, a: &Line, b: &Line) -> i64 {
        self.side_sign(a.intersection_point(b))
    }

    #[inline(always)]
    fn is_parallel_to(&self, other: &Line) -> bool {
        sign(self.direction().cross(&other.direction())) == 0
    }

    #[inline(always)]
    fn contain_intersection_of(&self, a: &Line, b: &Line) -> bool {
        self.contain_left(&(a.intersection_point(b)))
    }

    #[inline(always)]
    fn same_direction_as(&self, other: &Line) -> bool {
        self.is_parallel_to(other) && self.direction().dot(&other.direction()) > 0.0
    }

    #[inline(always)]
    fn cmp_by_direction(&self, other: &Line) -> Ordering {
        if self.same_direction_as(other) {
            return if other.contain_left(&self.a) {
                Ordering::Less
            } else {
                Ordering::Greater
            };
        }

        self.direction().cmp_polar_direction(&other.direction())
    }

    #[inline(always)]
    fn intersection_point(&self, other: &Line) -> Point {
        let s1 = other.a.cross2(&other.b, &self.a);
        let s2 = -(other.a.cross2(&other.b, &self.b));

        ((self.a * s2) + (self.b * s1)) / (s1 + s2) as f64
    }

    #[inline(always)]
    fn perpendicular_bisector(a: &Point, b: &Point) -> Line {
        let mid = (*a + *b) * 0.5;
        let dir = (*b - *a).rotate90();

        Line::new(mid, mid + dir, b.idx)
    }

    #[inline(always)]
    fn rational_intersection(&self, other: &Line) -> (i128, i128, i128) {
        #[derive(Clone, Copy)]
        struct PointInt {
            x: i128,
            y: i128,
        }

        fn to_point_int(p: Point) -> PointInt {
            PointInt {
                x: (p.x * 2.0).round() as i128,
                y: (p.y * 2.0).round() as i128,
            }
        }

        fn cross(a: PointInt, b: PointInt, c: PointInt) -> i128 {
            let ux = b.x - a.x;
            let uy = b.y - a.y;
            let vx = c.x - a.x;
            let vy = c.y - a.y;

            (ux as i128) * (vy as i128) - (uy as i128) * (vx as i128)
        }

        let a1 = to_point_int(self.a);
        let a2 = to_point_int(self.b);
        let b1 = to_point_int(other.a);
        let b2 = to_point_int(other.b);

        let s1 = cross(b1, b2, a1);
        let s2 = -cross(b1, b2, a2);

        let mut z = (s1 + s2) * 2;
        let mut x = a1.x * s2 + a2.x * s1;
        let mut y = a1.y * s2 + a2.y * s1;

        if z < 0 {
            z = -z;
            x = -x;
            y = -y;
        }

        let g = gcd(gcd(x.abs(), y.abs()), z.abs());
        (x / g, y / g, z / g)
    }
}

struct ConvexHull {
    points: Vec<Point>,
    hull: Vec<Point>,
}

impl ConvexHull {
    fn new(points: Vec<Point>) -> Self {
        Self {
            points,
            hull: Vec::new(),
        }
    }

    fn build(&mut self) {
        let mut points = self.points.clone();

        points.sort_unstable_by(|p, q| {
            if (p.x - q.x).abs() < EPS {
                p.y.partial_cmp(&q.y).unwrap()
            } else {
                p.x.partial_cmp(&q.x).unwrap()
            }
        });

        let mut upper: Vec<Point> = Vec::new();
        let mut lower: Vec<Point> = Vec::new();

        for &point in points.iter() {
            while upper.len() >= 2
                && sign(upper[upper.len() - 2].cross2(&upper[upper.len() - 1], &point)) <= 0
            {
                upper.pop();
            }

            upper.push(point);
        }

        for &point in points.iter().rev() {
            while lower.len() >= 2
                && sign(lower[lower.len() - 2].cross2(&lower[lower.len() - 1], &point)) <= 0
            {
                lower.pop();
            }

            lower.push(point);
        }

        upper.pop();
        lower.pop();

        self.hull = upper.into_iter().chain(lower.into_iter()).collect();
    }

    fn hull(&self) -> &Vec<Point> {
        &self.hull
    }
}

struct LineFunc {
    a: i128,
    b: i128,
    c: i128,
}

impl LineFunc {
    fn new(a: i128, b: i128, c: i128) -> Self {
        Self { a, b, c }
    }
}

fn process_line(points: &Vec<Point>) -> (f64, LineFunc) {
    let n = points.len();

    if n == 1 {
        return (
            0.0,
            LineFunc::new(
                -(points[0].y.round() as i128),
                points[0].x.round() as i128,
                0,
            ),
        );
    }

    let mut convex_hull = ConvexHull::new(points.clone());
    convex_hull.build();

    let hull = convex_hull.hull();
    let m = hull.len();

    if m == 2 {
        let (u, v) = (hull[0], hull[1]);
        let a = (v.y - u.y).round() as i128;
        let b = (u.x - v.x).round() as i128;
        let c = ((a as f64) * u.x + (b as f64) * u.y).round() as i128;

        return (0.0, LineFunc::new(a, b, c));
    }

    let mut idx = 1;
    let mut best_radius = f64::MAX;
    let mut best_line = LineFunc::new(0, 0, 0);

    for i in 0..m {
        let (u, v) = (hull[i], hull[(i + 1) % m]);

        while u.cross2(&v, &hull[idx]) < u.cross2(&v, &hull[(idx + 1) % m]) {
            idx = (idx + 1) % m;
        }

        let radius = u.cross2(&v, &hull[idx]).abs() / u.dist(&v) * 0.5;

        if radius + EPS < best_radius {
            best_radius = radius;

            let (dx, dy) = (v.x - u.x, v.y - u.y);
            let (a, b) = (dy.round() as i128, (-dx).round() as i128);
            let normal = Point::new(a as f64, b as f64, 0);
            let center = (normal.dot(&u) + normal.dot(&hull[idx])) * 0.5;

            best_line = if (center - center.round()).abs() < EPS {
                LineFunc::new(a, b, center.round() as i128)
            } else {
                LineFunc::new(a * 2, b * 2, (center * 2.0).round() as i128)
            }
        }
    }

    (best_radius, best_line)
}

fn voronoi_cut(poly: &Vec<Line>, line: Line) -> Vec<Line> {
    if poly.is_empty() {
        return Vec::new();
    }

    let n = poly.len();
    let mut ret = Vec::new();

    for i in 0..n {
        let a1 = &poly[i];
        let a2 = &poly[(i + 1) % n];
        let a3 = &poly[(i + 2) % n];

        let d1 = line.side_sign_of_intersection(a1, a2);
        let d2 = line.side_sign_of_intersection(a2, a3);

        if d1 > 0 || d2 > 0 || (d1 == 0 && d2 == 0) {
            ret.push(a2.clone());
        }

        if d1 >= 0 && d2 < 0 {
            ret.push(line.clone());
        }
    }

    ret
}

struct VoronoiDiagram {
    points: Vec<Point>,
    boundary: f64,
}

impl VoronoiDiagram {
    fn new(points: &Vec<Point>, boundary: f64) -> Self {
        Self {
            points: points.clone(),
            boundary,
        }
    }

    fn bbox(&self) -> Vec<Line> {
        let b = self.boundary;

        vec![
            Line::new(Point::new(-b, -b, 0), Point::new(b, -b, 0), 0),
            Line::new(Point::new(b, -b, 0), Point::new(b, b, 0), 0),
            Line::new(Point::new(b, b, 0), Point::new(-b, b, 0), 0),
            Line::new(Point::new(-b, b, 0), Point::new(-b, -b, 0), 0),
        ]
    }

    fn build(&self) -> Vec<Vec<Line>> {
        let n = self.points.len();
        let mut cells = vec![Vec::new(); n];

        let mut shuffled = self.points.clone();
        let mut rng = Xorshift::default();
        rng.shuffle(&mut shuffled);

        for i in 0..n {
            let mut cell = self.bbox();

            for &point in shuffled.iter() {
                if point.dist(&self.points[i]) > EPS {
                    let mut bisector = Line::perpendicular_bisector(&self.points[i], &point);
                    bisector.idx = point.idx;

                    cell = voronoi_cut(&cell, bisector);

                    if cell.is_empty() {
                        break;
                    }
                }
            }

            let mut sorted = cell;
            sorted.sort_unstable_by(|l1, l2| l1.cmp_by_direction(l2));

            cells[i] = sorted;
        }

        cells
    }
}

struct FarthestVoronoiDiagram {
    points: Vec<Point>,
    hull: Vec<Point>,
    boundary: f64,
}

impl FarthestVoronoiDiagram {
    fn new(points: &Vec<Point>, hull: &Vec<Point>, boundary: f64) -> Self {
        Self {
            points: points.clone(),
            hull: hull.clone(),
            boundary,
        }
    }

    fn bbox(&self) -> Vec<Line> {
        let b = self.boundary;

        vec![
            Line::new(Point::new(-b, -b, 0), Point::new(b, -b, 0), 0),
            Line::new(Point::new(b, -b, 0), Point::new(b, b, 0), 0),
            Line::new(Point::new(b, b, 0), Point::new(-b, b, 0), 0),
            Line::new(Point::new(-b, b, 0), Point::new(-b, -b, 0), 0),
        ]
    }

    fn build(&self) -> Vec<Vec<Line>> {
        let n = self.points.len();
        let mut is_hull = vec![false; n];

        for &point in self.hull.iter() {
            is_hull[point.idx] = true;
        }

        let mut cells = vec![Vec::new(); n];

        let mut shuffled = self.points.clone();
        let mut rng = Xorshift::default();
        rng.shuffle(&mut shuffled);

        for i in 0..n {
            if !is_hull[i] {
                continue;
            }

            let mut cell = self.bbox();

            for &point in shuffled.iter() {
                if point.dist(&self.points[i]) > EPS {
                    let mut bisector = Line::perpendicular_bisector(&point, &self.points[i]);
                    bisector.idx = point.idx;

                    cell = voronoi_cut(&cell, bisector);

                    if cell.is_empty() {
                        break;
                    }
                }
            }

            let mut sorted = cell;
            sorted.sort_unstable_by(|l1, l2| l1.cmp_by_direction(l2));

            cells[i] = sorted;
        }

        cells
    }
}

struct HalfPlaneIntersection {
    lines: Vec<Line>,
}

impl HalfPlaneIntersection {
    fn new(lines: Vec<Line>) -> Self {
        Self { lines }
    }

    fn build(&self) -> Vec<Line> {
        let mut queue = VecDeque::new();

        for (idx, line) in self.lines.iter().enumerate() {
            if idx > 0 && line.same_direction_as(&self.lines[idx - 1]) {
                continue;
            };

            while queue.len() >= 2
                && !line.contain_intersection_of(&queue[queue.len() - 2], &queue[queue.len() - 1])
            {
                queue.pop_back();
            }

            while queue.len() >= 2 && !line.contain_intersection_of(&queue[1], &queue[0]) {
                queue.pop_front();
            }

            queue.push_back(line.clone());
        }

        while queue.len() > 2
            && !queue[0].contain_intersection_of(&queue[queue.len() - 2], &queue[queue.len() - 1])
        {
            queue.pop_back();
        }

        while queue.len() > 2
            && !queue[queue.len() - 1].contain_intersection_of(&queue[1], &queue[0])
        {
            queue.pop_front();
        }

        queue.into_iter().collect()
    }
}

// Reference: Petrozavodsk Summer 2023. Day 6. olmrgcsi And His Friends' Contest Editorial
// Thanks for Crysfly to refer the some part of the code and to fix some mistakes.
fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let n = scan.token::<usize>();
    let mut points = Vec::with_capacity(n);

    for i in 0..n {
        let (x, y) = (scan.token::<f64>(), scan.token::<f64>());
        points.push(Point::new(x, y, i));
    }

    let (mut best_radius, best_line) = process_line(&points);
    let mut is_line = true;

    let mut convex_hull = ConvexHull::new(points.clone());
    convex_hull.build();

    let hull = convex_hull.hull();

    let voronoi = VoronoiDiagram::new(&points, 1e9).build();
    let farthest_voronoi = FarthestVoronoiDiagram::new(&points, &hull, 1e9).build();

    let mut best_circle_radius = f64::MAX;
    let mut best_center = (0, 0, 1);

    for i in 0..n {
        if farthest_voronoi[i].is_empty() {
            continue;
        }

        for j in 0..n {
            if voronoi[j].is_empty() {
                continue;
            }

            let mut lines_merged = Vec::with_capacity(farthest_voronoi[i].len() + voronoi[j].len());
            let (mut idx1, mut idx2) = (0, 0);

            while idx1 < farthest_voronoi[i].len() && idx2 < voronoi[j].len() {
                if farthest_voronoi[i][idx1].cmp_by_direction(&voronoi[j][idx2])
                    != Ordering::Greater
                {
                    lines_merged.push(farthest_voronoi[i][idx1].clone());
                    idx1 += 1;
                } else {
                    lines_merged.push(voronoi[j][idx2].clone());
                    idx2 += 1;
                }
            }

            while idx1 < farthest_voronoi[i].len() {
                lines_merged.push(farthest_voronoi[i][idx1].clone());
                idx1 += 1;
            }

            while idx2 < voronoi[j].len() {
                lines_merged.push(voronoi[j][idx2].clone());
                idx2 += 1;
            }

            let hpi = HalfPlaneIntersection::new(lines_merged).build();

            if hpi.len() < 3 {
                continue;
            }

            let m = hpi.len();

            for k in 0..m {
                let point = hpi[k].intersection_point(&hpi[(k + 1) % m]);
                let dist_max = point.dist(&points[i]);
                let dist_min = point.dist(&points[j]);
                let radius = (dist_max - dist_min).abs() * 0.5;

                if radius + EPS < best_radius {
                    is_line = false;
                    best_radius = radius;
                    best_circle_radius = (dist_min + dist_max) * 0.5;
                    best_center = hpi[k].rational_intersection(&hpi[(k + 1) % m]);
                }
            }
        }
    }

    let (mut x, mut y, mut z) = best_center;

    if z < 0 {
        x = -x;
        y = -y;
        z = -z;
    }

    let g = gcd(gcd(x.abs(), y.abs()), z.abs());
    (x, y, z) = (x / g, y / g, z / g);

    writeln!(out, "{:.12}", best_radius).unwrap();

    if is_line {
        writeln!(out, "L {} {} {}", best_line.a, best_line.b, best_line.c).unwrap();
    } else {
        writeln!(out, "C {x} {y} {z} {:.12}", best_circle_radius).unwrap();
    }
}
