use io::Write;
use std::cmp::Ordering;
use std::collections::BTreeSet;
use std::ops::Bound::{Excluded, Unbounded};
use std::ops::{Add, Div, Mul, Sub};
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

fn xorshift32(seed: &mut u32) -> u32 {
    let mut x = *seed;

    x ^= x << 13;
    x ^= x >> 17;
    x ^= x << 5;
    *seed = x;

    x
}

#[derive(Clone, Copy, PartialEq, PartialOrd)]
struct Float64(f64);

impl Eq for Float64 {}

impl Ord for Float64 {
    fn cmp(&self, o: &Self) -> Ordering {
        self.partial_cmp(o).unwrap()
    }
}

type Tree<K, V> = Option<Box<TreeNode<K, V>>>;

#[derive(Clone)]
struct TreeNode<K: Ord + Copy, V: Copy> {
    left: Option<Box<TreeNode<K, V>>>,
    right: Option<Box<TreeNode<K, V>>>,
    low: K,
    high: K,
    high_max: K,
    val: V,
    priority: u32,
}

impl<K: Ord + Copy, V: Copy> TreeNode<K, V> {
    fn new(
        left: Option<Box<TreeNode<K, V>>>,
        right: Option<Box<TreeNode<K, V>>>,
        low: K,
        high: K,
        high_max: K,
        val: V,
        priority: u32,
    ) -> Self {
        Self {
            left,
            right,
            low,
            high,
            high_max,
            val,
            priority,
        }
    }

    fn rotate_left(mut p: Box<TreeNode<K, V>>) -> Box<TreeNode<K, V>> {
        let mut q = p.right.take().unwrap();

        p.right = q.left.take();
        p.update();
        q.left = Some(p);
        q.update();

        q
    }

    fn rotate_right(mut p: Box<TreeNode<K, V>>) -> Box<TreeNode<K, V>> {
        let mut q = p.left.take().unwrap();

        p.left = q.right.take();
        p.update();
        q.right = Some(p);
        q.update();

        q
    }

    fn update(&mut self) {
        self.high_max = self.high;

        if let Some(ref c) = self.left {
            self.high_max = self.high_max.max(c.high_max);
        }

        if let Some(ref c) = self.right {
            self.high_max = self.high_max.max(c.high_max);
        }
    }

    fn insert(tree: Tree<K, V>, low: K, high: K, val: V, seed: &mut u32) -> Tree<K, V> {
        if let Some(mut node) = tree {
            if node.low > low {
                node.left = TreeNode::insert(node.left, low, high, val, seed);

                if node.priority < node.left.as_ref().unwrap().priority {
                    return Some(TreeNode::rotate_right(node));
                }
            } else {
                node.right = TreeNode::insert(node.right, low, high, val, seed);

                if node.priority < node.right.as_ref().unwrap().priority {
                    return Some(TreeNode::rotate_left(node));
                }
            }

            node.update();
            Some(node)
        } else {
            Some(Box::new(TreeNode::new(
                None,
                None,
                low,
                high,
                high,
                val,
                xorshift32(seed),
            )))
        }
    }

    fn remove(tree: Tree<K, V>, low: K, high: K) -> Tree<K, V> {
        let mut node = match tree {
            None => return None,
            Some(node) => node,
        };

        if low == node.low && high == node.high {
            return TreeNode::merge(node.left, node.right);
        }

        if node.low > low {
            node.left = TreeNode::remove(node.left, low, high);
        } else {
            node.right = TreeNode::remove(node.right, low, high);
        }

        node.update();
        Some(node)
    }

    fn merge(a: Tree<K, V>, b: Tree<K, V>) -> Tree<K, V> {
        match (a, b) {
            (None, x) | (x, None) => x,
            (Some(mut a), Some(mut b)) => {
                if a.priority > b.priority {
                    a.right = TreeNode::merge(a.right, Some(b));
                    a.update();
                    Some(a)
                } else {
                    b.left = TreeNode::merge(Some(a), b.left);
                    b.update();
                    Some(b)
                }
            }
        }
    }

    fn query(tree: &Tree<K, V>, low: K, high: K, ret: &mut Vec<V>) {
        if let Some(node) = tree {
            if node.low <= high && node.high >= low {
                ret.push(node.val);
            }

            if let Some(ref left) = node.left {
                if left.high_max >= low {
                    TreeNode::query(&node.left, low, high, ret);
                }
            }

            if node.low <= high {
                TreeNode::query(&node.right, low, high, ret);
            }
        }
    }
}

const EPS: f64 = 1e-9;

#[inline(always)]
fn sign(x: i64) -> i64 {
    if x < 0 {
        -1
    } else if x > 0 {
        1
    } else {
        0
    }
}

#[derive(Debug, Default, Copy, Clone)]
pub struct Point {
    pub x: i64,
    pub y: i64,
}

impl Point {
    #[inline(always)]
    pub fn new(x: i64, y: i64) -> Self {
        Self { x, y }
    }

    #[inline(always)]
    pub fn dot(&self, other: &Point) -> i64 {
        self.x * other.x + self.y * other.y
    }

    #[inline(always)]
    pub fn cross(&self, other: &Point) -> i64 {
        self.x * other.y - self.y * other.x
    }

    #[inline(always)]
    pub fn cross2(&self, p1: &Point, p2: &Point) -> i64 {
        (*p1 - *self).cross(&(*p2 - *self))
    }
}

impl Point {
    #[inline(always)]
    pub fn orient(a: &Point, b: &Point, c: &Point) -> i64 {
        (b.x - a.x) * (c.y - a.y) - (b.y - a.y) * (c.x - a.x)
    }

    #[inline(always)]
    pub fn is_on_segment(a: &Point, b: &Point, p: &Point) -> bool {
        Self::orient(a, b, p) == 0
            && p.x >= a.x.min(b.x)
            && p.x <= a.x.max(b.x)
            && p.y >= a.y.min(b.y)
            && p.y <= a.y.max(b.y)
    }

    #[inline(always)]
    pub fn is_on_segment_f64(a: &Point, b: &Point, x: f64, y: f64) -> bool {
        let cross = (b.x - a.x) as f64 * (y - a.y as f64) - (b.y - a.y) as f64 * (x - a.x as f64);

        if cross.abs() > EPS {
            return false;
        }

        let (x_min, x_max) = (a.x.min(b.x) as f64 - EPS, a.x.max(b.x) as f64 + EPS);
        let (y_min, y_max) = (a.y.min(b.y) as f64 - EPS, a.y.max(b.y) as f64 + EPS);

        x >= x_min && x <= x_max && y >= y_min && y <= y_max
    }
}

impl Add for Point {
    type Output = Point;
    fn add(self, other: Point) -> Point {
        Point::new(self.x + other.x, self.y + other.y)
    }
}

impl Sub for Point {
    type Output = Point;
    fn sub(self, other: Point) -> Point {
        Point::new(self.x - other.x, self.y - other.y)
    }
}

impl Mul<i64> for Point {
    type Output = Point;

    fn mul(self, rhs: i64) -> Self::Output {
        Point::new(self.x * rhs, self.y * rhs)
    }
}

impl Div<i64> for Point {
    type Output = Point;

    fn div(self, rhs: i64) -> Self::Output {
        Point::new(self.x / rhs, self.y / rhs)
    }
}

impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        sign(self.x - other.x) == 0 && sign(self.y - other.y) == 0
    }
}

#[derive(Debug, Clone)]
struct Edge {
    p: Point,
    q: Point,
}

impl Edge {
    fn new(p: Point, q: Point) -> Self {
        Self { p, q }
    }
}

#[derive(Debug, Clone, Copy)]
struct BBox {
    x_min: f64,
    x_max: f64,
    y_min: f64,
    y_max: f64,
}

impl BBox {
    fn new(x_min: f64, x_max: f64, y_min: f64, y_max: f64) -> Self {
        BBox {
            x_min,
            x_max,
            y_min,
            y_max,
        }
    }

    fn from_edge(edge: &Edge) -> Self {
        BBox {
            x_min: edge.p.x.min(edge.q.x) as f64,
            x_max: edge.p.x.max(edge.q.x) as f64,
            y_min: edge.p.y.min(edge.q.y) as f64,
            y_max: edge.p.y.max(edge.q.y) as f64,
        }
    }

    fn union(a: BBox, b: BBox) -> BBox {
        BBox {
            x_min: a.x_min.min(b.x_min),
            x_max: a.x_max.max(b.x_max),
            y_min: a.y_min.min(b.y_min),
            y_max: a.y_max.max(b.y_max),
        }
    }

    fn longest_axis(&self) -> bool {
        (self.x_max - self.x_min) >= (self.y_max - self.y_min)
    }
}

#[derive(Debug, Clone)]
struct BVHNode {
    bbox: BBox,
    left: Option<Box<BVHNode>>,
    right: Option<Box<BVHNode>>,
    edges: Vec<Edge>,
}

impl BVHNode {
    fn new(
        bbox: BBox,
        left: Option<Box<BVHNode>>,
        right: Option<Box<BVHNode>>,
        edges: Vec<Edge>,
    ) -> Self {
        BVHNode {
            bbox,
            left,
            right,
            edges,
        }
    }

    fn build(edges: &mut [Edge]) -> Self {
        let mut bbox = BBox::new(f64::MAX, f64::MIN, f64::MAX, f64::MIN);

        for edge in edges.iter() {
            bbox = BBox::union(bbox, BBox::from_edge(edge));
        }

        if edges.len() <= 8 {
            return BVHNode::new(bbox, None, None, edges.to_vec());
        }

        let axis_x = bbox.longest_axis();
        let mid = edges.len() / 2;

        edges.sort_unstable_by(|a, b| {
            let centroid_a = if axis_x { a.p.x + a.q.x } else { a.p.y + a.q.y };
            let centroid_b = if axis_x { b.p.x + b.q.x } else { b.p.y + b.q.y };
            centroid_a.cmp(&centroid_b)
        });

        let (left, right) = edges.split_at_mut(mid);

        BVHNode::new(
            bbox,
            Some(Box::new(BVHNode::build(left))),
            Some(Box::new(BVHNode::build(right))),
            Vec::new(),
        )
    }

    fn from_edges(mut edges: Vec<Edge>) -> Self {
        BVHNode::build(&mut edges)
    }

    #[inline(always)]
    fn is_point_on_edge(&self, x: f64, y: f64) -> bool {
        let mut stack = vec![self];

        while let Some(node) = stack.pop() {
            if x < node.bbox.x_min - EPS
                || x > node.bbox.x_max + EPS
                || y < node.bbox.y_min - EPS
                || y > node.bbox.y_max + EPS
            {
                continue;
            }

            for edge in node.edges.iter() {
                if Point::is_on_segment_f64(&edge.p, &edge.q, x, y) {
                    return true;
                }
            }

            if let Some(ref left) = node.left {
                stack.push(left);
            }

            if let Some(ref right) = node.right {
                stack.push(right);
            }
        }

        false
    }

    #[inline(always)]
    fn count_ray_crossings(&self, x: f64, y: f64) -> usize {
        let mut stack = vec![self];
        let mut cnt = 0;

        while let Some(node) = stack.pop() {
            if x > node.bbox.x_max || y < node.bbox.y_min || y > node.bbox.y_max {
                continue;
            }

            for edge in node.edges.iter() {
                let (ay, by) = (edge.p.y as f64, edge.q.y as f64);

                if (ay <= y && by > y) || (by <= y && ay > y) {
                    let x_int =
                        ((edge.q.x - edge.p.x) as f64 * (y - ay) / (by - ay)) + edge.p.x as f64;

                    if x_int > x {
                        cnt += 1;
                    }
                }
            }

            if let Some(ref left) = node.left {
                stack.push(left);
            }

            if let Some(ref right) = node.right {
                stack.push(right);
            }
        }

        cnt
    }
}

#[derive(Debug)]
struct TrapMap {
    bvh: BVHNode,
}

impl TrapMap {
    fn new(polygon: &Polygon) -> Self {
        let edges = (0..polygon.points.len())
            .map(|i| {
                Edge::new(
                    polygon.points[i],
                    polygon.points[(i + 1) % polygon.points.len()],
                )
            })
            .collect::<Vec<_>>();

        Self {
            bvh: BVHNode::from_edges(edges),
        }
    }

    #[inline(always)]
    fn locate(&self, point: Point) -> i64 {
        if self.bvh.is_point_on_edge(point.x as f64, point.y as f64) {
            return 2;
        }

        let cnt = self.bvh.count_ray_crossings(point.x as f64, point.y as f64);

        if cnt & 1 == 1 {
            1
        } else {
            0
        }
    }

    #[inline(always)]
    fn locate_f64(&self, x: f64, y: f64) -> i64 {
        if self.bvh.is_point_on_edge(x, y) {
            return 2;
        }

        let cnt = self.bvh.count_ray_crossings(x, y);

        if cnt & 1 == 1 {
            1
        } else {
            0
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum PolygonType {
    A,
    B,
    AB,
}

#[derive(Debug, Clone)]
struct Segment {
    p: Point,
    q: Point,
    dx: i64,
    dy: i64,
    idx: usize,
    n: usize,
    polygon_type: PolygonType,
}

impl Segment {
    fn new(
        p: Point,
        q: Point,
        dx: i64,
        dy: i64,
        idx: usize,
        n: usize,
        polygon_type: PolygonType,
    ) -> Self {
        Self {
            p,
            q,
            dx,
            dy,
            idx,
            n,
            polygon_type,
        }
    }

    fn y_at(&self, x: f64) -> f64 {
        if self.is_vertical() {
            self.p.y as f64
        } else {
            let t = (x - self.p.x as f64) / (self.q.x - self.p.x) as f64;
            self.p.y as f64 + t * (self.q.y - self.p.y) as f64
        }
    }

    fn y_range(&self) -> (i64, i64) {
        (self.p.y.min(self.q.y), self.p.y.max(self.q.y))
    }

    fn is_vertical(&self) -> bool {
        self.p.x == self.q.x
    }

    fn is_intersect(&self, other: &Segment) -> bool {
        let o1 = Point::orient(&self.p, &self.q, &other.p).signum();
        let o2 = Point::orient(&self.p, &self.q, &other.q).signum();
        let o3 = Point::orient(&other.p, &other.q, &self.p).signum();
        let o4 = Point::orient(&other.p, &other.q, &self.q).signum();

        // Collinear case
        if o1 == 0 && o2 == 0 && o3 == 0 && o4 == 0 {
            let cond1 = self.p.x.max(self.q.x) >= other.p.x.min(other.q.x);
            let cond2 = self.p.y.max(self.q.y) >= other.p.y.min(other.q.y);
            let cond3 = self.p.x.min(self.q.x) <= other.p.x.max(other.q.x);
            let cond4 = self.p.y.min(self.q.y) <= other.p.y.max(other.q.y);

            return cond1 && cond2 && cond3 && cond4;
        }

        o1 * o2 <= 0 && o3 * o4 <= 0
    }

    fn is_intersect_proper(&self, other: &Segment) -> bool {
        self.is_intersect(other)
            && !(Point::is_on_segment(&self.p, &self.q, &other.p)
                || Point::is_on_segment(&self.p, &self.q, &other.q)
                || Point::is_on_segment(&other.p, &other.q, &self.p)
                || Point::is_on_segment(&other.p, &other.q, &self.q))
    }

    fn is_adjacent(&self, other: &Segment) -> bool {
        (self.idx + 1) % self.n == other.idx || (other.idx + 1) % other.n == self.idx
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum EventKind {
    Left,
    Right,
}

#[derive(Clone, Copy)]
struct Event {
    point: Point,
    kind: EventKind,
    idx: usize,
}

impl Event {
    fn new(point: Point, kind: EventKind, idx: usize) -> Self {
        Self { point, kind, idx }
    }
}

#[derive(Clone)]
pub struct Polygon {
    points: Vec<Point>,
}

impl Polygon {
    fn new(points: Vec<Point>) -> Self {
        Self { points }
    }

    fn orient(&self) -> i64 {
        if self.area() > 0.0 {
            1
        } else {
            -1
        }
    }

    fn area(&self) -> f64 {
        let mut ret = 0;

        for i in 0..self.points.len() {
            let j = (i + 1) % self.points.len();

            ret += self.points[i].x * self.points[j].y;
            ret -= self.points[j].x * self.points[i].y;
        }

        ret as f64 * 0.5
    }

    fn is_simple(&self) -> bool {
        let n = self.points.len();

        if n < 3 || self.area() == 0.0 {
            return false;
        }

        for i in 0..n {
            let prev = self.points[(i + n - 1) % n];
            let curr = self.points[i];
            let next = self.points[(i + 1) % n];

            if Point::orient(&prev, &curr, &next) == 0 {
                if Point::is_on_segment(&prev, &curr, &next)
                    || Point::is_on_segment(&curr, &next, &prev)
                {
                    return false;
                }
            }
        }

        let mut segments = Vec::with_capacity(n);

        for i in 0..n {
            let p0 = self.points[i];
            let p1 = self.points[(i + 1) % n];
            let (left, right) = if (p1.x, p1.y) < (p0.x, p0.y) {
                (p1, p0)
            } else {
                (p0, p1)
            };

            segments.push(Segment::new(
                left,
                right,
                p1.x - p0.x,
                p1.y - p0.y,
                i,
                n,
                PolygonType::AB,
            ));
        }

        let mut events = Vec::with_capacity(segments.len() * 2);

        for (idx, segment) in segments.iter().enumerate() {
            events.push(Event::new(segment.p, EventKind::Left, idx));
            events.push(Event::new(segment.q, EventKind::Right, idx));
        }

        events.sort_unstable_by(|a, b| {
            a.point
                .x
                .cmp(&b.point.x)
                .then(a.kind.cmp(&b.kind))
                .then(a.point.y.cmp(&b.point.y))
        });

        let mut status = BTreeSet::new();
        let mut key_of = vec![(Float64(0.0), 0); segments.len()];
        let mut tree: Tree<i64, usize> = None;
        let mut seed = 0x1234_5678;

        for event in events {
            let segment = &segments[event.idx];
            let x = event.point.x as f64 + EPS;

            match event.kind {
                EventKind::Left => {
                    if segment.is_vertical() {
                        for &(_, idx) in status.iter() {
                            if segment.is_intersect(&segments[idx])
                                && !segment.is_adjacent(&segments[idx])
                            {
                                return false;
                            }
                        }

                        let mut candidate = Vec::new();
                        let (low, high) = segment.y_range();

                        TreeNode::query(&tree, low, high, &mut candidate);

                        for idx in candidate {
                            if segment.is_intersect(&segments[idx])
                                && !segment.is_adjacent(&segments[idx])
                            {
                                return false;
                            }
                        }

                        tree = TreeNode::insert(tree, low, high, event.idx, &mut seed);
                    } else {
                        let mut candidate = Vec::new();
                        let (low, high) = segment.y_range();

                        TreeNode::query(&tree, low, high, &mut candidate);

                        for idx in candidate {
                            if segment.is_intersect(&segments[idx])
                                && !segment.is_adjacent(&segments[idx])
                            {
                                return false;
                            }
                        }
                    }

                    let key = (
                        if segment.is_vertical() {
                            Float64(segment.p.y.min(segment.q.y) as f64)
                        } else {
                            Float64(segment.y_at(x))
                        },
                        event.idx,
                    );
                    let pred = status.range(..key).next_back().cloned();
                    let succ = status.range((Excluded(key), Unbounded)).next().cloned();

                    if let Some((_, idx)) = pred {
                        if segment.is_intersect(&segments[idx])
                            && !segment.is_adjacent(&segments[idx])
                        {
                            return false;
                        }
                    }

                    if let Some((_, idx)) = succ {
                        if segment.is_intersect(&segments[idx])
                            && !segment.is_adjacent(&segments[idx])
                        {
                            return false;
                        }
                    }

                    status.insert(key);
                    key_of[event.idx] = key;
                }
                EventKind::Right => {
                    if segment.is_vertical() {
                        let (low, high) = segment.y_range();
                        tree = TreeNode::remove(tree, low, high);
                    }

                    let key = key_of[event.idx];
                    let pred = status.range(..key).next_back().cloned();
                    let succ = status.range((Excluded(key), Unbounded)).next().cloned();

                    status.remove(&key);

                    if let (Some((_, idx1)), Some((_, idx2))) = (pred, succ) {
                        if segments[idx1].is_intersect(&segments[idx2])
                            && !segments[idx1].is_adjacent(&segments[idx2])
                        {
                            return false;
                        }
                    }
                }
            }
        }

        true
    }

    fn intersect(&self, other: &Polygon) -> bool {
        let sign = [self.orient(), other.orient()];
        let mut segments = Vec::with_capacity(self.points.len() + other.points.len());

        let mut push_edges = |points: &[Point], polygon_type: PolygonType| {
            let n = points.len();

            for i in 0..n {
                let p0 = points[i];
                let p1 = points[(i + 1) % n];
                let (left, right) = if (p1.x, p1.y) < (p0.x, p0.y) {
                    (p1, p0)
                } else {
                    (p0, p1)
                };

                segments.push(Segment::new(
                    left,
                    right,
                    p1.x - p0.x,
                    p1.y - p0.y,
                    i,
                    n,
                    polygon_type,
                ));
            }
        };

        push_edges(&self.points, PolygonType::A);
        push_edges(&other.points, PolygonType::B);

        let interior_normal = |segment: &Segment| -> (i64, i64) {
            if sign[segment.polygon_type as usize] > 0 {
                (-segment.dy, segment.dx)
            } else {
                (segment.dy, -segment.dx)
            }
        };

        let overlap_same_side = |a: &Segment, b: &Segment| -> bool {
            if Point::orient(&a.p, &a.q, &b.p) != 0 || Point::orient(&a.p, &a.q, &b.q) != 0 {
                return false;
            }

            let (ax1, ax2) = if a.p.x != a.q.x {
                (a.p.x.min(a.q.x), a.p.x.max(a.q.x))
            } else {
                (a.p.y.min(a.q.y), a.p.y.max(a.q.y))
            };
            let (bx1, bx2) = if b.p.x != b.q.x {
                (b.p.x.min(b.q.x), b.p.x.max(b.q.x))
            } else {
                (b.p.y.min(b.q.y), b.p.y.max(b.q.y))
            };

            if ax1.max(bx1) >= ax2.min(bx2) {
                return false;
            }

            let n1 = interior_normal(a);
            let n2 = interior_normal(b);

            n1.0 * n2.0 + n1.1 * n2.1 > 0
        };

        let proper_hit = |a: &Segment, b: &Segment| -> bool {
            a.polygon_type != b.polygon_type && a.is_intersect_proper(&b)
        };
        let general_hit = |a: &Segment, b: &Segment| -> bool {
            a.polygon_type != b.polygon_type
                && (a.is_intersect_proper(&b) || overlap_same_side(a, b))
        };

        let mut events = Vec::with_capacity(segments.len() * 2);

        for (idx, segment) in segments.iter().enumerate() {
            events.push(Event::new(segment.p, EventKind::Left, idx));
            events.push(Event::new(segment.q, EventKind::Right, idx));
        }

        events.sort_unstable_by(|a, b| {
            a.point
                .x
                .cmp(&b.point.x)
                .then(a.kind.cmp(&b.kind))
                .then(a.point.y.cmp(&b.point.y))
        });

        let mut status = BTreeSet::new();
        let mut key_of = vec![(Float64(0.0), 0); segments.len()];
        let mut tree: Tree<i64, usize> = None;
        let mut seed = 0x8765_4321;

        for event in events {
            let segment = &segments[event.idx];
            let x = event.point.x as f64 + EPS;

            match event.kind {
                EventKind::Left => {
                    if segment.is_vertical() {
                        for &(_, idx) in status.iter() {
                            if general_hit(&segment, &segments[idx]) {
                                return true;
                            }
                        }

                        let mut candidate = Vec::new();
                        let (low, high) = segment.y_range();

                        TreeNode::query(&tree, low, high, &mut candidate);

                        for idx in candidate {
                            if general_hit(&segment, &segments[idx]) {
                                return true;
                            }
                        }

                        tree = TreeNode::insert(tree, low, high, event.idx, &mut seed);
                    } else {
                        let mut candidate = Vec::new();
                        let (low, high) = segment.y_range();

                        TreeNode::query(&tree, low, high, &mut candidate);

                        for idx in candidate {
                            if general_hit(&segment, &segments[idx]) {
                                return true;
                            }
                        }
                    }

                    let key = (
                        if segment.is_vertical() {
                            Float64(segment.p.y.min(segment.q.y) as f64)
                        } else {
                            Float64(segment.y_at(x))
                        },
                        event.idx,
                    );
                    let pred = status.range(..key).next_back().cloned();
                    let succ = status.range((Excluded(key), Unbounded)).next().cloned();

                    if let Some((_, idx)) = pred {
                        if general_hit(&segment, &segments[idx]) {
                            return true;
                        }
                    }

                    if let Some((_, idx)) = succ {
                        if general_hit(&segment, &segments[idx]) {
                            return true;
                        }
                    }

                    status.insert(key);
                    key_of[event.idx] = key;
                }
                EventKind::Right => {
                    if segment.is_vertical() {
                        let (low, high) = segment.y_range();
                        tree = TreeNode::remove(tree, low, high);
                    }

                    let key = key_of[event.idx];
                    let pred = status.range(..key).next_back().cloned();
                    let succ = status.range((Excluded(key), Unbounded)).next().cloned();

                    status.remove(&key);

                    if let (Some((_, idx1)), Some((_, idx2))) = (pred, succ) {
                        if proper_hit(&segments[idx1], &segments[idx2]) {
                            return true;
                        }
                    }
                }
            }
        }

        let map_a = TrapMap::new(self);
        let map_b = TrapMap::new(other);

        let class_a = self
            .points
            .iter()
            .map(|&p| map_b.locate(p))
            .collect::<Vec<_>>();
        let class_b = other
            .points
            .iter()
            .map(|&p| map_a.locate(p))
            .collect::<Vec<_>>();

        if class_a.iter().any(|&c| c == 1) || class_b.iter().any(|&c| c == 1) {
            return true;
        }

        let chord_inside_fast = |polygon: &Polygon, class: &[i64], map: &TrapMap| -> bool {
            let n = polygon.points.len();

            for i in 0..n {
                if class[i] != 2 {
                    continue;
                }

                let j = (i + 1) % n;

                if class[j] != 2 {
                    continue;
                }

                let mid_x = (polygon.points[i].x + polygon.points[j].x) as f64 * 0.5;
                let mid_y = (polygon.points[i].y + polygon.points[j].y) as f64 * 0.5;

                if map.locate_f64(mid_x, mid_y) == 1 {
                    return true;
                }
            }

            false
        };

        if chord_inside_fast(&self, &class_a, &map_b) || chord_inside_fast(&other, &class_b, &map_a)
        {
            return true;
        }

        false
    }

    fn equal_union(&self, a: &Polygon, b: &Polygon) -> bool {
        if self.area().abs() != a.area().abs() + b.area().abs() {
            return false;
        }

        let map_a = TrapMap::new(a);
        let map_b = TrapMap::new(b);
        let map_ab = TrapMap::new(self);

        if a.points.iter().any(|&p| map_ab.locate(p) == 0) {
            return false;
        }

        if b.points.iter().any(|&p| map_ab.locate(p) == 0) {
            return false;
        }

        for &point in self.points.iter() {
            if map_a.locate(point) == 0 && map_b.locate(point) == 0 {
                return false;
            }
        }

        true
    }
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let n_a = scan.token::<usize>();
    let mut points_a = Vec::with_capacity(n_a);

    for _ in 0..n_a {
        points_a.push(Point::new(scan.token::<i64>(), scan.token::<i64>()));
    }

    let n_b = scan.token::<usize>();
    let mut points_b = Vec::with_capacity(n_b);

    for _ in 0..n_b {
        points_b.push(Point::new(scan.token::<i64>(), scan.token::<i64>()));
    }

    let n_ab = scan.token::<usize>();
    let mut points_ab = Vec::with_capacity(n_ab);

    for _ in 0..n_ab {
        points_ab.push(Point::new(scan.token::<i64>(), scan.token::<i64>()));
    }

    let polygon_a = Polygon::new(points_a);
    let polygon_b = Polygon::new(points_b);
    let polygon_ab = Polygon::new(points_ab);

    if !polygon_a.is_simple() {
        writeln!(out, "Aastria is not a polygon").unwrap();
        return;
    }

    if !polygon_b.is_simple() {
        writeln!(out, "Abstria is not a polygon").unwrap();
        return;
    }

    if !polygon_ab.is_simple() {
        writeln!(out, "Aabstria is not a polygon").unwrap();
        return;
    }

    if polygon_a.intersect(&polygon_b) {
        writeln!(out, "Aastria and Abstria intersect").unwrap();
        return;
    }

    if !polygon_ab.equal_union(&polygon_a, &polygon_b) {
        writeln!(
            out,
            "The union of Aastria and Abstria is not equal to Aabstria"
        )
        .unwrap();
        return;
    }

    writeln!(out, "OK").unwrap();
}
