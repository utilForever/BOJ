use io::Write;
use std::{
    collections::hash_map::RandomState,
    hash::{BuildHasher, Hasher},
    iter::repeat_with,
};
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

const EPS: f64 = 1e-9;

#[derive(Clone, Copy, Debug)]
struct Vec2 {
    x: f64,
    y: f64,
}

impl std::ops::Add for Vec2 {
    type Output = Vec2;

    fn add(self, other: Vec2) -> Vec2 {
        Vec2 {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl std::ops::Sub for Vec2 {
    type Output = Vec2;

    fn sub(self, other: Vec2) -> Vec2 {
        Vec2 {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl std::ops::Mul<f64> for Vec2 {
    type Output = Vec2;

    fn mul(self, scalar: f64) -> Vec2 {
        Vec2 {
            x: self.x * scalar,
            y: self.y * scalar,
        }
    }
}

fn dot(a: Vec2, b: Vec2) -> f64 {
    a.x * b.x + a.y * b.y
}

fn cross(a: Vec2, b: Vec2) -> f64 {
    a.x * b.y - a.y * b.x
}

fn squared_length(v: Vec2) -> f64 {
    dot(v, v)
}

fn signed_angle(a: Vec2, b: Vec2) -> f64 {
    cross(a, b).atan2(dot(a, b))
}

struct HalfPlane {
    normal: Vec2,
    offset: f64,
}

fn clip_convex_polygon(polygon: &[Vec2], plane: &HalfPlane) -> Vec<Vec2> {
    let n = polygon.len();
    let mut ret = Vec::new();

    for i in 0..n {
        let p = polygon[i];
        let q = polygon[(i + 1) % n];

        let vp = dot(plane.normal, p) - plane.offset;
        let vq = dot(plane.normal, q) - plane.offset;

        let inside_p = vp <= EPS;
        let inside_q = vq <= EPS;

        if inside_p {
            ret.push(p);
        }

        if inside_p ^ inside_q {
            let direction = q - p;
            let t = (plane.offset - dot(plane.normal, p)) / dot(plane.normal, direction);

            ret.push(Vec2 {
                x: p.x + direction.x * t,
                y: p.y + direction.y * t,
            });
        }
    }

    ret
}

fn segment_circle_intersection_params(p: Vec2, q: Vec2, r: f64) -> Vec<f64> {
    let mut ret = Vec::new();

    let diff = q - p;
    let a = dot(diff, diff);
    let b = 2.0 * dot(p, diff);
    let c = dot(p, p) - r * r;

    let mut d = b * b - 4.0 * a * c;

    if d < -EPS {
        return ret;
    }

    d = d.max(0.0);

    let d_sqrt = d.sqrt();
    let t1 = (-b - d_sqrt) / (2.0 * a);
    let t2 = (-b + d_sqrt) / (2.0 * a);

    if (0.0..1.0).contains(&t1) {
        ret.push(t1);
    }

    if (0.0..1.0).contains(&t2) {
        ret.push(t2);
    }

    ret
}

fn area_segment_circle_intersection(a: Vec2, b: Vec2, r: f64) -> f64 {
    let a_inside = squared_length(a) <= r * r + EPS;
    let b_inside = squared_length(b) <= r * r + EPS;

    if a_inside && b_inside {
        return 0.5 * cross(a, b);
    }

    let mut partitions = vec![(0.0, a)];

    for t in segment_circle_intersection_params(a, b, r) {
        let ip = a + (b - a) * t;
        partitions.push((t, ip));
    }

    partitions.push((1.0, b));
    partitions.sort_unstable_by(|(t1, _), (t2, _)| t1.partial_cmp(t2).unwrap());

    let mut ret = 0.0;

    for window in partitions.windows(2) {
        let p = window[0].1;
        let q = window[1].1;
        let mid = Vec2 {
            x: 0.5 * (p.x + q.x),
            y: 0.5 * (p.y + q.y),
        };

        ret += if squared_length(mid) <= r * r {
            0.5 * cross(p, q)
        } else {
            let angle = signed_angle(p, q);
            0.5 * r * r * angle
        }
    }

    ret
}

fn area_convex_polygon_circle_intersection(polygon: &[Vec2], center: Vec2, r: f64) -> f64 {
    if polygon.len() < 3 {
        return 0.0;
    }

    let mut ret = 0.0;

    for i in 0..polygon.len() {
        let p = Vec2 {
            x: polygon[i].x - center.x,
            y: polygon[i].y - center.y,
        };
        let q = Vec2 {
            x: polygon[(i + 1) % polygon.len()].x - center.x,
            y: polygon[(i + 1) % polygon.len()].y - center.y,
        };

        ret += area_segment_circle_intersection(p, q, r);
    }

    ret.abs()
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let (n, q) = (scan.token::<usize>(), scan.token::<i64>());
    let mut circles = vec![(Vec2 { x: 0.0, y: 0.0 }, 0.0); n];

    for i in 0..n {
        let (x, y, r) = (
            scan.token::<f64>(),
            scan.token::<f64>(),
            scan.token::<f64>(),
        );
        circles[i] = (Vec2 { x, y }, r);
    }

    Xorshift::new().shuffle(&mut circles);

    let bound_extent = 3e6;
    let bounding_box = vec![
        Vec2 {
            x: -bound_extent,
            y: -bound_extent,
        },
        Vec2 {
            x: bound_extent,
            y: -bound_extent,
        },
        Vec2 {
            x: bound_extent,
            y: bound_extent,
        },
        Vec2 {
            x: -bound_extent,
            y: bound_extent,
        },
    ];
    let mut power_cells: Vec<Vec<Vec2>> = vec![Vec::new(); n];

    for i in 0..n {
        let mut cell_polygon = bounding_box.clone();

        for j in 0..n {
            if i == j {
                continue;
            }

            let a = circles[i];
            let b = circles[j];

            let normal = Vec2 {
                x: b.0.x - a.0.x,
                y: b.0.y - a.0.y,
            };
            let offset = (dot(b.0, b.0) - dot(a.0, a.0) + a.1 * a.1 - b.1 * b.1) * 0.5;
            let half_plane = HalfPlane { normal, offset };

            cell_polygon = clip_convex_polygon(&cell_polygon, &half_plane);

            if cell_polygon.is_empty() {
                break;
            }
        }

        power_cells[i] = cell_polygon;
    }

    for _ in 0..q {
        let (x0, y0, w, h) = (
            scan.token::<f64>(),
            scan.token::<f64>(),
            scan.token::<f64>(),
            scan.token::<f64>(),
        );
        let x1 = x0 + w;
        let y1 = y0 + h;

        let right = HalfPlane {
            normal: Vec2 { x: 1.0, y: 0.0 },
            offset: x1,
        };
        let left = HalfPlane {
            normal: Vec2 { x: -1.0, y: 0.0 },
            offset: -x0,
        };
        let top = HalfPlane {
            normal: Vec2 { x: 0.0, y: 1.0 },
            offset: y1,
        };
        let bottom = HalfPlane {
            normal: Vec2 { x: 0.0, y: -1.0 },
            offset: -y0,
        };

        let mut shaded_area = 0.0;

        for i in 0..n {
            if power_cells[i].is_empty() {
                continue;
            }

            let mut clipped_poly = clip_convex_polygon(&power_cells[i], &right);
            clipped_poly = clip_convex_polygon(&clipped_poly, &left);
            clipped_poly = clip_convex_polygon(&clipped_poly, &top);
            clipped_poly = clip_convex_polygon(&clipped_poly, &bottom);

            if clipped_poly.is_empty() {
                continue;
            }

            shaded_area +=
                area_convex_polygon_circle_intersection(&clipped_poly, circles[i].0, circles[i].1);
        }

        let ret = 100.0 * shaded_area / (w * h);

        writeln!(out, "{:.10}", ret).unwrap();
    }
}
