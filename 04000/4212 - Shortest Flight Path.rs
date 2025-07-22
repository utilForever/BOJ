use io::Write;
use std::{
    cmp::{Ordering, Reverse},
    collections::BinaryHeap,
    io,
    ops::{Add, Mul, Sub},
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

#[derive(Clone, Copy, Debug, Default)]
struct Vec3 {
    x: f64,
    y: f64,
    z: f64,
}

impl Vec3 {
    #[inline]
    fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }

    #[inline]
    fn dot(self, other: Vec3) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    #[inline]
    fn cross(self, other: Vec3) -> Self {
        Self::new(
            self.y * other.z - self.z * other.y,
            self.z * other.x - self.x * other.z,
            self.x * other.y - self.y * other.x,
        )
    }

    #[inline]
    fn norm(self) -> f64 {
        self.dot(self).sqrt()
    }

    #[inline]
    fn normalize(self) -> Self {
        self.mul(1.0 / self.norm())
    }
}

impl Add for Vec3 {
    type Output = Self;

    #[inline]
    fn add(self, other: Self) -> Self {
        Self::new(self.x + other.x, self.y + other.y, self.z + other.z)
    }
}

impl Sub for Vec3 {
    type Output = Self;

    #[inline]
    fn sub(self, other: Self) -> Self {
        Self::new(self.x - other.x, self.y - other.y, self.z - other.z)
    }
}

impl Mul<f64> for Vec3 {
    type Output = Self;

    #[inline]
    fn mul(self, scalar: f64) -> Self {
        Self::new(self.x * scalar, self.y * scalar, self.z * scalar)
    }
}

const RADIUS_EARTH: f64 = 6370.0;
const EPS: f64 = 1e-9;

fn intersect_circle(a: Vec3, b: Vec3, r_cos: f64) -> Option<(Vec3, Vec3)> {
    let d = a.dot(b).acos();
    let r = r_cos.acos();

    if d > 2.0 * r + EPS {
        return None;
    }

    let k = a.dot(b);
    let alpha = r_cos / (1.0 + k);
    let p0 = (a + b) * alpha;
    let p0_dot = p0.dot(p0);

    if p0_dot > 1.0 {
        return None;
    }

    let gamma = (1.0 - p0_dot).sqrt();
    let n = a.cross(b);
    let n_norm = n.norm();

    if n_norm < EPS {
        return None;
    }

    let n_unit = n * (1.0 / n_norm);
    let p1 = p0.add(n_unit * (gamma)).normalize();
    let p2 = p0.sub(n_unit * (gamma)).normalize();

    Some((p1, p2))
}

fn angle_on_circle(center: Vec3, p: Vec3, r_sin: f64) -> f64 {
    let mut v = Vec3::new(0.0, 0.0, 1.0);

    if center.cross(v).norm() < EPS {
        v = Vec3::new(0.0, 1.0, 0.0);
    }

    let axis_x = center.cross(v).normalize();
    let axis_y = center.cross(axis_x).normalize();

    let u = p.sub(center * (center.dot(p))) * (1.0 / r_sin);
    let angle = u.dot(axis_x).atan2(u.dot(axis_y));

    if angle < 0.0 {
        angle + 2.0 * std::f64::consts::PI
    } else {
        angle
    }
}

fn len_small_circle_arc(center: Vec3, p: Vec3, q: Vec3, r_sin: f64) -> f64 {
    let u = (p - (center * center.dot(p))) * (1.0 / r_sin);
    let v = (q - (center * center.dot(q))) * (1.0 / r_sin);

    let dot = u.dot(v);
    let cross = center.dot(u.cross(v));
    let delta = cross.atan2(dot).abs();

    RADIUS_EARTH * r_sin * delta
}

fn inside_arc(a: Vec3, b: Vec3, centers: &[Vec3], r_cos: f64) -> bool {
    let d = a.dot(b).acos();

    if d < EPS {
        return true;
    }

    let (d_sin, d_cos) = (d.sin(), d.cos());
    let mut segments: Vec<(f64, f64)> = Vec::new();

    for &center in centers.iter() {
        let k1 = center.dot(a);
        let k2 = center.dot(b);
        let k3 = (-k1 * d_cos + k2) / d_sin;
        let r = (k1 * k1 + k3 * k3).sqrt();

        if r < EPS {
            continue;
        }

        let ratio = r_cos / r;

        if ratio > 1.0 + EPS {
            continue;
        }

        if ratio <= -1.0 {
            segments.push((0.0, d));
            continue;
        }

        let delta = ratio.acos();
        let phi = k3.atan2(k1);

        for k in -1..=1 {
            let left = phi - delta + 2.0 * std::f64::consts::PI * (k as f64);
            let right = phi + delta + 2.0 * std::f64::consts::PI * (k as f64);

            let left = left.max(0.0);
            let right = right.min(d);

            if right - left > EPS {
                segments.push((left, right));
            }
        }
    }

    if segments.is_empty() {
        return false;
    }

    segments.sort_by(|x, y| x.0.partial_cmp(&y.0).unwrap_or(Ordering::Equal));

    let left = segments[0].0;
    let mut right = segments[0].1;

    if left > EPS {
        return false;
    }

    for &(l, r) in &segments[1..] {
        if l <= right + EPS {
            if r > right {
                right = r;
            }
        } else {
            return false;
        }
    }

    right >= d - EPS
}

#[derive(Clone, Copy, PartialEq, PartialOrd)]
struct Float64(f64);

impl Eq for Float64 {}

impl Ord for Float64 {
    fn cmp(&self, o: &Self) -> Ordering {
        self.partial_cmp(o).unwrap()
    }
}

fn process_dijkstra(graph: &Vec<Vec<(usize, f64)>>, from: usize) -> Vec<f64> {
    let mut dist = vec![f64::MAX; graph.len()];
    dist[from] = 0.0;

    let mut queue = BinaryHeap::new();
    queue.push((Reverse(Float64(0.0)), from));

    while let Some((Reverse(cost), u)) = queue.pop() {
        if cost.0 > dist[u] + EPS {
            continue;
        }

        for &(vertex_next, mut cost_next) in graph[u].iter() {
            cost_next += cost.0;

            if dist[vertex_next] > cost_next + EPS {
                dist[vertex_next] = cost_next;
                queue.push((Reverse(Float64(cost_next)), vertex_next));
            }
        }
    }

    dist
}

fn can_travel(dist: &Vec<Vec<f64>>, s: usize, t: usize, c: f64) -> Option<f64> {
    let mut ret = vec![f64::MAX; dist.len()];
    ret[s] = 0.0;

    let mut queue = BinaryHeap::new();
    queue.push((Reverse(Float64(0.0)), s));

    while let Some((Reverse(cost), u)) = queue.pop() {
        if cost.0 > ret[u] + EPS {
            continue;
        }

        if u == t {
            return Some(cost.0);
        }

        for vertex_next in 0..dist.len() {
            let mut cost_next = dist[u][vertex_next];

            if cost_next == f64::MAX || cost_next > c + EPS {
                continue;
            }

            cost_next += cost.0;

            if ret[vertex_next] > cost_next + EPS {
                ret[vertex_next] = cost_next;
                queue.push((Reverse(Float64(cost_next)), vertex_next));
            }
        }
    }

    None
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let mut t = 1;

    loop {
        let line = scan.line().trim().to_string();

        if line.is_empty() {
            break;
        }

        let s = line.split_whitespace().collect::<Vec<_>>();
        let (n, r) = (s[0].parse::<usize>().unwrap(), s[1].parse::<f64>().unwrap());
        let r = r / RADIUS_EARTH;
        let (r_sin, r_cos) = (r.sin(), r.cos());

        let mut airports = Vec::with_capacity(n);

        for _ in 0..n {
            let (longitude, latitude) = (
                scan.token::<f64>().to_radians(),
                scan.token::<f64>().to_radians(),
            );
            airports.push(Vec3::new(
                longitude.cos() * latitude.cos(),
                longitude.sin() * latitude.cos(),
                latitude.sin(),
            ));
        }

        let mut points = airports.clone();
        let mut idxes_on_circle = vec![Vec::new(); n];

        for i in 0..n {
            for j in i + 1..n {
                if let Some((p, q)) = intersect_circle(airports[i], airports[j], r_cos) {
                    let idx_p = points.len();
                    points.push(p);
                    idxes_on_circle[i].push(idx_p);
                    idxes_on_circle[j].push(idx_p);

                    let idx_q = points.len();
                    points.push(q);
                    idxes_on_circle[i].push(idx_q);
                    idxes_on_circle[j].push(idx_q);
                }
            }
        }

        let mut graph = vec![Vec::new(); points.len()];

        for (i, idxes) in idxes_on_circle.iter_mut().enumerate() {
            if idxes.len() < 2 {
                continue;
            }

            idxes.sort_unstable_by(|&a, &b| {
                let angle_a = angle_on_circle(airports[i], points[a], r_sin);
                let angle_b = angle_on_circle(airports[i], points[b], r_sin);
                angle_a.partial_cmp(&angle_b).unwrap()
            });

            for a in 0..idxes.len() {
                let b = (a + 1) % idxes.len();
                let idx_a = idxes[a];
                let idx_b = idxes[b];
                let len = len_small_circle_arc(airports[i], points[idx_a], points[idx_b], r_sin);

                graph[idx_a].push((idx_b, len));
                graph[idx_b].push((idx_a, len));
            }
        }

        for i in 0..points.len() {
            for j in i + 1..points.len() {
                if inside_arc(points[i], points[j], &airports, r_cos) {
                    let len = points[i].dot(points[j]).acos() * RADIUS_EARTH;

                    graph[i].push((j, len));
                    graph[j].push((i, len));
                }
            }
        }

        let mut dist_airport = vec![vec![f64::MAX; n]; n];

        for i in 0..n {
            let dist = process_dijkstra(&graph, i);

            for j in 0..n {
                dist_airport[i][j] = dist[j];
            }
        }

        writeln!(out, "Case {t}:").unwrap();

        let q = scan.token::<i64>();

        for _ in 0..q {
            let (s, t, c) = (
                scan.token::<usize>() - 1,
                scan.token::<usize>() - 1,
                scan.token::<f64>(),
            );

            match can_travel(&dist_airport, s, t, c) {
                Some(ret) => writeln!(out, "{:.3}", ret).unwrap(),
                None => writeln!(out, "impossible").unwrap(),
            }
        }

        t += 1;
    }
}
