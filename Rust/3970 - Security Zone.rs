use io::Write;
use std::{cmp::Ordering, collections::BTreeSet, io, ops::Sub, str};
use Ordering::{Equal, Greater, Less};

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
struct Disc {
    center: (f64, f64),
    radius: f64,
}

impl Disc {
    pub fn new(center: (f64, f64), radius: f64) -> Self {
        Self { center, radius }
    }

    pub fn exist_common_support_line(&self, other: &Disc) -> bool {
        let (x1, y1, r1) = (self.center.0, self.center.1, self.radius);
        let (x2, y2, r2) = (other.center.0, other.center.1, other.radius);
        let square = (x1 - x2).powi(2) + (y1 - y2).powi(2);

        square > (r1 - r2).powi(2)
    }

    pub fn compute_common_support_line_to(&self, other: &Disc) -> Edge {
        let (x1, y1, r1) = (self.center.0, self.center.1, self.radius);
        let (x2, y2, r2) = (other.center.0, other.center.1, other.radius);

        let square = (x1 - x2).powi(2) + (y1 - y2).powi(2);
        let d = square.sqrt();
        let vx = (x2 - x1) / d;
        let vy = (y2 - y1) / d;

        let c = (r1 - r2) / d;
        let h = (1.0 - c.powi(2)).max(0.0).sqrt();

        let nx = vx * c - vy * h;
        let ny = vx * h + vy * c;

        Edge {
            p1: Point {
                x: x1 + r1 * nx,
                y: y1 + r1 * ny,
            },
            p2: Point {
                x: x2 + r2 * nx,
                y: y2 + r2 * ny,
            },
        }
    }

    pub fn compute_parallel_support_line(&self, line: &Edge) -> Edge {
        let line_unit = line.normalize().unit();
        let p = Point {
            x: self.center.0 - self.radius * (line_unit.p2.x - line_unit.p1.x),
            y: self.center.1 - self.radius * (line_unit.p2.y - line_unit.p1.y),
        };
        let od = Edge {
            p1: Point { x: p.x, y: p.y },
            p2: Point {
                x: self.center.0,
                y: self.center.1,
            },
        }
        .normalize();

        Edge {
            p1: od.p2,
            p2: od.p1,
        }
    }

    fn calculate_ccw(&self, p1: Point, p2: Point, p3: Point) -> i64 {
        let (x1, y1) = (p1.x as i64, p1.y as i64);
        let (x2, y2) = (p2.x as i64, p2.y as i64);
        let (x3, y3) = (p3.x as i64, p3.y as i64);

        let res = (x2 - x1) * (y3 - y1) - (x3 - x1) * (y2 - y1);
        if res > 0 {
            1
        } else if res < 0 {
            -1
        } else {
            0
        }
    }

    fn get_points(&self, other: &Disc, points: &mut Vec<Point>, val: f64) {
        let sign = |val: f64| -> i64 {
            if val.abs() < 1e-10 {
                0
            } else if val > 0.0 {
                1
            } else {
                -1
            }
        };
        let calculate_ab = |x1: f64, y1: f64, x2: f64, y2: f64| -> Point {
            let a = (y1 - y2) / (x1 - x2);
            let b = x1 * a + y1;

            Point { x: a, y: b }
        };

        let (x1, y1, r1) = (self.center.0, self.center.1, self.radius);
        let (x2, y2, r2) = (other.center.0, other.center.1, other.radius);

        if val == 0.0 {
            if r1 + y1 == r2 + y2 {
                points.push(Point { x: x1, y: r1 + y1 });
                points.push(Point { x: x2, y: r2 + y2 });
            }

            if y1 - r1 == y2 - r2 {
                points.push(Point { x: x1, y: y1 - r1 });
                points.push(Point { x: x2, y: y2 - r2 });
            }

            return;
        }

        if r1 == r2 {
            let val1 = val;
            let val2 = r1 * (val * val + 1.0).sqrt() + y1 - val * x1;
            let val3 = val;
            let val4 = r2 * (val * val + 1.0).sqrt() + y2 - val * x2;
            let val5 = -1.0 / val;
            let val6 = -val5 * x1 + y1;
            let val7 = -1.0 / val;
            let val8 = -val7 * x2 + y2;

            points.push(calculate_ab(val1, val2, val5, val6));
            points.push(calculate_ab(val3, val4, val7, val8));

            let val2 = -r1 * (val * val + 1.0).sqrt() + y1 - val * x1;
            let val4 = -r2 * (val * val + 1.0).sqrt() + y2 - val * x2;

            points.push(calculate_ab(val1, val2, val5, val6));
            points.push(calculate_ab(val3, val4, val7, val8));
        }

        if sign(r1 - r2) == sign(val * (x1 - x2) - (y1 - y2)) {
            let val1 = val;
            let val2 = r1 * (val * val + 1.0).sqrt() + y1 - val * x1;
            let val3 = val;
            let val4 = r2 * (val * val + 1.0).sqrt() + y2 - val * x2;
            let val5 = -1.0 / val;
            let val6 = -val5 * x1 + y1;
            let val7 = -1.0 / val;
            let val8 = -val7 * x2 + y2;

            points.push(calculate_ab(val1, val2, val5, val6));
            points.push(calculate_ab(val3, val4, val7, val8));
        }

        if sign(r2 - r1) == sign(val * (x1 - x2) - (y1 - y2)) {
            let val1 = val;
            let val2 = -r1 * (val * val + 1.0).sqrt() + y1 - val * x1;
            let val3 = val;
            let val4 = -r2 * (val * val + 1.0).sqrt() + y2 - val * x2;
            let val5 = -1.0 / val;
            let val6 = -val5 * x1 + y1;
            let val7 = -1.0 / val;
            let val8 = -val7 * x2 + y2;

            points.push(calculate_ab(val1, val2, val5, val6));
            points.push(calculate_ab(val3, val4, val7, val8));
        }
    }

    pub fn calculate_angle(&self, other: &Disc) -> (f64, Vec<Point>) {
        let (x1, y1, r1) = (self.center.0, self.center.1, self.radius);
        let (x2, y2, r2) = (other.center.0, other.center.1, other.radius);
        let mut points = Vec::new();

        if (x1 - x2).powi(2) + (y1 - y2).powi(2) <= (r1 - r2).powi(2) {
            return (f64::MAX / 2.0, Vec::new());
        }

        if x1 == x2 && y1 == y2 && r1 == r2 {
            return (f64::MAX / 2.0, Vec::new());
        }

        if x1 + r1 == x2 + r2 {
            points.push(Point { x: x1 + r1, y: y1 });
            points.push(Point { x: x2 + r2, y: y2 });
        }

        if x1 - r1 == x2 - r2 {
            points.push(Point { x: x1 - r1, y: y1 });
            points.push(Point { x: x2 - r2, y: y2 });
        }

        let val1 = (r1 - r2).powi(2) - (x1 - x2).powi(2);
        let val2 = 2.0 * (x1 - x2) * (y1 - y2);
        let val3 = (r1 - r2).powi(2) - (y1 - y2).powi(2);

        if val1 == 0.0 {
            if val2 != 0.0 {
                self.get_points(other, &mut points, -val3 / val2);
            }
        } else if val2 * val2 / 4.0 >= val1 * val3 {
            let val = (-val2 + (val2 * val2 - 4.0 * val1 * val3).sqrt()) / (2.0 * val1);
            self.get_points(other, &mut points, val);
            let val = (-val2 - (val2 * val2 - 4.0 * val1 * val3).sqrt()) / (2.0 * val1);
            self.get_points(other, &mut points, val);
        }

        while points.len() > 4 {
            points.pop();
        }

        let mut points_new = Vec::new();

        if points.len() == 4 {
            if self.calculate_ccw(points[0], points[1], points[2]) == -1 {
                points_new.push(points[0]);
                points_new.push(points[1]);
            } else {
                points_new.push(points[2]);
                points_new.push(points[3]);
            }
        } else if points.len() == 2 {
            points_new.push(points[0]);
            points_new.push(points[1]);
        }

        points = points_new;

        if points[0].x == points[1].x {
            if points[0].y < points[1].y {
                return (1.5 * std::f64::consts::PI, points);
            } else {
                return (0.5 * std::f64::consts::PI, points);
            }
        }

        if points[0].y == points[1].y {
            if points[0].x < points[1].x {
                return (2.0 * std::f64::consts::PI, points);
            } else {
                return (std::f64::consts::PI, points);
            }
        }

        let val = ((points[1].y - points[0].y) / (points[1].x - points[0].x)).abs();

        if points[0].x < points[1].x && points[0].y < points[1].y {
            (2.0 * std::f64::consts::PI - val.atan(), points)
        } else if points[0].x > points[1].x && points[0].y < points[1].y {
            (std::f64::consts::PI + val.atan(), points)
        } else if points[0].x < points[1].x && points[0].y > points[1].y {
            (val.atan(), points)
        } else {
            (std::f64::consts::PI - val.atan(), points)
        }
    }
}

impl Eq for Disc {}

impl PartialEq for Disc {
    fn eq(&self, other: &Self) -> bool {
        self.center == other.center && self.radius == other.radius
    }
}

impl Ord for Disc {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.radius < other.radius {
            Less
        } else if self.radius > other.radius {
            Greater
        } else {
            Equal
        }
    }
}

impl PartialOrd for Disc {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.radius < other.radius {
            Some(Less)
        } else if self.radius > other.radius {
            Some(Greater)
        } else {
            Some(Equal)
        }
    }
}

#[derive(Copy, Clone, Debug, Default)]
struct Point {
    x: f64,
    y: f64,
}

impl Sub for Point {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

#[derive(Clone, Debug, Default)]
struct Edge {
    p1: Point,
    p2: Point,
}

fn radians_to_degrees(radians: f64) -> f64 {
    radians * 180.0 / std::f64::consts::PI
}

impl Edge {
    fn normalize(&self) -> Self {
        let (x1, y1) = (self.p1.x, self.p1.y);
        let (x2, y2) = (self.p2.x, self.p2.y);
        let dx = x2 - x1;
        let dy = y2 - y1;
        let d = (dx * dx + dy * dy).sqrt();

        Self {
            p1: Point {
                x: x1 / d,
                y: y1 / d,
            },
            p2: Point {
                x: x2 / d,
                y: y2 / d,
            },
        }
    }

    fn unit(&self) -> Self {
        let (x1, y1) = (self.p1.x, self.p1.y);
        let (x2, y2) = (self.p2.x, self.p2.y);
        let dx = x2 - x1;
        let dy = y2 - y1;
        let d = (dx * dx + dy * dy).sqrt();

        Self {
            p1: Point { x: 0.0, y: 0.0 },
            p2: Point {
                x: dx / d,
                y: dy / d,
            },
        }
    }

    fn angle(&self) -> f64 {
        let dx = self.p2.x - self.p1.x;
        let dy = self.p2.y - self.p1.y;
        let theta = radians_to_degrees(-dy.atan2(dx));
        let theta_norm = if theta < 0.0 { theta + 360.0 } else { theta };

        if (theta_norm - 360.0).abs() < 1e-12 {
            0.0
        } else {
            theta_norm
        }
    }

    fn angle_to(&self, other: &Self) -> f64 {
        let a1 = self.angle();
        let a2 = other.angle();
        let delta = a2 - a1;
        let delta_norm = if delta < 0.0 { delta + 360.0 } else { delta };

        if (delta_norm - 360.0).abs() < 1e-12 {
            0.0
        } else {
            delta_norm
        }
    }
}

#[derive(Debug)]
struct ConvexHull {
    discs: Vec<Disc>,
}

impl ConvexHull {
    fn length(&self) -> f64 {
        if self.discs.len() == 1 {
            return 2.0 * std::f64::consts::PI * self.discs[0].radius;
        }

        let mut set = BTreeSet::new();
        let mut circles = Vec::new();

        for i in 0..self.discs.len() {
            let j = (i + 1) % self.discs.len();

            if set
                .iter()
                .find(|(a, b)| *a == self.discs[i] && *b == self.discs[j])
                .is_some()
            {
                break;
            }

            set.insert((self.discs[i].clone(), self.discs[j].clone()));
            circles.push(self.discs[i].clone());
        }

        println!("{:?}", self.discs);
        println!("{:?}", circles);

        let mut ret = 0.0;

        for j in 0..circles.len() {
            let i = (j + circles.len() - 1) % circles.len();
            let k = (j + 1) % circles.len();

            let (angle1, _) = circles[i].calculate_angle(&circles[j]);
            let (angle2, points2) = circles[j].calculate_angle(&circles[k]);
            let mut angle = angle2 - angle1;

            if angle < 0.0 {
                angle += 2.0 * std::f64::consts::PI;
            }

            println!(
                "i : {} {} {} {}",
                circles[i].center.0,
                circles[i].center.1,
                circles[i].radius,
                angle1 / std::f64::consts::PI * 180.0
            );
            println!(
                "j : {} {} {} {}",
                circles[j].center.0,
                circles[j].center.1,
                circles[j].radius,
                angle / std::f64::consts::PI * 180.0
            );
            println!(
                "k : {} {} {} {}",
                circles[k].center.0,
                circles[k].center.1,
                circles[k].radius,
                angle2 / std::f64::consts::PI * 180.0
            );

            ret += angle * circles[j].radius;

            if !points2.is_empty() {
                ret += (points2[0].x - points2[1].x).hypot(points2[0].y - points2[1].y);
            }
        }

        ret
    }
}

#[derive(Default, Debug)]
struct ConvexHullAlgorithm {
    discs: Vec<Disc>,
}

impl ConvexHullAlgorithm {
    pub fn add_disc(&mut self, disc: Disc) {
        self.discs.push(disc);
    }

    pub fn sort(&mut self) {
        self.discs.sort_by(|a, b| {
            if a.center.1 + a.radius == b.center.1 + b.radius {
                a.center.0.partial_cmp(&b.center.0).unwrap()
            } else {
                (a.center.1 + a.radius)
                    .partial_cmp(&(b.center.1 + b.radius))
                    .unwrap()
            }
        });
    }

    pub fn find(&self, start: usize, end: usize) -> ConvexHull {
        if start == end {
            return ConvexHull {
                discs: vec![self.discs[start].clone()],
            };
        }

        let mid = (start + end) / 2;
        println!("start: {}, end: {}, mid: {}", start, end, mid);
        let p = self.find(start, mid);
        let q = self.find(mid + 1, end);

        self.merge(&p, &q)
    }
}

impl ConvexHullAlgorithm {
    fn triangle_orientation(&self, p: &Point, q: &Point, r: &Point) -> f64 {
        (q.x - p.x) * (r.y - p.y) - (q.y - p.y) * (r.x - p.x)
    }

    fn dom(&self, line_p: &Edge, line_q: &Edge) -> bool {
        let to = self.triangle_orientation(&line_p.p1, &line_q.p1, &line_p.p2);

        if to < 0.0 {
            false
        } else if to > 0.0 {
            true
        } else {
            let direction = line_p.p2.y - line_p.p1.y;

            if direction > 0.0 {
                if line_p.p1.y > line_q.p2.y {
                    true
                } else {
                    false
                }
            } else if direction < 0.0 {
                if line_p.p1.y > line_q.p2.y {
                    false
                } else {
                    true
                }
            } else {
                let direction_x = line_p.p2.x - line_p.p1.x;

                if direction_x > 0.0 {
                    if line_p.p1.x > line_q.p2.x {
                        true
                    } else {
                        false
                    }
                } else {
                    if line_p.p1.x > line_q.p2.x {
                        false
                    } else {
                        true
                    }
                }
            }
        }
    }

    fn add(&self, list: &mut Vec<Disc>, disc: &Disc) -> bool {
        if list.is_empty() || *list.last().unwrap() != *disc {
            println!("list : {:?}", list);
            println!("add disc to list : {:?}", disc);
            list.push(disc.clone());
            println!("after add list : {:?}", list);
            true
        } else {
            false
        }
    }

    fn succ(&self, list: &Vec<Disc>, idx: usize) -> usize {
        if idx + 1 < list.len() {
            idx + 1
        } else if list.len() > 1 {
            1
        } else {
            0
        }
    }

    fn merge(&self, lch: &ConvexHull, rch: &ConvexHull) -> ConvexHull {
        let mut list = Vec::new();
        let end_p = lch.discs.len();
        let end_q = rch.discs.len();
        let mut disc_idx_p = 0;
        let mut disc_idx_q = 0;
        let mut i = 0;
        let mut j = 0;

        let mut line_star = Edge {
            p1: Point { x: 0.0, y: 0.0 },
            p2: Point { x: 0.0, y: 100.0 },
        };
        let mut line_p = lch.discs[disc_idx_p].compute_parallel_support_line(&line_star);
        let mut line_q = rch.discs[disc_idx_q].compute_parallel_support_line(&line_star);

        println!("lch : {:?}", lch);
        println!("rch : {:?}", rch);
        println!("dom : {}", self.dom(&line_p, &line_q));

        while i < end_p || j < end_q {
            if self.dom(&line_p, &line_q) {
                self.add(&mut list, &lch.discs[disc_idx_p]);
                self.advance(
                    &mut line_star,
                    &mut i,
                    &mut disc_idx_p,
                    &mut j,
                    &mut disc_idx_q,
                    &mut list,
                    &lch.discs,
                    &rch.discs,
                );
            } else {
                self.add(&mut list, &rch.discs[disc_idx_q]);
                self.advance(
                    &mut line_star,
                    &mut j,
                    &mut disc_idx_q,
                    &mut i,
                    &mut disc_idx_p,
                    &mut list,
                    &rch.discs,
                    &lch.discs,
                );
            }

            line_p = lch.discs[disc_idx_p].compute_parallel_support_line(&line_star);
            line_q = rch.discs[disc_idx_q].compute_parallel_support_line(&line_star);
        }

        if list.len() > 1 && list[0] == *list.last().unwrap() {
            list.pop();
        }

        ConvexHull { discs: list }
    }

    fn advance(
        &self,
        line_star: &mut Edge,
        idx_x: &mut usize,
        idx_disc_x: &mut usize,
        idx_y: &mut usize,
        idx_disc_y: &mut usize,
        list: &mut Vec<Disc>,
        list_x: &Vec<Disc>,
        list_y: &Vec<Disc>,
    ) {
        let mut line_x_succ = Edge::default();
        let mut line_y_succ = Edge::default();

        let mut a1 = 361.0;
        let mut a2 = 361.0;
        let mut a3 = 361.0;
        let mut a4 = 361.0;

        if list_x[*idx_disc_x].exist_common_support_line(&list_y[*idx_disc_y]) {
            let line_xy = list_x[*idx_disc_x].compute_common_support_line_to(&list_y[*idx_disc_y]);
            let line_yx = list_y[*idx_disc_y].compute_common_support_line_to(&list_x[*idx_disc_x]);
            a1 = line_star.angle_to(&line_xy);
            a4 = line_star.angle_to(&line_yx);
        }

        let x_succ = list_x[self.succ(list_x, *idx_x)].clone();
        let y_succ = list_y[self.succ(list_y, *idx_y)].clone();

        if list_x.len() == 1 && *idx_x == 0 {
            *idx_x += 1;
        } else if *idx_x < list_x.len() {
            line_x_succ = list_x[*idx_disc_x].compute_common_support_line_to(&x_succ);
            a2 = line_star.angle_to(&line_x_succ);
        }

        if list_y.len() == 1 && *idx_y == 0 {
            *idx_y += 1;
        } else if *idx_y < list_y.len() {
            line_y_succ = list_y[*idx_disc_y].compute_common_support_line_to(&y_succ);
            a3 = line_star.angle_to(&line_y_succ);
        }

        println!("a1 : {}, a2 : {}, a3 : {}, a4 : {}", a1, a2, a3, a4);

        // if a1 == a1.min(a2.min(a3)) {
        //     let disc_y = list_y[*idx_disc_y].clone();

        //     if list.is_empty() || *list.last().unwrap() != disc_y {
        //         if list.len() >= 2 && list[0] == *list.last().unwrap() && list[1] == disc_y {
        //             return;
        //         }

        //         list.push(disc_y.clone());
        //         *idx_y += 1;
        //     }
        // }

        // if a4 == a4.min(a2.min(a3)) {
        //     let disc_x = list_x[*idx_disc_x].clone();

        //     if list.is_empty() || *list.last().unwrap() != disc_x {
        //         if list.len() >= 2 && list[0] == *list.last().unwrap() && list[1] == disc_x {
        //             return;
        //         }

        //         list.push(disc_x.clone());
        //         *idx_x += 1;
        //     }
        // }

        if a1 < a4 {
            let disc_x = list_x[*idx_disc_x].clone();
            let disc_y = list_y[*idx_disc_y].clone();

            if a1 < a2.min(a3) {
                if list.is_empty() || *list.last().unwrap() != disc_y {
                    if list.len() >= 2 && list[0] == *list.last().unwrap() && list[1] == disc_y {
                        return;
                    }

                    list.push(disc_y.clone());
                    *idx_y += 1;
                }
            }

            if a4 < a2.min(a3) {
                if list.is_empty() || *list.last().unwrap() != disc_x {
                    if list.len() >= 2 && list[0] == *list.last().unwrap() && list[1] == disc_x {
                        return;
                    }

                    list.push(disc_x.clone());
                    *idx_x += 1;
                }
            }
        } else {
            let disc_x = list_x[*idx_disc_x].clone();
            let disc_y = list_y[*idx_disc_y].clone();

            if a4 < a2.min(a3) {
                if list.is_empty() || *list.last().unwrap() != disc_x {
                    if list.len() >= 2 && list[0] == *list.last().unwrap() && list[1] == disc_x {
                        return;
                    }

                    list.push(disc_x.clone());
                    *idx_x += 1;
                }
            }

            if a1 < a2.min(a3) {
                if list.is_empty() || *list.last().unwrap() != disc_y {
                    if list.len() >= 2 && list[0] == *list.last().unwrap() && list[1] == disc_y {
                        return;
                    }

                    list.push(disc_y.clone());
                    *idx_y += 1;
                }
            }
        }

        if a2 < a3 {
            *line_star = line_x_succ.clone();

            if *idx_x < list_x.len() {
                *idx_x += 1;
                *idx_disc_x = *idx_x % list_x.len();
            }
        } else {
            *line_star = line_y_succ.clone();

            if *idx_y < list_y.len() {
                *idx_y += 1;
                *idx_disc_y = *idx_y % list_y.len();
            }
        }
    }
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    // let c = scan.token::<i64>();
    let c = 1;

    for _ in 0..c {
        let n = scan.token::<usize>();
        let mut convex_hull_algorithm = ConvexHullAlgorithm::default();

        for _ in 0..n {
            let (x, y, r) = (
                scan.token::<f64>(),
                scan.token::<f64>(),
                scan.token::<f64>(),
            );
            convex_hull_algorithm.add_disc(Disc::new((x, y), r));
        }

        convex_hull_algorithm.sort();

        writeln!(out, "{:.7}", convex_hull_algorithm.find(0, n - 1).length()).unwrap();
    }
}
