use io::Write;
use std::cmp::Ordering;
use std::collections::BTreeSet;
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

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct Point {
    x: i64,
    y: i64,
}

impl Point {
    fn new(x: i64, y: i64) -> Self {
        Self { x, y }
    }
}

static mut T: i64 = 0;

#[allow(dead_code)]
#[derive(Clone)]
struct Segment {
    idx: usize,
    p: Point,
    q: Point,
}

impl Segment {
    fn new(idx: usize, p: Point, q: Point) -> Self {
        Self { idx, p, q }
    }

    fn eval(&self, x: i64) -> f64 {
        let dx = self.q.x - self.p.x;
        let dy = self.q.y - self.p.y;

        if dx == 0 {
            self.p.y as f64
        } else {
            (dy as f64) / (dx as f64) * ((x - self.p.x) as f64) + (self.p.y as f64)
        }
    }
}

impl Eq for Segment {}

impl PartialEq for Segment {
    fn eq(&self, other: &Self) -> bool {
        self.p == other.p && self.q == other.q
    }
}

impl Ord for Segment {
    fn cmp(&self, other: &Self) -> Ordering {
        let t = self.eval(unsafe { T }) - other.eval(unsafe { T });

        if t.abs() <= 1e-6 {
            self.idx.cmp(&other.idx)
        } else {
            t.partial_cmp(&0.0).unwrap()
        }
    }
}

impl PartialOrd for Segment {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Event {
    idx: usize,
    direction: i32,
    x: i64,
    y: i64,
}

impl Event {
    fn new(idx: usize, direction: i32, x: i64, y: i64) -> Self {
        Self {
            idx,
            direction,
            x,
            y,
        }
    }
}

impl Ord for Event {
    fn cmp(&self, other: &Self) -> Ordering {
        self.x
            .cmp(&other.x)
            .then(self.direction.cmp(&other.direction))
            .then(self.y.cmp(&other.y))
    }
}

impl PartialOrd for Event {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn ccw(a: &Point, b: &Point, c: &Point) -> i32 {
    let ret = (b.x - a.x) as f64 * (c.y - b.y) as f64 - (c.x - b.x) as f64 * (b.y - a.y) as f64;

    if ret.abs() <= 1e-6 {
        0
    } else if ret > 0.0 {
        1
    } else {
        -1
    }
}

fn is_cross(a: &Segment, b: &Segment) -> bool {
    let t1 = ccw(&a.p, &a.q, &b.p) * ccw(&a.p, &a.q, &b.q);
    let t2 = ccw(&b.p, &b.q, &a.p) * ccw(&b.p, &b.q, &a.q);

    if t1 < 0 && t2 < 0 {
        true
    } else if t1 == 0 && t2 == 0 {
        b.p <= a.q && a.p <= b.q
    } else {
        t1 <= 0 && t2 <= 0
    }
}

fn is_intersect(segments: &[Segment]) -> bool {
    let mut events = Vec::with_capacity(segments.len() * 2);

    for (idx, segment) in segments.iter().enumerate() {
        events.push(Event::new(idx, 0, segment.p.x, segment.p.y));
        events.push(Event::new(idx, 1, segment.q.x, segment.q.y));
    }

    events.sort();

    let mut active: BTreeSet<Segment> = BTreeSet::new();

    for event in events {
        unsafe {
            T = event.x;
        }

        if event.direction == 0 {
            if !active.insert(segments[event.idx].clone()) {
                return true;
            }

            if let Some(p) = active.range(..segments[event.idx].clone()).last() {
                if is_cross(p, &segments[event.idx]) {
                    return true;
                }
            }

            if let Some(p) = active.range(segments[event.idx].clone()..).skip(1).next() {
                if is_cross(p, &segments[event.idx]) {
                    return true;
                }
            }
        } else {
            if let Some(p) = active.range(..segments[event.idx].clone()).last() {
                if let Some(q) = active.range(segments[event.idx].clone()..).skip(1).next() {
                    if is_cross(p, q) {
                        return true;
                    }
                }
            }

            active.remove(&segments[event.idx]);
        }
    }

    false
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let n = scan.token::<usize>();
    let mut points = Vec::with_capacity(n);
    let mut slopes: BTreeSet<i64> = BTreeSet::new();
    let mut segments = Vec::with_capacity(n);

    for _ in 0..n {
        let (x1, y1, x2, y2) = (
            scan.token::<i64>(),
            scan.token::<i64>(),
            scan.token::<i64>(),
            scan.token::<i64>(),
        );

        points.push((Point::new(x1, y1), Point::new(x2, y2)));

        if x1 != x2 {
            let slope = (y2 - y1) / (x2 - x1);
            slopes.insert(slope);
        }
    }

    let mut k = 0;

    while slopes.contains(&k) {
        k += 1;
    }

    for i in 0..n {
        let (p, q) = points[i];

        let x1 = k * p.x - p.y;
        let y1 = p.x + k * p.y;
        let x2 = k * q.x - q.y;
        let y2 = q.x + k * q.y;

        let mut p = Point::new(x1, y1);
        let mut q = Point::new(x2, y2);

        if p > q {
            std::mem::swap(&mut p, &mut q);
        }

        segments.push(Segment::new(i, p, q));
    }

    writeln!(out, "{}", if is_intersect(&segments) { 1 } else { 0 }).unwrap();
}
