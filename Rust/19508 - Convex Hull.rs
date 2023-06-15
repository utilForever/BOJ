use io::Write;
use std::{io, ptr, str};

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

#[derive(Debug, Copy, Clone, PartialEq)]
struct Point3 {
    x: f64,
    y: f64,
    z: f64,
}

impl Point3 {
    fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }

    fn mul(&self, k: f64) -> Self {
        Self::new(self.x * k, self.y * k, self.z * k)
    }

    fn cross(&self, other: &Point3) -> Self {
        Self::new(
            self.y * other.z - self.z * other.y,
            self.z * other.x - self.x * other.z,
            self.x * other.y - self.y * other.x,
        )
    }

    fn dot(&self, other: &Point3) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    fn abs(&self) -> f64 {
        self.dot(self).sqrt()
    }

    fn normalize(&self) -> Self {
        let abs = self.abs();
        Self::new(self.x / abs, self.y / abs, self.z / abs)
    }
}

impl std::ops::Add for Point3 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}

impl std::ops::Sub for Point3 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}

#[derive(Debug, Clone)]
struct Edge {
    rev: *mut Edge,
    face: *mut Face,
}

#[derive(Debug, Clone)]
struct Face {
    a: usize,
    b: usize,
    c: usize,
    q: Point3,
    e1: *mut Edge,
    e2: *mut Edge,
    e3: *mut Edge,
    points: Vec<usize>,
    dead: usize,
}

impl Face {
    fn new(a: usize, b: usize, c: usize, q: Point3) -> Self {
        Self {
            a,
            b,
            c,
            q,
            e1: ptr::null_mut(),
            e2: ptr::null_mut(),
            e3: ptr::null_mut(),
            points: Vec::new(),
            dead: 1_000_000_000,
        }
    }
}

fn glue(f1: *mut Face, f2: *mut Face, e1: &mut *mut Edge, e2: &mut *mut Edge) {
    let edge1 = Box::new(Edge {
        rev: ptr::null_mut(),
        face: ptr::null_mut(),
    });
    let edge2 = Box::new(Edge {
        rev: ptr::null_mut(),
        face: ptr::null_mut(),
    });
    *e1 = Box::into_raw(edge1);
    *e2 = Box::into_raw(edge2);

    unsafe {
        (*(*e1)).rev = *e2;
        (*(*e2)).rev = *e1;
        (*(*e1)).face = f2;
        (*(*e2)).face = f1;
    }
}

fn prepare(p: &mut Vec<Point3>) -> bool {
    let n = p.len();
    let mut vec = Vec::new();
    vec.push(0);

    for i in 1..n {
        if vec.len() == 1 {
            if (p[vec[0]] - p[i]).abs() > 1e-9 {
                vec.push(i);
            }
        } else if vec.len() == 2 {
            if (p[vec[1]] - p[vec[0]]).cross(&(p[i] - p[vec[0]])).abs() > 1e-9 {
                vec.push(i);
            }
        } else if (p[i] - p[vec[0]])
            .dot(&(p[vec[1]] - p[vec[0]]).cross(&(p[vec[2]] - p[vec[0]])))
            .abs()
            > 1e-9
        {
            vec.push(i);
            break;
        }
    }

    if vec.len() != 4 {
        return false;
    }

    let mut vec2 = Vec::new();

    for i in vec.iter() {
        vec2.push(p[*i]);
    }

    vec.reverse();

    for i in vec.iter() {
        p.remove(*i);
    }

    for point in vec2.iter().rev() {
        p.insert(0, *point);
    }

    true
}

fn stable_partition_by_key(slice: &mut [usize], is_upper: impl Fn(usize) -> bool) -> usize {
    let mut upper = Vec::new();
    let mut idx = 0;

    for j in 0..slice.len() {
        if is_upper(slice[j]) {
            upper.push(slice[j]);
        } else {
            slice[idx] = slice[j];
            idx += 1;
        }
    }

    slice[idx..].copy_from_slice(&upper);

    idx
}

unsafe fn hull3(p: &mut Vec<Point3>) -> Vec<*mut Face> {
    let n = p.len();
    let check = prepare(p);
    let mut f: Vec<*mut Face> = Vec::new();

    if !check {
        return f;
    }

    let mut new_face: Vec<*mut Face> = vec![ptr::null_mut(); n];
    let mut conflict = vec![Vec::new(); n];

    let mut add_face = |a: usize, b: usize, c: usize| -> *mut Face {
        let face = Box::new(Face::new(a, b, c, (p[b] - p[a]).cross(&(p[c] - p[a]))));
        let face_ptr = Box::into_raw(face);
        f.push(face_ptr);
        face_ptr
    };

    let f1 = add_face(0, 1, 2);
    let f2 = add_face(0, 2, 1);

    glue(f1, f2, &mut (*f1).e1, &mut (*f2).e3);
    glue(f1, f2, &mut (*f1).e2, &mut (*f2).e2);
    glue(f1, f2, &mut (*f1).e3, &mut (*f2).e1);

    for i in 3..n {
        for f in [f1, f2].iter() {
            let q = (p[i] - p[(*(*f)).a]).dot(&(*(*f)).q);

            if q > 1e-9 {
                conflict[i].push(*f);
            }

            if q >= -1e-9 {
                (*(*f)).points.push(i);
            }
        }
    }

    let set_union = |a: Vec<usize>, b: Vec<usize>| -> Vec<usize> {
        let mut stack = a.clone();

        for val in b.iter() {
            stack.push(*val);
        }

        let mut ret = stack.clone();

        for x in (0..ret.len()).rev() {
            for y in (x + 1..ret.len()).rev() {
                if ret[x] == ret[y] {
                    ret.remove(y);
                }
            }
        }

        ret
    };

    for i in 3..n {
        for f in conflict[i].iter() {
            (*(*f)).dead = (*(*f)).dead.min(i);
        }

        let mut v = -1;

        for idx in 0..conflict[i].len() {
            if (*conflict[i][idx]).dead != i {
                continue;
            }

            let parr = [
                (*conflict[i][idx]).a,
                (*conflict[i][idx]).b,
                (*conflict[i][idx]).c,
            ];
            let earr = [
                (*conflict[i][idx]).e1,
                (*conflict[i][idx]).e2,
                (*conflict[i][idx]).e3,
            ];

            for j in 0..3 {
                let j2 = (j + 1) % 3;

                if (*(*earr[j]).face).dead > i {
                    new_face[parr[j]] = add_face(parr[j], parr[j2], i);
                    let combined_face = new_face[parr[j]];

                    let union = set_union(
                        (*conflict[i][idx]).points.clone(),
                        (*(*earr[j]).face).points.clone(),
                    );
                    (*combined_face).points.extend(union.iter());

                    let pos = stable_partition_by_key(&mut (*combined_face).points, |x| {
                        !(x > i && (p[x] - p[(*combined_face).a]).dot(&(*combined_face).q) > 1e-9)
                    });
                    (*combined_face).points.truncate(pos);

                    for k in (*combined_face).points.iter() {
                        conflict[*k].push(combined_face);
                    }

                    (*(*earr[j]).rev).face = combined_face;
                    (*combined_face).e1 = earr[j];
                    v = parr[j] as i64;
                }
            }
        }

        if v == -1 {
            continue;
        }

        while (*new_face[v as usize]).e2 == ptr::null_mut() {
            let u = (*new_face[v as usize]).b;
            glue(
                new_face[v as usize],
                new_face[u],
                &mut (*new_face[v as usize]).e2,
                &mut (*new_face[u]).e3,
            );
            v = u as i64;
        }
    }

    f.retain(|x| (*(*x)).dead >= n);

    f
}

struct Plane {
    normal: Point3,
    origin: f64,
}

impl Plane {
    fn new(normal: Point3, origin: f64) -> Self {
        Self { normal, origin }
    }

    fn dist(&self, point: &Point3) -> f64 {
        self.normal.dot(point) + self.origin
    }

    fn coord(&self) -> Vec<Point3> {
        let mut origin = Point3::new(0.0, 0.0, 0.0);

        if self.origin != 0.0 {
            origin = self.intersect_plane(&Point3::new(0.0, 0.0, 0.0), &self.normal);
        }

        let never = Point3::new(2103.0, 1.0, 0.0);
        let xh = self.normal.cross(&never);
        let yh = self.normal.cross(&xh);

        vec![xh.normalize(), yh.normalize(), origin]
    }

    unsafe fn intersect_all(&self, faces: &Vec<*mut Face>, points: &Vec<Point3>) -> Vec<Point3> {
        let mut ret = Vec::new();

        for point in points.iter() {
            if self.dist(point) == 0.0 {
                ret.push(point.clone());
            }
        }

        for face in faces.iter() {
            let a = points[(*(*face)).a];
            let b = points[(*(*face)).b];
            let c = points[(*(*face)).c];
            let plane_a = self.dist(&a);
            let plane_b = self.dist(&b);
            let plane_c = self.dist(&c);

            if plane_a == 0.0 {
                ret.push(a);
            }

            if plane_b == 0.0 {
                ret.push(b);
            }

            if plane_c == 0.0 {
                ret.push(c);
            }

            if plane_a * plane_b < 0.0 {
                ret.push(self.intersect_plane(&a, &(b - a)));
            }

            if plane_b * plane_c < 0.0 {
                ret.push(self.intersect_plane(&b, &(c - b)));
            }

            if plane_c * plane_a < 0.0 {
                ret.push(self.intersect_plane(&c, &(a - c)));
            }
        }

        ret.sort_by(|a, b| {
            if a.x != b.x {
                a.x.partial_cmp(&b.x).unwrap()
            } else if a.y != b.y {
                a.y.partial_cmp(&b.y).unwrap()
            } else {
                a.z.partial_cmp(&b.z).unwrap()
            }
        });
        ret.dedup();

        ret
    }

    fn intersect_plane(&self, face: &Point3, direction: &Point3) -> Point3 {
        let t = -(self.normal.dot(face) + self.origin) / (self.normal.dot(direction));
        *face + direction.mul(t)
    }
}

#[derive(Debug, Copy, Clone)]
struct Point2 {
    x: f64,
    y: f64,
}

impl Point2 {
    fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }
}

impl std::ops::Sub for Point2 {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

#[derive(Debug, Clone)]
struct ConvexHull2 {
    points: Vec<Point2>,
}

impl ConvexHull2 {
    fn try_new(mut points: Vec<Point2>) -> Self {
        let mut idx_p0 = 0;

        for i in 0..points.len() {
            if points[i].x < points[idx_p0].x {
                idx_p0 = i;
            } else if points[i].x == points[idx_p0].x && points[i].y < points[idx_p0].y {
                idx_p0 = i;
            }
        }

        let p0 = points.remove(idx_p0);

        points.sort_by(|a, b| {
            let ccw = ConvexHull2::calculate_ccw(&p0, a, b);

            if ccw != 0.0 {
                return ccw.partial_cmp(&0.0).unwrap().reverse();
            }

            if a.x != b.x {
                return a.x.partial_cmp(&b.x).unwrap();
            }

            a.y.partial_cmp(&b.y).unwrap()
        });

        let mut stack = Vec::new();
        stack.push(p0);

        for i in 0..points.len() {
            while stack.len() >= 2
                && ConvexHull2::calculate_ccw(
                    &stack[stack.len() - 2],
                    &stack[stack.len() - 1],
                    &points[i],
                ) <= 0.0
            {
                stack.pop();
            }

            stack.push(points[i]);
        }

        if ConvexHull2::calculate_ccw(&stack[stack.len() - 2], &stack[stack.len() - 1], &p0) == 0.0
        {
            stack.pop();
        }

        Self { points: stack }
    }

    fn area(&self) -> f64 {
        let mut ret = 0.0;
        let a = self.points[0];

        for i in 1..self.points.len() - 1 {
            let b = self.points[i];
            let c = self.points[i + 1];

            ret += ConvexHull2::calculate_ccw(&a, &b, &c)
        }

        ret / 2.0
    }

    fn calculate_ccw(p1: &Point2, p2: &Point2, p3: &Point2) -> f64 {
        let (x1, y1) = (p1.x, p1.y);
        let (x2, y2) = (p2.x, p2.y);
        let (x3, y3) = (p3.x, p3.y);

        (x2 - x1) * (y3 - y1) - (x3 - x1) * (y2 - y1)
    }
}

// Reference: https://github.com/andom9/chull
// Reference: https://codeforces.com/blog/entry/81768
// Thanks for @seungwuk98 to help some parts of code! (Plane-related)
fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let (n, q) = (scan.token::<usize>(), scan.token::<i64>());
    let mut points = vec![Point3::new(0.0, 0.0, 0.0); n];

    for i in 0..n {
        points[i] = Point3::new(
            scan.token::<f64>(),
            scan.token::<f64>(),
            scan.token::<f64>(),
        );
    }

    unsafe {
        let hull = hull3(&mut points);

        let convert = |points: &Vec<Point3>, plane: &Plane| -> Vec<Point2> {
            let coords = plane.coord();
            let (xh, yh, origin) = (&coords[0], &coords[1], &coords[2]);
            let mut ret = Vec::new();

            for point in points {
                let x = xh.dot(&(*point - *origin));
                let y = yh.dot(&(*point - *origin));

                ret.push(Point2::new(x, y));
            }

            ret
        };

        for _ in 0..q {
            let (a, b, c, d) = (
                scan.token::<f64>(),
                scan.token::<f64>(),
                scan.token::<f64>(),
                scan.token::<f64>(),
            );

            let plane = Plane::new(Point3::new(a, b, c), d);
            let intersections = plane.intersect_all(&hull, &points);

            if intersections.len() < 3 {
                writeln!(out, "0").unwrap();
                continue;
            }

            let points = convert(&intersections, &plane);
            let convex_hull = ConvexHull2::try_new(points);
            let ret = convex_hull.area();

            writeln!(out, "{:.3}", ret).unwrap();
        }
    }
}
