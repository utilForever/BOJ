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

#[derive(Clone, Copy)]
struct Point {
    x: f64,
    y: f64,
}

impl Point {
    fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }

    fn cross(&self, other: Point) -> f64 {
        self.x * other.y - self.y * other.x
    }

    fn cross2(&self, a: Point, b: Point) -> f64 {
        (a.x - self.x) * (b.y - self.y) - (a.y - self.y) * (b.x - self.x)
    }
}

#[derive(Clone, Copy)]
struct Line {
    s: Point,
    t: Point,
}

impl Line {
    fn new(s: Point, t: Point) -> Self {
        Self { s, t }
    }
}

fn is_equal(a: f64, b: f64) -> bool {
    (a - b).abs() < EPS
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
    let mut v = Point::new(0.0, 0.0);

    if !is_line_intersect(&a.s, &a.t, &b.s, &b.t, &mut v) {
        return false;
    }

    let cross = c.s.cross2(c.t, v);

    cross < 0.0 || is_equal(cross, 0.0)
}

fn on_left(l: &Line, p: &Point) -> bool {
    l.s.cross2(l.t, *p) >= -EPS
}

fn same_dir(a: &Line, b: &Line) -> bool {
    let ax = a.t.x - a.s.x;
    let ay = a.t.y - a.s.y;
    let bx = b.t.x - b.s.x;
    let by = b.t.y - b.s.y;

    is_equal(ax * by - ay * bx, 0.0) && ax * bx + ay * by > 0.0
}

fn get_half_plane_intersection(lines: &mut Vec<Line>) -> Vec<Point> {
    let sign = |l: &Line| {
        if l.s.y == l.t.y {
            l.s.x > l.t.x
        } else {
            l.s.y > l.t.y
        }
    };

    lines.sort_unstable_by(|a, b| {
        let sign_a = sign(a);
        let sign_b = sign(b);

        if sign_a != sign_b {
            return sign_a.cmp(&sign_b);
        } else {
            ((a.t.y - a.s.y) * (b.t.x - b.s.x))
                .partial_cmp(&((a.t.x - a.s.x) * (b.t.y - b.s.y)))
                .unwrap()
        }
    });

    let mut filtered = Vec::new();

    for &line in lines.iter() {
        if filtered.is_empty() {
            filtered.push(line);
            continue;
        }

        if same_dir(filtered.last().unwrap(), &line) {
            if on_left(filtered.last().unwrap(), &line.s) {
                *filtered.last_mut().unwrap() = line;
            }
        } else {
            filtered.push(line);
        }
    }

    let mut queue = VecDeque::new();

    for line in filtered {
        while queue.len() >= 2 && is_bad(&queue[queue.len() - 2], &queue[queue.len() - 1], &line) {
            queue.pop_back();
        }

        while queue.len() >= 2 && is_bad(&queue[0], &queue[1], &line) {
            queue.pop_front();
        }

        if queue.len() < 2 || !is_bad(&queue[queue.len() - 1], &line, &queue[0]) {
            queue.push_back(line);
        }
    }

    let mut ret = Vec::new();

    if queue.len() >= 3 {
        for i in 0..queue.len() {
            let mut p = Point::new(0.0, 0.0);
            let j = (i + 1) % queue.len();

            if !is_line_intersect(&queue[i].s, &queue[i].t, &queue[j].s, &queue[j].t, &mut p) {
                continue;
            }

            ret.push(p);
        }
    }

    ret
}

fn polygon_area(polygon: &Vec<Point>) -> f64 {
    if polygon.len() < 3 {
        return 0.0;
    }

    let mut area = 0.0;

    for i in 0..polygon.len() {
        let j = (i + 1) % polygon.len();
        area += polygon[i].cross(polygon[j]);
    }

    area.abs() * 0.5
}

const EPS: f64 = 1e-9;

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let n = scan.token::<usize>();
    let mut points = Vec::with_capacity(n);

    for _ in 0..n {
        points.push(Point::new(scan.token::<f64>(), scan.token::<f64>()));
    }

    let mut lines = Vec::with_capacity(n);

    // Clockwise order
    for i in 0..n {
        lines.push(Line::new(points[(i + 1) % n], points[i]));
    }

    let intersection = get_half_plane_intersection(&mut lines);

    writeln!(out, "{:.10}", polygon_area(&intersection)).unwrap();
}
