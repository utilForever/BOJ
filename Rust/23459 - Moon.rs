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
    x: i64,
    y: i64,
    z: i64,
}

impl Point3 {
    fn new(x: i64, y: i64, z: i64) -> Self {
        Self { x, y, z }
    }

    fn cross(&self, other: &Point3) -> Self {
        Self::new(
            self.y * other.z - self.z * other.y,
            self.z * other.x - self.x * other.z,
            self.x * other.y - self.y * other.x,
        )
    }

    fn dot(&self, other: &Point3) -> i64 {
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

impl std::ops::Mul<f64> for Point3 {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        Self::new(
            (self.x as f64 * rhs) as i64,
            (self.y as f64 * rhs) as i64,
            (self.z as f64 * rhs) as i64,
        )
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
struct Point3d {
    x: f64,
    y: f64,
    z: f64,
}

impl Point3d {
    fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }

    fn dot(&self, other: &Point3d) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }
}

impl std::ops::Add for Point3d {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}

impl std::ops::Sub for Point3d {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}

impl std::ops::Mul<f64> for Point3d {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        Self::new(self.x * rhs, self.y * rhs, self.z * rhs)
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

        match ConvexHull3::prepare(p) {
            Ok(_) => (),
            Err(_) => return Self { faces: Vec::new() },
        }

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
            let mut ret = a;

            ret.extend(b);
            ret.sort();
            ret.dedup();

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

            while (*new_face[v as usize]).e2.is_null() {
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

    fn prepare(p: &mut Vec<Point3>) -> Result<(), ()> {
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

        if vec.len() < 4 {
            return Err(());
        }

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

        Ok(())
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

// Function to determine the orientation of the plane formed by points a, b, c relative to point v
fn orientation(a: &Point3, b: &Point3, c: &Point3, v: &Point3) -> i64 {
    // Calculate the normal vector to the plane defined by points a, b, c
    let normal = (*b - *a).cross(&(*c - *a));

    // Calculate the dot product of the normal vector and the vector from a to v to determine the side of the plane v lies on
    // The result of the dot product can be:
    //     Positive => v is on the positive side of the plane
    //     Negative => v is on the negative side of the plane
    //     Zero     => v is on the plane
    match normal.dot(&(*v - *a)) {
        x if x > 0 => 1,
        x if x < 0 => -1,
        _ => 0,
    }
}

// Function to project point v onto the plane defined by point p and normal vector
fn project(p: &Point3d, normal: &Point3d, v: &Point3d) -> Point3d {
    // Projection formula: v - normal * (dot product of normal and (v - p)) / magnitude of normal squared
    let magnitude = normal.x * normal.x + normal.y * normal.y + normal.z * normal.z;
    *v - *normal * (normal.dot(&(*v - *p)) / magnitude)
}

// Function to calculate the angle between vectors projected onto the plane normal to vector a
fn calculate_angle(a: &Point3d, b: &Point3d, c: &Point3d) -> f64 {
    // Define origin for projection
    let origin = Point3d::new(0.0, 0.0, 0.0);

    // Project b and c onto plane normal to a
    let proj_b = project(&origin, a, b);
    let proj_c = project(&origin, a, c);

    // Calculate the magnitude of the projected vectors
    let abs_b = ((proj_b.x * proj_b.x + proj_b.y * proj_b.y + proj_b.z * proj_b.z) as f64).sqrt();
    let abs_c = ((proj_c.x * proj_c.x + proj_c.y * proj_c.y + proj_c.z * proj_c.z) as f64).sqrt();

    // Calculate cosine of the angle using dot product of projected vectors divided by product of their magnitudes
    // Then clamp it to the range [-1,1] for valid acos
    let ret = (proj_b.dot(&proj_c) as f64 / (abs_b * abs_c)).clamp(-1.0, 1.0);

    // Return the arc cosine of ret, which is the angle in radians
    ret.acos()
}

/*
- How to solve?

Step 1: Understanding the Problem and Geometry
- If you can draw a hemisphere that includes all the given points on the sphere, then f is 1.
  If no such hemisphere exists that can contain all points, f is 0.
  A key insight is that if the convex hull of these points doesn't enclose the center of the sphere,
  then a hemisphere containing all points exists.
Step 2: Convex Hull Calculation
- The convex hull of points on a sphere is the smallest convex set on the sphere that includes all points.
  The key geometric concept is that if all points and their convex hull do not include the origin (center of the sphere),
  these points can be contained in a hemisphere.
Step 3: Orientation Function:
- The orientation function determines whether the plane formed by three points a, b, and c faces towards or away from another point v.
  In the context of this problem, if the origin lies outside the convex hull on one side of every face of the hull,
  then all points lie on the hemisphere on the opposite side.
Step 4: Calculating Solid Angles
- The total solid angle subtended by the convex hull at the sphere's center is computed.
  If this angle is less than 2π (half the total spherical angle of 4π), it implies there's at least one hemisphere that can contain all points.
  Otherwise, points wrap around the sphere in such a way that no single hemisphere can contain them all.
Step 5: Main Calculation
- Convex Hull: First, the convex hull of the points on the sphere is calculated.
               This requires a robust convex hull algorithm suitable for three-dimensional points.
- Checking Faces of the Hull: For each face of the convex hull, it checks if the origin (0, 0, 0) is not enclosed by ensuring it's on the outside of every face.
                              If any face encloses the origin, the loop breaks, and further calculations adjust the solid angle calculation accordingly.
- Calculate Solid Angles for Faces: If a face's orientation is such that it does not face towards the origin,
                                    it contributes to the total solid angle subtended by these points at the origin.
Step 6: Output Calculation
- The calculated total angle ret subtracted from π (to adjust for overcounting in the convex hull faces) gives the total solid angle subtended.
- The final expected value f is adjusted by dividing this angle by 4π (total spherical angle), subtracting from 1,
  and ensuring the value stays within 0 and 1 using .clamp(0.0, 1.0).
- This value represents the probability that a randomly chosen point a_0 will also be inside that hemisphere,
  thus giving the expected value of f.
*/

// Thanks for henryx to provide the important idea of the solution
fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let n = scan.token::<usize>();
    let mut points = vec![Point3::new(0, 0, 0); n + 1];

    for i in 0..n {
        let (x, y, z) = (
            scan.token::<i64>(),
            scan.token::<i64>(),
            scan.token::<i64>(),
        );
        points[i] = Point3::new(x, y, z);
    }

    let mut ret = 0.0;

    unsafe {
        let convex_hull = ConvexHull3::try_new(&mut points);
        let mut points_normalized = vec![Point3d::new(0.0, 0.0, 0.0); n + 1];

        for i in 0..=n {
            let (x, y, z) = (points[i].x, points[i].y, points[i].z);
            let d = ((x * x + y * y + z * z) as f64).sqrt();

            points_normalized[i] = Point3d::new(x as f64 / d, y as f64 / d, z as f64 / d);
        }

        // Calculate solid angles for each face of the convex hull and accumulate them
        for face in convex_hull.faces.iter() {
            let a = points[(*(*face)).a];
            let b = points[(*(*face)).b];
            let c = points[(*(*face)).c];

            // Skip processing if the orientation relative to the origin is not negative
            if orientation(&a, &b, &c, &Point3::new(0, 0, 0)) >= 0 {
                continue;
            }

            // Use normalized points to calculate angles
            let a = &points_normalized[(*(*face)).a];
            let b = &points_normalized[(*(*face)).b];
            let c = &points_normalized[(*(*face)).c];

            let angle_a = calculate_angle(a, b, c);
            let angle_b = calculate_angle(b, c, a);
            let angle_c = calculate_angle(c, a, b);

            // Accumulate solid angles and subtract pi to account for the spherical excess
            ret += angle_a + angle_b + angle_c - std::f64::consts::PI;
        }
    }

    // Output the result clamped between 0 and 1, normalized over the total solid angle
    writeln!(
        out,
        "{:.9}",
        (1.0 - ret / (4.0 * std::f64::consts::PI)).clamp(0.0, 1.0)
    )
    .unwrap();
}
