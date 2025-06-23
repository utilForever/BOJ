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
    pub fn cross(&self, other: &Point) -> f64 {
        self.x * other.y - self.y * other.x
    }

    #[inline(always)]
    pub fn cross2(&self, p1: &Point, p2: &Point) -> f64 {
        (*p1 - *self).cross(&(*p2 - *self))
    }

    #[inline(always)]
    fn intersect_segment(a: &Point, b: &Point, c: &Point, d: &Point) -> Option<Point> {
        let oa = c.cross2(d, a);
        let ob = c.cross2(d, b);
        let oc = a.cross2(b, c);
        let od = a.cross2(b, d);

        if sign(oa) * sign(ob) < 0 && sign(oc) * sign(od) < 0 {
            Some((*a * ob - *b * oa) / (ob - oa))
        } else {
            None
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
enum State {
    // The current vertex is visible; stack already updated
    Visible,
    // Hidden by a front arc (apex index, still a gap?)
    HiddenFront(usize, bool),
    // Hidden by a back arc (the sign of the occulting turn)
    HiddenBack(bool),
}

#[inline(always)]
fn ccw(p: &[Point], a: usize, b: usize, c: usize) -> f64 {
    p[a].cross2(&p[b], &p[c])
}

#[inline(always)]
fn prune_stack(stack: &mut Vec<usize>, points: &[Point], curr: usize, next: usize) {
    // Cond 1: last becomes invisible from origin once `next` is added.
    // Cond 2: triangle (curr, next, last) makes the hull locally right‑turn
    while let Some(&last) = stack.last() {
        if ccw(points, 0, last, next) <= 0.0 && ccw(points, curr, next, last) > 0.0 {
            stack.pop();
        } else {
            break;
        }
    }
}

fn process_visible(
    points: &[Point],
    stack: &mut Vec<usize>,
    prev: usize,
    curr: usize,
    next: usize,
) -> State {
    if ccw(points, 0, *stack.last().unwrap(), next) > 0.0 {
        // Case 1: next is on the left of the current radial hull ray
        if ccw(points, 0, prev, curr) < 0.0 && ccw(points, prev, curr, next) > 0.0 {
            State::HiddenBack(true)
        } else {
            stack.push(next);
            State::Visible
        }
    } else {
        // Case 2: next is on or to the right: hull may need pruning
        if ccw(points, 0, prev, curr) > 0.0 && ccw(points, prev, curr, next) < 0.0 {
            State::HiddenBack(false)
        } else {
            stack.pop();
            prune_stack(stack, points, curr, next);

            if ccw(points, curr, next, *stack.last().unwrap()) < 0.0 {
                // Next hides behind the front arc
                State::HiddenFront(next, false)
            } else {
                stack.push(next);
                State::Visible
            }
        }
    }
}

fn process_hidden_front(
    points: &[Point],
    stack: &mut Vec<usize>,
    prev: usize,
    curr: usize,
    next: usize,
    apex: usize,
    gap: bool,
) -> State {
    // Check if the segment (curr,next) pierces the gap between the last hull vertex and the front apex
    let mut gap = gap
        || Point::intersect_segment(
            &points[*stack.last().unwrap()],
            &points[apex],
            &points[curr],
            &points[next],
        )
        .is_some();

    // Close the gap if the front arc bends away from the hull
    if ccw(points, *stack.last().unwrap(), apex, next) < 0.0
        || (apex == curr && ccw(points, prev, curr, next) > 0.0)
    {
        gap = false;
    }

    if ccw(points, 0, *stack.last().unwrap(), next) > 0.0 && gap {
        // Gap is still open and visible: next becomes visible
        stack.push(next);
        State::Visible
    } else {
        // Still hidden in front
        State::HiddenFront(apex, gap)
    }
}

fn process_hidden_back(
    points: &[Point],
    stack: &mut Vec<usize>,
    curr: usize,
    next: usize,
    is_positive: bool,
) -> State {
    // Check whether the occulting back arc has ended
    let crossed = (ccw(points, 0, *stack.last().unwrap(), next) > 0.0) ^ is_positive;

    if !crossed {
        return State::HiddenBack(is_positive);
    }

    if is_positive {
        // Back arc finished with a positive turn: prune then possibly front‑hide
        stack.pop();
        prune_stack(stack, points, curr, next);

        if ccw(points, curr, next, *stack.last().unwrap()) < 0.0 {
            State::HiddenFront(next, false)
        } else {
            stack.push(next);
            State::Visible
        }
    } else {
        // Negative turn: next becomes immediately visible
        stack.push(next);
        State::Visible
    }
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let n = scan.token::<usize>();
    let mut points = Vec::with_capacity(n + 1);

    points.push(Point::new(0.0, 0.0));

    for _ in 0..n {
        points.push(Point::new(scan.token::<f64>(), scan.token::<f64>()));
    }

    let mut first = 1;

    for i in 2..=n {
        let dir = points[0].cross2(&points[first], &points[i]);

        if dir < 0.0 || (dir == 0.0 && points[i].x < points[first].x) {
            first = i;
        }
    }

    let prev_first = if first == 1 { n } else { first - 1 };
    let next_first = if first == n { 1 } else { first + 1 };
    let dir = if points[next_first].cross2(&points[first], &points[prev_first]) > 0.0 {
        1.0
    } else {
        -1.0
    };

    let advance = |idx: usize, dir: f64| -> usize {
        let step = if dir > 0.0 { 1 } else { n - 1 };
        (idx - 1 + step) % n + 1
    };

    let mut stack: Vec<usize> = Vec::new();
    let mut curr = advance(first, dir);
    stack.push(first);
    stack.push(curr);

    let mut state = State::Visible;

    for _ in 2..n {
        let prev = advance(curr, -dir);
        let next = advance(curr, dir);

        state = match state {
            State::Visible => process_visible(&points, &mut stack, prev, curr, next),
            State::HiddenFront(apex, gap) => {
                process_hidden_front(&points, &mut stack, prev, curr, next, apex, gap)
            }
            State::HiddenBack(pos) => process_hidden_back(&points, &mut stack, curr, next, pos),
        };

        curr = next;
    }

    let mut ret = vec![false; n + 1];

    for idx in stack {
        ret[idx] = true;
    }

    writeln!(out, "{}", ret.iter().filter(|&&x| x).count()).unwrap();

    for i in 1..=n {
        if ret[i] {
            write!(out, "{i} ").unwrap();
        }
    }

    writeln!(out).unwrap();
}
