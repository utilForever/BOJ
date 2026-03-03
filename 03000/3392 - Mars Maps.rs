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

#[derive(Debug, Default, Clone, Copy)]
struct Node {
    min_cover: i64,
    min_cover_len: i64,
    total_len: i64,
}

impl Node {
    fn new(len: i64) -> Self {
        Self {
            min_cover: 0,
            min_cover_len: len,
            total_len: len,
        }
    }

    fn merge(&self, other: &Self) -> Node {
        let min_cover = self.min_cover.min(other.min_cover);
        let mut min_cover_len = 0;

        if self.min_cover == min_cover {
            min_cover_len += self.min_cover_len;
        }

        if other.min_cover == min_cover {
            min_cover_len += other.min_cover_len;
        }

        Node {
            min_cover,
            min_cover_len,
            total_len: self.total_len + other.total_len,
        }
    }
}

#[derive(Debug)]
struct SegmentTree {
    size: usize,
    data: Vec<Node>,
    lazy: Vec<i64>,
}

impl SegmentTree {
    fn new(n: usize) -> Self {
        Self {
            size: n,
            data: vec![Node::default(); n * 4 + 1],
            lazy: vec![0; n * 4 + 1],
        }
    }

    fn init(&mut self, data: &Vec<i64>) {
        let n = data.len();
        self.init_internal(data, 1, 1, n);
    }

    fn init_internal(&mut self, data: &Vec<i64>, node: usize, start: usize, end: usize) {
        if start == end {
            self.data[node] = Node::new(data[start - 1]);
            return;
        }

        let mid = (start + end) / 2;

        self.init_internal(data, node * 2, start, mid);
        self.init_internal(data, node * 2 + 1, mid + 1, end);
        self.data[node] = self.data[node * 2].merge(&self.data[node * 2 + 1]);
    }

    fn propagate(&mut self, node: usize, start: usize, end: usize) {
        if start != end {
            self.lazy[node * 2] += self.lazy[node];
            self.lazy[node * 2 + 1] += self.lazy[node];
        }

        self.data[node].min_cover += self.lazy[node];
        self.lazy[node] = 0;
    }

    pub fn update(&mut self, start: usize, end: usize, val: i64) {
        self.update_internal(start, end, val, 1, 1, self.size);
    }

    fn update_internal(
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
            self.lazy[node] += val;
            self.propagate(node, node_start, node_end);
            return;
        }

        let mid = (node_start + node_end) / 2;
        self.update_internal(start, end, val, node * 2, node_start, mid);
        self.update_internal(start, end, val, node * 2 + 1, mid + 1, node_end);

        self.data[node] = self.data[node * 2].merge(&self.data[node * 2 + 1]);
    }

    pub fn query(&self) -> i64 {
        let root = self.data[1];
        let uncovered = if root.min_cover == 0 {
            root.min_cover_len
        } else {
            0
        };

        root.total_len - uncovered
    }
}

#[derive(Clone, Copy)]
struct Rect {
    x1: i64,
    x2: i64,
    y1: i64,
    y2: i64,
}

impl Rect {
    fn new(x1: i64, x2: i64, y1: i64, y2: i64) -> Self {
        Self { x1, x2, y1, y2 }
    }
}

#[derive(Clone, Copy)]
struct Event {
    x: i64,
    idx_y1: usize,
    idx_y2: usize,
    delta: i32,
}

impl Event {
    fn new(x: i64, idx_y1: usize, idx_y2: usize, delta: i32) -> Self {
        Self {
            x,
            idx_y1,
            idx_y2,
            delta,
        }
    }
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let n = scan.token::<usize>();
    let mut rectangles = Vec::with_capacity(n);
    let mut points_y = Vec::with_capacity(n * 2);

    for _ in 0..n {
        let (x1, y1, x2, y2) = (
            scan.token::<i64>(),
            scan.token::<i64>(),
            scan.token::<i64>(),
            scan.token::<i64>(),
        );

        rectangles.push(Rect::new(x1, x2, y1, y2));
        points_y.push(y1);
        points_y.push(y2);
    }

    points_y.sort_unstable();
    points_y.dedup();

    let mut segments = Vec::with_capacity(points_y.len() - 1);

    for i in 0..points_y.len() - 1 {
        segments.push(points_y[i + 1] - points_y[i]);
    }

    let mut segment_tree = SegmentTree::new(segments.len());
    segment_tree.init(&segments);

    let mut events = Vec::with_capacity(n * 2);

    for rect in rectangles {
        let idx_y1 = points_y.binary_search(&rect.y1).unwrap();
        let idx_y2 = points_y.binary_search(&rect.y2).unwrap();

        events.push(Event::new(rect.x1, idx_y1 + 1, idx_y2, 1));
        events.push(Event::new(rect.x2, idx_y1 + 1, idx_y2, -1));
    }

    events.sort_unstable_by(|a, b| a.x.cmp(&b.x));

    let mut x_prev = events[0].x;
    let mut idx = 0;
    let mut ret = 0;

    while idx < events.len() {
        let x = events[idx].x;

        ret += segment_tree.query() * (x - x_prev);

        while idx < events.len() && events[idx].x == x {
            segment_tree.update(
                events[idx].idx_y1,
                events[idx].idx_y2,
                events[idx].delta as i64,
            );
            idx += 1;
        }

        x_prev = x;
    }

    writeln!(out, "{ret}").unwrap();
}
