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
    a: i64,
    b: i64,
}

impl Line {
    fn new(a: i64, b: i64) -> Self {
        Self { a, b }
    }

    fn calculate(&self, x: i64) -> i64 {
        self.a * x + self.b
    }

    fn compare(&self, x: &Line, t: i64) -> i64 {
        if self.a == x.a {
            return INF;
        }

        let mut up = x.b - self.b;
        let mut down = self.a - x.a;

        if down < 0 {
            up *= -1;
            down *= -1;
        }

        let intercept = if up <= 0 {
            -(-up / down)
        } else {
            (up + down - 1) / down
        };

        if intercept <= t {
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
    time: i64,
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
            time: 0,
        }
    }

    pub fn construct(&mut self, lines: &[Line], time: i64, start: usize, end: usize) {
        self.time = time;

        self.construct_internal(lines, 1, start, end);
    }

    fn construct_internal(&mut self, lines: &[Line], node: usize, start: usize, end: usize) {
        if start == end {
            self.data[node] = lines[start].clone();
        } else {
            let mid = (start + end) / 2;

            self.construct_internal(lines, node * 2, start, mid);
            self.construct_internal(lines, node * 2 + 1, mid + 1, end);

            self.merge(node);
        }
    }

    fn merge(&mut self, node: usize) {
        let left = self.data[node * 2].calculate(self.time);
        let right = self.data[node * 2 + 1].calculate(self.time);

        self.data[node] = if left < right
            || (left == right && self.data[node * 2].a < self.data[node * 2 + 1].a)
        {
            self.data[node * 2].clone()
        } else {
            self.data[node * 2 + 1].clone()
        };
        self.melt[node] = self.melt[node * 2]
            .min(self.melt[node * 2 + 1])
            .min(self.data[node * 2].compare(&self.data[node * 2 + 1], self.time));
    }

    fn update(&mut self, pos: usize, val: Line, node: usize, start: usize, end: usize) {
        if pos < start || pos > end {
            return;
        }

        if start == end {
            self.data[node] = val;
            return;
        }

        let mid = (start + end) / 2;

        self.update(pos, val.clone(), node * 2, start, mid);
        self.update(pos, val, node * 2 + 1, mid + 1, end);

        self.merge(node);
    }

    fn query(
        &mut self,
        node: usize,
        start: usize,
        end: usize,
        node_start: usize,
        node_end: usize,
    ) -> i64 {
        if end < node_start || node_end < start {
            return INF;
        }

        if start <= node_start && node_end <= end {
            return self.data[node].calculate(self.time);
        }

        let mid = (node_start + node_end) / 2;
        let left = self.query(node * 2, start, end, node_start, mid);
        let right = self.query(node * 2 + 1, start, end, mid + 1, node_end);

        left.min(right)
    }

    fn heaten(&mut self, time: i64) {
        self.time = time;

        self.heaten_internal(1, 0, self.size);
    }

    fn heaten_internal(&mut self, node: usize, start: usize, end: usize) {
        if self.melt[node] > self.time {
            return;
        }

        let mid = (start + end) / 2;

        self.heaten_internal(node * 2, start, mid);
        self.heaten_internal(node * 2 + 1, mid + 1, end);

        self.merge(node);
    }
}

struct Query {
    start: usize,
    end: usize,
    time: i64,
    idx: usize,
}

// Reference: https://koosaga.com/307
fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let q = scan.token::<i64>();
    let mut stones = Vec::new();
    let mut queries = Vec::new();

    for _ in 0..q {
        let num = scan.token::<i64>();

        if num == 1 {
            let (a, b) = (scan.token::<i64>(), scan.token::<i64>());
            stones.push(Line::new(-a, -b));
        } else {
            let t = scan.token::<i64>();
            queries.push(Query {
                start: 0,
                end: stones.len(),
                time: t,
                idx: queries.len(),
            });
        }
    }

    let mut tree = KineticSegmentTree::new(stones.len());
    tree.construct(&stones, -INF, 0, stones.len() - 1);

    queries.sort_by(|a, b| a.time.cmp(&b.time));

    let mut ret = vec![0; queries.len()];

    for query in queries.iter() {
        tree.heaten(query.time);
        ret[query.idx] = -tree.query(1, query.start, query.end - 1, 0, stones.len() - 1);
    }

    for val in ret.iter() {
        writeln!(out, "{val}").unwrap();
    }
}
