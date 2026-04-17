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

#[derive(Clone, Copy)]
struct Constraint {
    a: f64,
    b: f64,
    c: f64,
}

impl Constraint {
    fn new(a: i64, b: i64, c: i64) -> Self {
        Self {
            a: a as f64,
            b: b as f64,
            c: c as f64,
        }
    }

    fn reversed(&self) -> Self {
        Self {
            a: -self.a,
            b: -self.b,
            c: -self.c,
        }
    }

    fn eval(&self, p: Point) -> f64 {
        self.a * p.x + self.b * p.y + self.c
    }

    fn to_line(&self) -> Option<Line> {
        if is_equal(self.a, 0.0) && is_equal(self.b, 0.0) {
            return None;
        }

        let p = if !is_equal(self.a, 0.0) {
            Point::new(-self.c / self.a, 0.0)
        } else {
            Point::new(0.0, -self.c / self.b)
        };

        Some(Line::new(p, Point::new(p.x + self.b, p.y - self.a)))
    }
}

fn count_weights(weights: &[char]) -> (i64, i64, i64) {
    let mut cnt_a = 0;
    let mut cnt_b = 0;
    let mut cnt_one = 0;

    for &token in weights {
        match token {
            'A' => cnt_a += 1,
            'B' => cnt_b += 1,
            '1' => cnt_one += 1,
            _ => {}
        }
    }

    (cnt_a, cnt_b, cnt_one)
}

fn parse_experiment(expriment: &Vec<char>) -> Constraint {
    let mut pos = 0;

    while expriment[pos] != '<' && expriment[pos] != '>' {
        pos += 1;
    }

    let left = count_weights(&expriment[..pos]);
    let right = count_weights(&expriment[pos + 1..expriment.len() - 1]);

    let mut a = left.0 - right.0;
    let mut b = left.1 - right.1;
    let mut c = left.2 - right.2;

    if expriment[pos] == '<' {
        a = -a;
        b = -b;
        c = -c;
    }

    Constraint::new(a, b, c)
}

fn parse_query(tokens: &Vec<char>) -> Constraint {
    let pos = tokens.iter().position(|&x| x == '|').unwrap();
    let left = count_weights(&tokens[..pos]);
    let right = count_weights(&tokens[pos + 1..tokens.len() - 1]);

    Constraint::new(left.0 - right.0, left.1 - right.1, left.2 - right.2)
}

fn check(base: &Vec<Constraint>, extra: Constraint) -> bool {
    let mut constraints = Vec::with_capacity(base.len() + 3);
    constraints.extend_from_slice(base);

    constraints.push(Constraint {
        a: 1.0,
        b: 0.0,
        c: 0.0,
    });
    constraints.push(Constraint {
        a: 0.0,
        b: 1.0,
        c: 0.0,
    });
    constraints.push(extra);

    let mut lines = Vec::new();

    for &constraint in constraints.iter() {
        if is_equal(constraint.a, 0.0) && is_equal(constraint.b, 0.0) {
            if constraint.c <= EPS {
                return false;
            }
        } else {
            lines.push(constraint.to_line().unwrap());
        }
    }

    lines.push(
        Constraint {
            a: -1.0,
            b: 0.0,
            c: BOUND,
        }
        .to_line()
        .unwrap(),
    );
    lines.push(
        Constraint {
            a: 0.0,
            b: -1.0,
            c: BOUND,
        }
        .to_line()
        .unwrap(),
    );

    let intersection = get_half_plane_intersection(&mut lines);

    if intersection.len() < 3 {
        return false;
    }

    let mut center = Point::new(0.0, 0.0);

    for point in intersection.iter() {
        center.x += point.x;
        center.y += point.y;
    }

    center.x /= intersection.len() as f64;
    center.y /= intersection.len() as f64;

    for constraint in constraints {
        if constraint.eval(center) <= EPS {
            return false;
        }
    }

    true
}

const EPS: f64 = 1e-9;
const BOUND: f64 = 1e9;

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let k = scan.token::<usize>();
    let mut experiments = Vec::with_capacity(k);

    for _ in 0..k {
        let line = scan.line().trim().chars().collect::<Vec<_>>();
        experiments.push(parse_experiment(&line));
    }

    for _ in 0..5 {
        let line = scan.line().trim().chars().collect::<Vec<_>>();
        let query = parse_query(&line);

        let ret_left = check(&experiments, query);
        let ret_right = check(&experiments, query.reversed());

        writeln!(
            out,
            "{}",
            match (ret_left, ret_right) {
                (true, false) => ">",
                (false, true) => "<",
                _ => "?",
            }
        )
        .unwrap();
    }
}
