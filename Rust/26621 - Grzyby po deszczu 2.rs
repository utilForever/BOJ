use io::Write;
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

const INF: i64 = 1_000_000_000_000_000_000;

#[derive(Clone, Default)]
struct Line {
    idx: usize,
    slope: i64,
    ret: i64,
}

impl Line {
    fn new(a: i64, b: i64) -> Self {
        Self {
            idx: 0,
            slope: a,
            ret: b,
        }
    }

    fn compare(&self, x: &Line) -> i64 {
        if self.slope == x.slope {
            return INF;
        }

        let mut up = x.ret - self.ret;
        let mut down = self.slope - x.slope;

        if down < 0 {
            up *= -1;
            down *= -1;
        }

        let intercept = if up <= 0 {
            -(-up / down)
        } else {
            (up + down - 1) / down
        };

        if intercept <= 0 {
            INF
        } else {
            intercept
        }
    }
}

struct KineticSegmentTree {
    size: usize,
    data: Vec<Line>,
    melt: Vec<i64>,
    tag: Vec<bool>,
    time: Vec<i64>,
    intercept: Vec<i64>,
}

impl KineticSegmentTree {
    pub fn new(n: usize) -> Self {
        let mut real_n = 1;
        while real_n < n {
            real_n *= 2;
        }

        Self {
            size: n,
            data: vec![Line::default(); real_n * 4],
            melt: vec![INF; real_n * 4],
            tag: vec![false; real_n * 4],
            time: vec![0; real_n * 4],
            intercept: vec![0; real_n * 4],
        }
    }

    pub fn construct(&mut self, lines: &[Line], start: usize, end: usize) {
        self.construct_internal(lines, 1, start, end);
    }

    fn construct_internal(&mut self, lines: &[Line], node: usize, start: usize, end: usize) {
        if start == end {
            self.data[node] = lines[start].clone();
            return;
        }

        let mid = (start + end) / 2;

        self.construct_internal(lines, node * 2, start, mid);
        self.construct_internal(lines, node * 2 + 1, mid + 1, end);

        self.merge(node);
    }

    fn merge(&mut self, node: usize) {
        let left = self.data[node * 2].clone();
        let right = self.data[node * 2 + 1].clone();

        self.data[node] =
            if left.ret > right.ret || (left.ret == right.ret && left.slope > right.slope) {
                left.clone()
            } else {
                right.clone()
            };
        self.melt[node] = self.melt[node * 2]
            .min(self.melt[node * 2 + 1])
            .min(left.compare(&right));
    }

    fn propagate(&mut self, node: usize, start: usize, end: usize) {
        if !self.tag[node] {
            return;
        }

        self.data[node].ret += self.data[node].slope * self.time[node] + self.intercept[node];
        self.melt[node] -= self.time[node];

        if start == end {
            self.tag[node] = false;
            self.time[node] = 0;
            self.intercept[node] = 0;
            return;
        }

        self.tag[node * 2] = true;
        self.time[node * 2] += self.time[node];
        self.intercept[node * 2] += self.intercept[node];

        self.tag[node * 2 + 1] = true;
        self.time[node * 2 + 1] += self.time[node];
        self.intercept[node * 2 + 1] += self.intercept[node];

        self.tag[node] = false;
        self.time[node] = 0;
        self.intercept[node] = 0;
    }

    fn add_intercept(&mut self, start: usize, end: usize, val: i64) {
        self.add_intercept_internal(start, end, val, 1, 1, self.size);
    }

    fn add_intercept_internal(
        &mut self,
        start: usize,
        end: usize,
        val: i64,
        node: usize,
        node_start: usize,
        node_end: usize,
    ) {
        self.propagate(node, node_start, node_end);

        if end < node_start || node_end < start {
            return;
        }

        if start <= node_start && node_end <= end {
            self.tag[node] = true;
            self.intercept[node] += val;
            self.propagate(node, node_start, node_end);
            return;
        }

        let mid = (node_start + node_end) / 2;

        self.add_intercept_internal(start, end, val.clone(), node * 2, node_start, mid);
        self.add_intercept_internal(start, end, val, node * 2 + 1, mid + 1, node_end);

        self.merge(node);
    }

    fn update(&mut self, start: usize, end: usize, val: Line) {
        self.update_internal(start, end, val, 1, 1, self.size);
    }

    fn update_internal(
        &mut self,
        start: usize,
        end: usize,
        val: Line,
        node: usize,
        node_start: usize,
        node_end: usize,
    ) {
        self.propagate(node, node_start, node_end);

        if end < node_start || node_end < start {
            return;
        }

        if start <= node_start && node_end <= end {
            self.data[node] = val;
            return;
        }

        let mid = (node_start + node_end) / 2;

        self.update_internal(start, end, val.clone(), node * 2, node_start, mid);
        self.update_internal(start, end, val, node * 2 + 1, mid + 1, node_end);

        self.merge(node);
    }

    fn heaten(&mut self, start: usize, end: usize, time: i64) {
        self.heaten_internal(1, start, end, 1, self.size, time);
    }

    fn heaten_internal(
        &mut self,
        node: usize,
        start: usize,
        end: usize,
        node_start: usize,
        node_end: usize,
        time: i64,
    ) {
        self.propagate(node, node_start, node_end);

        if end < node_start || node_end < start {
            return;
        }

        if start <= node_start && node_end <= end {
            self.melt[node] -= 1;

            if self.melt[node] > 0 {
                self.tag[node] = true;
                self.time[node] += 1;
                self.propagate(node, node_start, node_end);
                return;
            }
        }

        let mid = (node_start + node_end) / 2;

        self.heaten_internal(node * 2, start, end, node_start, mid, time);
        self.heaten_internal(node * 2 + 1, start, end, mid + 1, node_end, time);

        self.merge(node);
    }

    pub fn query(&mut self, start: usize, end: usize) -> Line {
        self.query_internal(start, end, 1, 1, self.size)
    }

    fn query_internal(
        &mut self,
        start: usize,
        end: usize,
        node: usize,
        node_start: usize,
        node_end: usize,
    ) -> Line {
        if end < node_start || node_end < start {
            return Line::default();
        }

        if start <= node_start && node_end <= end {
            return self.data[node].clone();
        }

        let mid = (node_start + node_end) / 2;
        let left = self.query_internal(node * 2, start, end, node_start, mid);
        let right = self.query_internal(node * 2 + 1, start, end, mid + 1, node_end);

        if left.ret > right.ret {
            left
        } else {
            right
        }
    }
}

// Reference: https://koosaga.com/307
fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let n = scan.token::<usize>();
    let mut dragons = vec![Line::default(); n + 1];

    for i in 1..=n {
        let (d, h) = (scan.token::<i64>(), scan.token::<i64>());
        dragons[i] = Line::new(d, h);
    }

    // Sort by increasing height for greedy approach
    dragons.sort_by(|a, b| a.slope.cmp(&b.slope));

    for (i, dragon) in dragons.iter_mut().enumerate() {
        dragon.idx = i;
    }

    let mut tree = KineticSegmentTree::new(n);
    tree.construct(&dragons, 1, n);

    let mut ret = 0;

    for i in 1..=n {
        let val = tree.query(1, n);
        ret += val.ret;

        writeln!(out, "{ret}").unwrap();

        tree.add_intercept(1, val.idx, val.slope);
        tree.heaten(val.idx + 1, n, i as i64);
        tree.update(val.idx, val.idx, Line::new(0, -INF));
    }
}
