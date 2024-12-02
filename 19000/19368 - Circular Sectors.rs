use io::Write;
use std::{
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

#[derive(Debug, Clone, Copy)]
struct Segment {
    s: Point,
    t: Point,
}

impl Segment {
    #[inline(always)]
    pub fn new(s: Point, t: Point) -> Self {
        Self { s, t }
    }

    // Check if a point lies on the segment
    // Segments are parallel if their direction vectors are scalar multiples
    pub fn lie_on(&self, p: &Point) -> bool {
        sign((*p - self.s).cross(&(*p - self.t))) == 0
            && sign((*p - self.s).dot(&(*p - self.t))) <= 0
    }

    // Check if two segments are parallel
    pub fn parallel(&self, other: &Segment) -> bool {
        sign((self.t - self.s).cross(&(other.t - other.s))) == 0
    }

    // Compute the intersection point of two lines (as if they were infinite lines) using line equations
    pub fn intersect_point(&self, other: &Segment) -> Point {
        let cross1 = (self.t - self.s).cross(&(other.s - self.s));
        let cross2 = (self.t - self.s).cross(&(other.t - self.s));

        (other.s * cross2 - other.t * cross1) / (cross2 - cross1)
    }

    // Project a point onto the line defined by the segment
    pub fn project_to(&self, p: &Point) -> Point {
        self.s
            + (self.t - self.s)
                * ((*p - self.s).dot(&(self.t - self.s)) / (self.t - self.s).dist2())
    }

    // Compute the distance from a point to the line defined by the segment
    pub fn point_to(&self, p: &Point) -> f64 {
        ((self.t - self.s).cross(&(*p - self.s))).abs() / (self.s - self.t).dist()
    }
}

#[derive(Debug, Clone, Copy)]
struct CircularSector {
    center: Point,      // Center point of the circle
    radius: f64,        // Radius of the circle
    angle_start: f64,   // Starting angle (in radians)
    angle_central: f64, // Central angle (in radians)
    segment1: Segment,  // First boundary segment (from center to start angle)
    segment2: Segment,  // Second boundary segment (from center to end angle)
}

impl CircularSector {
    #[inline(always)]
    pub fn new(center: Point, radius: f64, angle_start: f64, angle_central: f64) -> Self {
        let segment1 = Segment::new(
            center,
            Point::new(
                center.x + radius * angle_start.cos(),
                center.y + radius * angle_start.sin(),
            ),
        );
        let segment2 = Segment::new(
            center,
            Point::new(
                center.x + radius * (angle_start + angle_central).cos(),
                center.y + radius * (angle_start + angle_central).sin(),
            ),
        );

        Self {
            center,
            radius,
            angle_start,
            angle_central,
            segment1,
            segment2,
        }
    }

    // Check if a point lies within the circular sector
    pub fn valid(&self, p: &Point) -> bool {
        // Compute distance from center
        let dist = (*p - self.center).dist();

        // The point is at the center
        if sign(dist) == 0 {
            return true;
        }

        // Outside the circle
        if sign(dist - self.radius) > 0 {
            return false;
        }

        // Compute the angle of the point relative to the center
        let mut phi = (p.y - self.center.y).atan2(p.x - self.center.x);

        // Adjust the angle to be within the sector's angle range
        for _ in 0..3 {
            // Point is within the sector
            if sign(self.angle_start - phi) <= 0
                && sign(phi - (self.angle_start + self.angle_central)) <= 0
            {
                return true;
            }

            phi += std::f64::consts::TAU;
        }

        // Point is outside the sector
        false
    }

    // Compute the intersection points between a segment and the circle boundary
    pub fn intersect_segment_and_circle(&self, seg: &Segment) -> Vec<Point> {
        let d = seg.point_to(&self.center);

        if sign(d - self.radius) > 0 {
            // No intersection
            vec![]
        } else {
            // Compute the intersection points
            let x = (self.radius * self.radius - d * d).max(0.0).sqrt();
            let p = seg.project_to(&self.center);
            let delta = (seg.t - seg.s).normalize() * x;

            let mut ret = vec![p - delta, p + delta];

            // Filter points that lie on the segment
            ret.retain(|&p| sign((p - seg.s).dot(&(p - seg.t))) <= 0);
            ret
        }
    }

    // Compute the intersection points between two circles (the boundaries of circular sectors)
    pub fn intersect_circle_and_circle(&self, other: &CircularSector) -> Vec<Point> {
        let d = (self.center - other.center).dist();

        // Check for special cases where there are no intersections or infinite intersections
        if sign(d) == 0
            || sign(d - self.radius - other.radius) >= 0
            || sign(d - (self.radius - other.radius).abs()) <= 0
        {
            vec![]
        } else {
            // Compute the intersection points
            let r = (other.center - self.center).normalize();
            let r_rot = Point::new(-r.y, r.x);
            let x = ((self.radius * self.radius - other.radius * other.radius) / d + d) / 2.0;
            let h = (self.radius * self.radius - x * x).sqrt();

            vec![
                self.center + r * x - r_rot * h,
                self.center + r * x + r_rot * h,
            ]
        }
    }
}

struct CircularSectorUnion {
    n: usize,
    sectors: Vec<CircularSector>,
}

impl CircularSectorUnion {
    pub fn new(n: usize, sectors: Vec<CircularSector>) -> Self {
        Self { n, sectors }
    }

    // Compute the area contributed by the straight edges (segments) of a sector
    fn area_segment(&self, idx: usize) -> f64 {
        // Initialize lists to store event points for segment1 and segment2
        let mut intersect_segments = [
            vec![
                (self.sectors[idx].center, 1),
                (self.sectors[idx].segment1.t, -1),
            ],
            vec![
                (self.sectors[idx].center, 1),
                (self.sectors[idx].segment2.t, -1),
            ],
        ];

        // For each other sector, find intersections with the segments
        for i in 0..self.n {
            if i == idx {
                continue;
            }

            // Process segment1
            {
                let mut base =
                    self.sectors[i].intersect_segment_and_circle(&self.sectors[idx].segment1);
                base.push(self.sectors[i].center);

                // Find intersection points with other sectors' segments
                if !self.sectors[idx]
                    .segment1
                    .parallel(&self.sectors[i].segment1)
                {
                    base.push(
                        self.sectors[idx]
                            .segment1
                            .intersect_point(&self.sectors[i].segment1),
                    );
                }

                if !self.sectors[idx]
                    .segment1
                    .parallel(&self.sectors[i].segment2)
                {
                    base.push(
                        self.sectors[idx]
                            .segment1
                            .intersect_point(&self.sectors[i].segment2),
                    );
                }

                // Collect candidate points that lie on segment1 and are valid
                let mut candidates = vec![self.sectors[idx].center, self.sectors[idx].segment1.t];

                for point in base {
                    if self.sectors[idx].segment1.lie_on(&point) && self.sectors[i].valid(&point) {
                        candidates.push(point);
                    }
                }

                // Sort candidates by distance from center
                candidates.sort_by(|a, b| {
                    let dist1 = (*a - self.sectors[idx].center).dist();
                    let dist2 = (*b - self.sectors[idx].center).dist();

                    sign(dist1 - dist2).partial_cmp(&0).unwrap()
                });

                // Create segments between consecutive candidates that are outside other sectors
                let mut segments = Vec::new();

                for j in 0..candidates.len() - 1 {
                    let mid = (candidates[j] + candidates[j + 1]) / 2.0;

                    if !self.sectors[i].valid(&mid)
                        || (i < idx && self.sectors[i].segment1.lie_on(&mid))
                    {
                        segments.push((candidates[j], candidates[j + 1]));
                    }
                }

                // Add segments to the event list
                for segment in segments {
                    intersect_segments[0].push((segment.0, 1));
                    intersect_segments[0].push((segment.1, -1));
                }
            }

            // Process segment2 (similar to segment1)
            {
                let mut base =
                    self.sectors[i].intersect_segment_and_circle(&self.sectors[idx].segment2);
                base.push(self.sectors[i].center);

                if !self.sectors[idx]
                    .segment2
                    .parallel(&self.sectors[i].segment1)
                {
                    base.push(
                        self.sectors[idx]
                            .segment2
                            .intersect_point(&self.sectors[i].segment1),
                    );
                }

                if !self.sectors[idx]
                    .segment2
                    .parallel(&self.sectors[i].segment2)
                {
                    base.push(
                        self.sectors[idx]
                            .segment2
                            .intersect_point(&self.sectors[i].segment2),
                    );
                }

                let mut candidates = vec![self.sectors[idx].center, self.sectors[idx].segment2.t];

                for point in base {
                    if self.sectors[idx].segment2.lie_on(&point) && self.sectors[i].valid(&point) {
                        candidates.push(point);
                    }
                }

                candidates.sort_by(|a, b| {
                    let dist1 = (*a - self.sectors[idx].center).dist();
                    let dist2 = (*b - self.sectors[idx].center).dist();

                    sign(dist1 - dist2).partial_cmp(&0).unwrap()
                });

                let mut segments = Vec::new();

                for j in 0..candidates.len() - 1 {
                    let mid = (candidates[j] + candidates[j + 1]) / 2.0;

                    if !self.sectors[i].valid(&mid)
                        || (i < idx && self.sectors[i].segment2.lie_on(&mid))
                    {
                        segments.push((candidates[j], candidates[j + 1]));
                    }
                }

                for segment in segments {
                    intersect_segments[1].push((segment.0, 1));
                    intersect_segments[1].push((segment.1, -1));
                }
            }
        }

        // Sort the event points for segment1 and segment2
        for events in &mut intersect_segments {
            events.sort_by(|a, b| {
                let dist_a = (a.0 - self.sectors[idx].center).dist();
                let dist_b = (b.0 - self.sectors[idx].center).dist();

                dist_a.partial_cmp(&dist_b).unwrap()
            });
        }

        let mut ret = 0.0;

        // Compute the area contributed by segment1
        {
            let mut cnt_event = 0;
            let mut sum = 0.0;

            for i in 0..intersect_segments[0].len() {
                cnt_event += intersect_segments[0][i].1;

                if cnt_event == self.n as i64 {
                    sum += intersect_segments[0][i]
                        .0
                        .cross(&intersect_segments[0][i + 1].0);
                }
            }

            ret += sum;
        }

        // Compute the area contributed by segment2
        {
            let mut cnt_event = 0;
            let mut sum = 0.0;

            for i in 0..intersect_segments[1].len() {
                cnt_event += intersect_segments[1][i].1;

                if cnt_event == self.n as i64 {
                    sum += intersect_segments[1][i]
                        .0
                        .cross(&intersect_segments[1][i + 1].0);
                }
            }

            ret -= sum;
        }

        ret
    }

    // Compute the area contributed by the arc (circular part) of a sector
    fn area_arc(&self, idx: usize) -> f64 {
        let angle_end = self.sectors[idx].angle_start + self.sectors[idx].angle_central;
        // Initialize event points for the arc
        let mut intersect_arcs = vec![(self.sectors[idx].angle_start, 1), (angle_end, -1)];

        // For each other sector, find intersections with the arc
        for i in 0..self.n {
            if i == idx {
                continue;
            }

            // Collect intersection points between the circles
            let mut base = self.sectors[idx].intersect_circle_and_circle(&self.sectors[i]);
            // Collect intersection points between the sector's segments and the other sector's circle
            let extra1 = self.sectors[idx].intersect_segment_and_circle(&self.sectors[i].segment1);
            let extra2 = self.sectors[idx].intersect_segment_and_circle(&self.sectors[i].segment2);

            base.append(&mut extra1.clone());
            base.append(&mut extra2.clone());

            // Collect candidate angles where the sectors intersect
            let mut candidates = vec![self.sectors[idx].angle_start, angle_end];

            for point in base {
                if !self.sectors[idx].valid(&point) || !self.sectors[i].valid(&point) {
                    continue;
                }

                // Compute the angle of the intersection point
                let mut phi = (point.y - self.sectors[idx].center.y)
                    .atan2(point.x - self.sectors[idx].center.x);

                // Adjust the angle to be within the sector's range
                while sign(self.sectors[idx].angle_start - phi) > 0 {
                    phi += std::f64::consts::TAU;
                }

                candidates.push(phi);
            }

            // Special case when centers coincide and radii are equal
            if sign((self.sectors[i].center - self.sectors[idx].center).dist()) == 0
                && sign(self.sectors[i].radius - self.sectors[idx].radius) == 0
            {
                // Adjust the angles of the other sector to the current sector's range
                for mut phi in [
                    self.sectors[i].angle_start - std::f64::consts::TAU,
                    self.sectors[i].angle_start + self.sectors[i].angle_central
                        - std::f64::consts::TAU,
                ] {
                    while sign(self.sectors[idx].angle_start - phi) > 0 {
                        phi += std::f64::consts::TAU;
                    }

                    if sign(phi - angle_end) <= 0 {
                        candidates.push(phi);
                    }
                }
            }

            // Sort the candidate angles
            candidates.sort_by(|a, b| a.partial_cmp(b).unwrap());

            // Create intervals (arcs) between consecutive candidates
            let mut arcs = Vec::new();

            for j in 0..candidates.len() - 1 {
                let phi = (candidates[j] + candidates[j + 1]) / 2.0;
                let point = self.sectors[idx].center
                    + Point::new(phi.cos(), phi.sin()) * self.sectors[idx].radius;

                let sign1 = sign((self.sectors[i].center - self.sectors[idx].center).dist());
                let sign2 = sign(self.sectors[i].radius - self.sectors[idx].radius);

                // If the midpoint is outside the other sector, or if the other sector is prior and overlaps, then exclude this arc
                if !self.sectors[i].valid(&point) || (i < idx && sign1 == 0 && sign2 == 0) {
                    arcs.push((candidates[j], candidates[j + 1]));
                }
            }

            // Add arcs to the event list
            for arc in arcs {
                intersect_arcs.push((arc.0, 1));
                intersect_arcs.push((arc.1, -1));
            }
        }

        // Sort the event points by angle
        intersect_arcs.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());

        let mut cnt_event = 0;
        let mut ret = 0.0;

        // Sweep through the angles to compute the arc area
        for i in 0..intersect_arcs.len() {
            cnt_event += intersect_arcs[i].1;

            if cnt_event == self.n as i64 {
                let phi1 = intersect_arcs[i].0;
                let phi2 = intersect_arcs[i + 1].0;

                // Compute the sector area using the formula for the sector of a circle
                ret += self.sectors[idx].radius
                    * (self.sectors[idx].center.x * (phi2.sin() - phi1.sin())
                        - self.sectors[idx].center.y * (phi2.cos() - phi1.cos())
                        + self.sectors[idx].radius * (phi2 - phi1));
            }
        }

        ret
    }

    // Compute the total area of the union of circular sectors
    fn area(&self) -> f64 {
        let mut ret = 0.0;

        // Sum the areas contributed by each sector
        for idx in 0..self.n {
            ret += self.area_segment(idx);
            ret += self.area_arc(idx);
        }

        ret / 2.0
    }
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    loop {
        let line = scan.line().trim().to_string();

        if line.is_empty() {
            break;
        }

        let n = line.parse::<usize>().unwrap();
        let mut circular_sectors = Vec::with_capacity(n);

        for _ in 0..n {
            let (x, y, r, s, theta) = (
                scan.token::<f64>(),
                scan.token::<f64>(),
                scan.token::<f64>(),
                scan.token::<f64>(),
                scan.token::<f64>(),
            );
            let sector = CircularSector::new(Point::new(x, y), r, s, theta);

            circular_sectors.push(sector);
        }

        let union = CircularSectorUnion::new(n, circular_sectors);
        writeln!(out, "{:.12}", union.area()).unwrap();
    }
}
