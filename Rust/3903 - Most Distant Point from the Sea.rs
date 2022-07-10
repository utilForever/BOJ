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

// Reference: https://www.secmem.org/blog/2019/09/17/Half-Plane-Intersection/
fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    loop {
        let n = scan.token::<usize>();

        if n == 0 {
            break;
        }

        let mut vertices = Vec::new();

        for _ in 0..n {
            let (x, y) = (scan.token::<i64>(), scan.token::<i64>());
            vertices.push(Point {
                x: x as f64,
                y: y as f64,
            });
        }

        let mut left = 0.0;
        let mut right = 10000.0;

        while left + 1e-9 <= right {
            let mut lines = Vec::new();
            let mid = (left + right) / 2.0;

            for i in 0..n {
                let mut polar = (vertices[(i + 1) % n].y - vertices[i].y)
                    .atan2(vertices[(i + 1) % n].x - vertices[i].x);
                polar += std::f64::consts::PI / 2.0;

                lines.push(Line {
                    s: Point {
                        x: vertices[i].x + mid * polar.cos(),
                        y: vertices[i].y + mid * polar.sin(),
                    },
                    t: Point {
                        x: vertices[(i + 1) % n].x + mid * polar.cos(),
                        y: vertices[(i + 1) % n].y + mid * polar.sin(),
                    },
                });
            }

            let intersections = get_half_plane_intersection(&mut lines);

            if intersections.len() > 0 {
                left = mid;
            } else {
                right = mid;
            }
        }

        writeln!(out, "{:.6}", left).unwrap();
    }
}
