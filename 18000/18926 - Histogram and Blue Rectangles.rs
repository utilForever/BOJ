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

fn div(a: i64, b: i64) -> i64 {
    let mut ret = a / b;

    if a ^ b < 0 && a % b != 0 {
        ret -= 1;
    }

    ret
}

#[derive(Clone)]
struct Line {
    a: i64,
    b: i64,
}

impl Line {
    fn get(&self, x: i64) -> i64 {
        self.a * x + self.b
    }

    fn cross(&self, other: &Line) -> i64 {
        div(other.b - self.b, self.a - other.a)
    }
}

#[derive(Clone)]
struct LineContainer {
    lines: Vec<Line>,
}

impl LineContainer {
    fn new() -> Self {
        Self { lines: Vec::new() }
    }

    fn push(&mut self, l: Line) {
        if !self.lines.is_empty() && self.lines[self.lines.len() - 1].a == l.a {
            if self.lines[self.lines.len() - 1].b > l.b {
                return;
            }

            self.lines.pop();
        }

        while self.lines.len() >= 2
            && self.lines[self.lines.len() - 2].cross(&self.lines[self.lines.len() - 1])
                >= self.lines[self.lines.len() - 1].cross(&l)
        {
            self.lines.pop();
        }

        self.lines.push(l);
    }

    fn get(&self, x: i64) -> i64 {
        if self.lines.is_empty() {
            return i64::MIN;
        }

        let mut low = 0;
        let mut high = self.lines.len() - 1;

        while low < high {
            let mid = (low + high) / 2;

            if self.lines[mid].get(x) < self.lines[mid + 1].get(x) {
                low = mid + 1;
            } else {
                high = mid;
            }
        }

        self.lines[low].get(x)
    }
}

struct SegmentTreeCHT {
    data: Vec<LineContainer>,
}

impl SegmentTreeCHT {
    fn new(n: usize) -> Self {
        let mut real_n = 1;
        while real_n < n {
            real_n *= 2;
        }

        Self {
            data: vec![LineContainer::new(); real_n * 4],
        }
    }

    fn update(
        &mut self,
        node: usize,
        start: usize,
        end: usize,
        left: usize,
        right: usize,
        a: i64,
        b: i64,
    ) {
        if left > end || right < start {
            return;
        }

        if left <= start && end <= right {
            self.data[node].push(Line { a, b });
            return;
        }

        let mid = (start + end) / 2;

        self.update(node * 2, start, mid, left, right, a, b);
        self.update(node * 2 + 1, mid + 1, end, left, right, a, b);
    }

    fn query(&mut self, node: usize, start: usize, end: usize, target: usize, x: i64) -> i64 {
        if start == end {
            return self.data[node].get(x);
        }

        let mid = (start + end) / 2;

        if target <= mid {
            self.data[node]
                .get(x)
                .max(self.query(node * 2, start, mid, target, x))
        } else {
            self.data[node]
                .get(x)
                .max(self.query(node * 2 + 1, mid + 1, end, target, x))
        }
    }
}

struct SegmentTree {
    data: Vec<i64>,
}

impl SegmentTree {
    fn new(n: usize) -> Self {
        let mut real_n = 1;
        while real_n < n {
            real_n *= 2;
        }

        Self {
            data: vec![i64::MIN; real_n * 4],
        }
    }

    fn update(&mut self, node: usize, start: usize, end: usize, idx: usize, val: i64) {
        if idx < start || idx > end {
            return;
        }

        self.data[node] = self.data[node].max(val);

        if start == end {
            return;
        }

        let mid = (start + end) / 2;

        self.update(node * 2, start, mid, idx, val);
        self.update(node * 2 + 1, mid + 1, end, idx, val);
    }

    fn query(&mut self, node: usize, start: usize, end: usize, left: usize, right: usize) -> i64 {
        if left > end || right < start {
            return i64::MIN;
        }

        if left <= start && end <= right {
            return self.data[node];
        }

        let mid = (start + end) / 2;

        self.query(node * 2, start, mid, left, right)
            .max(self.query(node * 2 + 1, mid + 1, end, left, right))
    }
}

#[derive(Default, Clone)]
struct Query {
    left: usize,
    right: usize,
    idx: usize,
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let n = scan.token::<usize>();
    let mut heights = vec![0; n + 1];

    for i in 1..=n {
        heights[i] = scan.token::<i64>();
    }

    let mut queries = vec![Query::default(); n];
    let mut ret = vec![0; n];

    for i in 0..n {
        queries[i] = Query {
            left: 1,
            right: i + 1,
            idx: i,
        };
    }

    let mut tree = SegmentTree::new(n);

    for i in 1..=n {
        tree.update(1, 1, n, i, -heights[i]);
    }

    for i in 0..n {
        ret[i] = -(queries[i].right as i64 - queries[i].left as i64 + 1)
            * tree.query(1, 1, n, queries[i].left, queries[i].right);
    }

    let mut stack = Vec::new();
    let mut left = vec![0; n + 1];
    let mut right = vec![0; n + 1];

    for i in 1..=n {
        while !stack.is_empty() && heights[stack[stack.len() - 1]] >= heights[i] {
            stack.pop();
        }

        left[i] = if stack.is_empty() {
            1
        } else {
            stack[stack.len() - 1] + 1
        };

        stack.push(i);
    }

    stack.clear();

    for i in (1..=n).rev() {
        while !stack.is_empty() && heights[stack[stack.len() - 1]] >= heights[i] {
            stack.pop();
        }

        right[i] = if stack.is_empty() {
            n
        } else {
            stack[stack.len() - 1] - 1
        };

        stack.push(i);
    }

    let mut markes = vec![Vec::new(); n + 1];

    for i in 1..=n {
        markes[right[i]].push((heights[i], i));
    }

    let mut tree_cht = SegmentTreeCHT::new(n);
    let mut tree = SegmentTree::new(n);

    queries.sort_by(|a, b| a.right.cmp(&b.right));

    let mut idx = 0;

    for i in 1..=n {
        markes[i].sort();
        markes[i].reverse();

        for &(_, j) in markes[i].iter() {
            tree_cht.update(
                1,
                1,
                n,
                left[j],
                right[j],
                -heights[j],
                heights[j] * (right[j] as i64 + 1),
            );
            tree.update(
                1,
                1,
                n,
                left[j],
                (right[j] as i64 - left[j] as i64 + 1) * heights[j],
            );
        }

        while idx < n && queries[idx].right == i {
            ret[queries[idx].idx] = ret[queries[idx].idx].max(tree_cht.query(
                1,
                1,
                n,
                queries[idx].left,
                queries[idx].left as i64,
            ));
            ret[queries[idx].idx] = ret[queries[idx].idx].max(tree.query(
                1,
                1,
                n,
                queries[idx].left,
                queries[idx].right,
            ));
            idx += 1;
        }
    }

    let mut markes = vec![Vec::new(); n + 1];

    for i in 1..=n {
        markes[left[i]].push((heights[i], i));
    }

    let mut tree_cht = SegmentTreeCHT::new(n);

    queries.sort_by(|a, b| b.left.cmp(&a.left));

    let mut idx = 0;

    for i in (1..=n).rev() {
        markes[i].sort();
        markes[i].reverse();

        for &(_, j) in markes[i].iter() {
            tree_cht.update(
                1,
                1,
                n,
                left[j],
                right[j],
                -heights[j],
                -heights[j] * (left[j] as i64 - 1),
            );
        }

        while idx < n && queries[idx].left == i {
            ret[queries[idx].idx] = ret[queries[idx].idx].max(tree_cht.query(
                1,
                1,
                n,
                queries[idx].right,
                -(queries[idx].right as i64),
            ));
            idx += 1;
        }
    }

    for val in ret {
        writeln!(out, "{val}").unwrap();
    }
}
