use io::Write;
use std::{io, ptr, str};

struct Rng([u64; 4]);

impl Rng {
    fn split_mix(v: u64) -> u64 {
        let mut z = v.wrapping_add(0x9e3779b97f4a7c15);

        z = (z ^ (z >> 30)).wrapping_mul(0xbf58476d1ce4e5b9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94d049bb133111eb);
        z ^ (z >> 31)
    }

    fn new() -> Self {
        let mut seed = 0;
        unsafe { std::arch::x86_64::_rdrand64_step(&mut seed) };

        let mut prev = seed;

        Self(std::array::from_fn(|_| {
            prev = Self::split_mix(prev);
            prev
        }))
    }

    fn next(&mut self, n: u64) -> u64 {
        let [x, y, z, c] = &mut self.0;
        let t = x.wrapping_shl(58) + *c;

        *c = *x >> 6;
        *x = x.wrapping_add(t);

        if *x < t {
            *c += 1;
        }

        *z = z.wrapping_mul(6906969069).wrapping_add(1234567);
        *y ^= y.wrapping_shl(13);
        *y ^= *y >> 17;
        *y ^= y.wrapping_shl(43);

        let base = x.wrapping_add(*y).wrapping_add(*z);
        ((base as u128 * n as u128) >> 64) as u64
    }
}

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
    x: i128,
    y: i128,
    z: i128,
}

impl Point3 {
    fn new(x: i128, y: i128, z: i128) -> Self {
        Self { x, y, z }
    }

    fn equals(&self, other: &Point3) -> bool {
        self.x == other.x && self.y == other.y && self.z == other.z
    }

    fn cross(&self, other: &Point3) -> Self {
        Self::new(
            self.y * other.z - self.z * other.y,
            self.z * other.x - self.x * other.z,
            self.x * other.y - self.y * other.x,
        )
    }

    fn dot(&self, other: &Point3) -> i128 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    fn is_colinear(a: &Point3, b: &Point3, c: &Point3) -> bool {
        let ab = *b - *a;
        let ac = *c - *a;

        ab.cross(&ac) == Point3::new(0, 0, 0)
    }

    fn is_coplanar(a: &Point3, b: &Point3, c: &Point3, d: &Point3) -> bool {
        let ab = *b - *a;
        let ac = *c - *a;
        let ad = *d - *a;

        ab.dot(&ac.cross(&ad)) == 0
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

struct ConvexHull3 {
    faces: Vec<*mut Face>,
}

impl ConvexHull3 {
    unsafe fn try_new(p: &mut Vec<Point3>) -> Self {
        let n = p.len();
        ConvexHull3::prepare(p);
        let mut f: Vec<*mut Face> = Vec::new();

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

        ConvexHull3::glue(f1, f2, &mut (*f1).e1, &mut (*f2).e3);
        ConvexHull3::glue(f1, f2, &mut (*f1).e2, &mut (*f2).e2);
        ConvexHull3::glue(f1, f2, &mut (*f1).e3, &mut (*f2).e1);

        for i in 3..n {
            for f in [f1, f2].iter() {
                let q = (p[i] - p[(*(*f)).a]).dot(&(*(*f)).q);

                if q > 0 {
                    conflict[i].push(*f);
                }

                if q >= 0 {
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

                        let pos = ConvexHull3::stable_partition_by_key(
                            &mut (*combined_face).points,
                            |x| {
                                !(x > i
                                    && (p[x] - p[(*combined_face).a]).dot(&(*combined_face).q) > 0)
                            },
                        );
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
                ConvexHull3::glue(
                    new_face[v as usize],
                    new_face[u],
                    &mut (*new_face[v as usize]).e2,
                    &mut (*new_face[u]).e3,
                );
                v = u as i64;
            }
        }

        f.retain(|x| (*(*x)).dead >= n);

        Self { faces: f }
    }

    fn prepare(p: &mut Vec<Point3>) {
        let n = p.len();

        for _ in 0..n {
            let idx1 = Rng::new().next(n as u64) as usize;
            let idx2 = Rng::new().next(n as u64) as usize;

            p.swap(idx1, idx2);
        }

        let mut vec = Vec::new();
        vec.push(0);

        for i in 1..n {
            if vec.len() == 1 {
                if p[vec[0]] - p[i] != Point3::new(0, 0, 0) {
                    vec.push(i);
                }
            } else if vec.len() == 2 {
                if !Point3::is_colinear(&p[vec[0]], &p[vec[1]], &p[i]) {
                    vec.push(i);
                }
            } else if !Point3::is_coplanar(&p[vec[0]], &p[vec[1]], &p[vec[2]], &p[i]) {
                vec.push(i);
                break;
            }
        }

        assert!(vec.len() == 4);

        let mut vec2 = Vec::with_capacity(vec.len());

        for i in vec.iter() {
            vec2.push(p[*i]);
        }

        vec.reverse();

        for i in vec.iter() {
            p.remove(*i);
        }

        vec2.append(p);
        *p = vec2;
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
}

fn check_omnicircumnavigation(points: &mut Vec<Point3>) -> bool {
    // Check colinear
    for i in 2..points.len() {
        if !Point3::is_colinear(&points[0], &points[1], &points[i]) {
            points.swap(2, i);
            break;
        }
    }

    let mut is_coplanar = true;

    // Check coplanar
    for i in 3..points.len() {
        if !Point3::is_coplanar(&points[0], &points[1], &points[2], &points[i]) {
            points.swap(3, i);
            is_coplanar = false;
            break;
        }
    }

    if is_coplanar {
        points.push(Point3::new(1_234_567, 2_345_678, 3_456_789));
    }

    unsafe {
        let convex_hull = ConvexHull3::try_new(points);
        let mut vertices = Vec::new();

        for face in convex_hull.faces.iter() {
            let a = points[(*(*face)).a];
            let b = points[(*(*face)).b];
            let c = points[(*(*face)).c];

            if a.equals(&Point3::new(0, 0, 0))
                || b.equals(&Point3::new(0, 0, 0))
                || c.equals(&Point3::new(0, 0, 0))
            {
                let cross = (b - a).cross(&(c - a));

                if vertices.len() == 0 {
                    vertices.push(cross);
                } else if vertices.len() == 1 {
                    if vertices[0].cross(&cross) != Point3::new(0, 0, 0) {
                        vertices.push(cross);
                    }
                } else {
                    if vertices[0].cross(&vertices[1]).dot(&cross) != 0 {
                        return false;
                    }
                }
            }
        }

        true
    }
}

// Reference: https://github.com/andom9/chull
// Reference: https://codeforces.com/blog/entry/81768
fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let t = scan.token::<i64>();

    for i in 1..=t {
        let n = scan.token::<usize>();
        let mut points = vec![Point3::new(0, 0, 0); n + 1];

        for j in 0..n {
            points[j] = Point3::new(
                scan.token::<i128>(),
                scan.token::<i128>(),
                scan.token::<i128>(),
            );
        }

        let ret = check_omnicircumnavigation(&mut points);

        writeln!(out, "Case #{i}: {}", if ret { "YES" } else { "NO" }).unwrap();
    }
}
