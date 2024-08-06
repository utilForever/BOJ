use io::Write;
use std::{collections::VecDeque, io, str};

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

#[derive(Clone)]
struct Point {
    x: f64,
    y: f64,
}

struct Line {
    s: Point,
    t: Point,
}

fn is_equal(a: f64, b: f64) -> bool {
    (a - b).abs() < 1e-9
}

fn is_line_intersect(s1: &Point, e1: &Point, s2: &Point, e2: &Point, v: &mut Point) -> bool {
    let (vx1, vy1) = (e1.x - s1.x, e1.y - s1.y);
    let (vx2, vy2) = (e2.x - s2.x, e2.y - s2.y);
    let det = vx1 * (-vy2) - (-vx2) * vy1;

    if is_equal(det, 0.0) {
        return false;
    }

    let s = ((s2.x - s1.x) * (-vy2) + (s2.y - s1.y) * vx2) / det;

    v.x = s1.x + vx1 * s;
    v.y = s1.y + vy1 * s;

    true
}

fn is_bad(a: &Line, b: &Line, c: &Line) -> bool {
    let mut v = Point { x: 0.0, y: 0.0 };

    if !is_line_intersect(&a.s, &a.t, &b.s, &b.t, &mut v) {
        return false;
    }

    let crs = (c.t.x - c.s.x) * (v.y - c.s.y) - (c.t.y - c.s.y) * (v.x - c.s.x);

    crs < 0.0 || is_equal(crs, 0.0)
}

fn get_half_plane_intersection(lines: &mut Vec<Line>) -> Vec<Point> {
    let lsgn = |l: &Line| {
        if l.s.y == l.t.y {
            l.s.x > l.t.x
        } else {
            l.s.y > l.t.y
        }
    };

    lines.sort_by(|a, b| {
        let lsgn_a = lsgn(a);
        let lsgn_b = lsgn(b);

        if lsgn_a != lsgn_b {
            return lsgn_a.cmp(&lsgn_b);
        } else {
            ((a.t.y - a.s.y) * (b.t.x - b.s.x))
                .partial_cmp(&((a.t.x - a.s.x) * (b.t.y - b.s.y)))
                .unwrap()
        }
    });

    let mut queue = VecDeque::new();

    for i in 0..lines.len() {
        while queue.len() >= 2 && is_bad(queue[queue.len() - 2], queue[queue.len() - 1], &lines[i])
        {
            queue.pop_back();
        }

        while queue.len() >= 2 && is_bad(queue[0], queue[1], &lines[i]) {
            queue.pop_front();
        }

        if queue.len() < 2 || !is_bad(queue[queue.len() - 1], &lines[i], queue[0]) {
            queue.push_back(&lines[i]);
        }
    }

    let mut ret = Vec::new();

    if queue.len() >= 3 {
        for i in 0..queue.len() {
            let mut p = Point { x: 0.0, y: 0.0 };
            let j = (i + 1) % queue.len();

            if !is_line_intersect(&queue[i].s, &queue[i].t, &queue[j].s, &queue[j].t, &mut p) {
                continue;
            }

            ret.push(p);
        }
    }

    ret
}

fn cross(p: Point, q: Point) -> f64 {
    let res = p.x * q.y - p.y * q.x;

    if res > 0.0 {
        1.0
    } else if res < 0.0 {
        -1.0
    } else {
        0.0
    }
}

fn get_dist(p1: &Point, p2: &Point) -> f64 {
    ((p1.x - p2.x).powi(2) + (p1.y - p2.y).powi(2)).sqrt()
}

fn process_rolling_calipers(mut points: Vec<Point>) -> f64 {
    points.sort_by(|a, b| {
        if a.y == b.y {
            a.x.partial_cmp(&b.x).unwrap()
        } else {
            a.y.partial_cmp(&b.y).unwrap()
        }
    });

    let point_start = points[0].clone();

    points[1..].sort_by(|a, b| {
        let p1 = Point {
            x: a.x - point_start.x,
            y: a.y - point_start.y,
        };
        let p2 = Point {
            x: b.x - point_start.x,
            y: b.y - point_start.y,
        };

        if p1.y * p2.x != p1.x * p2.y {
            (p1.y * p2.x).partial_cmp(&(p1.x * p2.y)).unwrap()
        } else {
            if p1.y == p2.y {
                p1.x.partial_cmp(&p2.x).unwrap()
            } else {
                p1.y.partial_cmp(&p2.y).unwrap()
            }
        }
    });

    points.reverse();

    let mut num_points = 0;
    let mut ret = 0.0_f64;

    for i in 0..points.len() {
        while num_points + 1 < points.len() {
            let p1 = Point {
                x: points[i + 1].x - points[i].x,
                y: points[i + 1].y - points[i].y,
            };
            let p2 = Point {
                x: points[num_points + 1].x - points[num_points].x,
                y: points[num_points + 1].y - points[num_points].y,
            };

            if cross(p1, p2) == 1.0 {
                break;
            }

            ret = ret.max(get_dist(&points[i], &points[num_points]));
            num_points += 1;
        }

        ret = ret.max(get_dist(&points[i], &points[num_points]));
    }

    ret
}

// Reference: https://www.secmem.org/blog/2019/09/17/Half-Plane-Intersection/
fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let n = scan.token::<usize>();
    let mut vertices = Vec::new();

    for _ in 0..n {
        let (x, y) = (scan.token::<i64>(), scan.token::<i64>());
        vertices.push(Point {
            x: x as f64,
            y: y as f64,
        });
    }

    let mut directions = Vec::new();

    for i in 0..n {
        let mut point = Point {
            x: vertices[(i + 1) % n].x - vertices[i].x,
            y: vertices[(i + 1) % n].y - vertices[i].y,
        };
        let dist = get_dist(&vertices[i], &vertices[(i + 1) % n]);

        point.x /= dist;
        point.y /= dist;

        directions.push((-point.y, point.x));
    }

    let mut left = 0_i64;
    let mut right = 200_000_000_000_i64;
    let mut ret = 0.0_f64;

    while left < right {
        let mut lines = Vec::new();
        let mid = (left + right) / 2;
        let r = mid as f64 / 10000.0;

        for i in 0..n {
            lines.push(Line {
                s: Point {
                    x: vertices[i].x + r * directions[i].0,
                    y: vertices[i].y + r * directions[i].1,
                },
                t: Point {
                    x: vertices[(i + 1) % n].x + r * directions[i].0,
                    y: vertices[(i + 1) % n].y + r * directions[i].1,
                },
            });
        }

        let intersections = get_half_plane_intersection(&mut lines);

        if intersections.len() > 0 {
            let dist_max = process_rolling_calipers(intersections);

            if dist_max >= 2.0 * r {
                left = mid + 1;
                ret = ret.max(r);
            } else {
                right = mid - 1;
            }
        } else {
            right = mid - 1;
        }
    }

    writeln!(out, "{:.3}", ret).unwrap();
}
