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

    pub fn line(&mut self) -> String {
        let mut input = String::new();
        self.reader.read_line(&mut input).expect("Failed read");
        input
    }
}

struct SegmentTree {
    size: usize,
    data: Vec<Vec<i64>>,
}

impl SegmentTree {
    pub fn new(n: usize) -> Self {
        let mut real_n = 1;
        while real_n < n {
            real_n *= 2;
        }

        Self {
            size: n,
            data: vec![Vec::new(); real_n * 4],
        }
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
        if start > node_end || end < node_start {
            return;
        }

        if start <= node_start && node_end <= end {
            if val > 0 {
                self.data[node].push(val);
            } else {
                self.data[node].pop();
            }

            return;
        }

        let mid = (node_start + node_end) / 2;

        self.update_internal(start, end, val, node * 2, node_start, mid);
        self.update_internal(start, end, val, node * 2 + 1, mid + 1, node_end);
    }

    pub fn query(&mut self, idx: usize) -> i64 {
        self.query_internal(idx, 1, 1, self.size)
    }

    fn query_internal(
        &mut self,
        idx: usize,
        node: usize,
        node_start: usize,
        node_end: usize,
    ) -> i64 {
        if node_start == node_end {
            if !self.data[node].is_empty() {
                return *self.data[node].last().unwrap();
            } else {
                return 0;
            }
        }

        let mid = (node_start + node_end) / 2;

        let ret = if idx <= mid {
            self.query_internal(idx, node * 2, node_start, mid)
        } else {
            self.query_internal(idx, node * 2 + 1, mid + 1, node_end)
        };

        if ret != 0 {
            ret
        } else if !self.data[node].is_empty() {
            *self.data[node].last().unwrap()
        } else {
            0
        }
    }
}

struct UnionFind {
    parent: Vec<usize>,
}

impl UnionFind {
    fn new(n: usize) -> Self {
        UnionFind { parent: vec![0; n] }
    }

    fn init(&mut self) {
        for i in 0..self.parent.len() {
            self.parent[i] = i;
        }
    }

    fn find(&mut self, x: usize) -> usize {
        if self.parent[x] != x {
            self.parent[x] = self.find(self.parent[x]);
        }

        self.parent[x]
    }

    fn union(&mut self, x: usize, y: usize) {
        let root_x = self.find(x);
        let root_y = self.find(y);

        if root_x != root_y {
            self.parent[root_y] = root_x;
        }
    }
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let n = scan.token::<usize>();
    let mut positions_x = vec![i64::MIN];
    let mut positions_y = vec![i64::MIN];
    let mut rectangles = vec![((0, 0), (0, 0)); n + 1];
    let mut colors = vec![0; n + 1];

    for i in 1..=n {
        let (x1, y1, x2, y2) = (
            scan.token::<i64>(),
            scan.token::<i64>(),
            scan.token::<i64>(),
            scan.token::<i64>(),
        );
        positions_x.push(x1);
        positions_x.push(x2);
        positions_y.push(y1);
        positions_y.push(y2);
        rectangles[i] = ((x1, y1), (x2, y2));

        for _ in 0..4 {
            let c = scan.token::<i64>();
            colors[i] |= 1 << (c - 1);
        }
    }

    let q = scan.token::<usize>();
    let mut queries = vec![((0, 0), (0, 0)); q + 1];

    for i in 1..=q {
        let (xs, ys, xe, ye) = (
            scan.token::<i64>(),
            scan.token::<i64>(),
            scan.token::<i64>(),
            scan.token::<i64>(),
        );
        positions_x.push(xs);
        positions_x.push(xe);
        positions_y.push(ys);
        positions_y.push(ye);
        queries[i] = ((xs, ys), (xe, ye));
    }

    positions_x.sort_unstable();
    positions_x.dedup();
    positions_y.sort_unstable();
    positions_y.dedup();

    let mut rectangles_compressed = vec![((0, 0), (0, 0)); n + 1];
    let mut queries_compressed = vec![((0, 0), (0, 0)); q + 1];
    let mut commands = Vec::with_capacity(2 * (n + q));

    let find = |positions: &Vec<i64>, point: i64| -> usize {
        match positions.binary_search(&point) {
            Ok(idx) => idx,
            Err(idx) => idx,
        }
    };

    for i in 1..=n {
        let ((x1, y1), (x2, y2)) = rectangles[i];
        let x1 = find(&positions_x, x1);
        let y1 = find(&positions_y, y1);
        let x2 = find(&positions_x, x2);
        let y2 = find(&positions_y, y2);

        rectangles_compressed[i] = ((x1, y1), (x2, y2));
        commands.push((x1, 0, i));
        commands.push((x2, 1, i));
    }

    for i in 1..=q {
        let ((xs, ys), (xe, ye)) = queries[i];
        let xs = find(&positions_x, xs);
        let ys = find(&positions_y, ys);
        let xe = find(&positions_x, xe);
        let ye = find(&positions_y, ye);

        queries_compressed[i] = ((xs, ys), (xe, ye));
        commands.push((xs, 2, i));
        commands.push((xe, 3, i));
    }

    commands.sort_unstable();

    let mut tree = SegmentTree::new(commands.len());
    let mut graph = vec![0; n + 1];
    let mut tree_queries = vec![(0, 0); q + 1];

    for &(_, op, idx) in commands.iter() {
        if op == 0 || op == 1 {
            let (y1, y2) = (
                rectangles_compressed[idx].0 .1,
                rectangles_compressed[idx].1 .1,
            );

            if op == 0 {
                graph[idx] = tree.query(y1) as usize;
            }

            tree.update(y1, y2, if op == 0 { idx as i64 } else { -(idx as i64) });
        } else {
            let y = if op == 2 {
                queries_compressed[idx].0 .1
            } else {
                queries_compressed[idx].1 .1
            };
            let val = tree.query(y);

            if op == 2 {
                tree_queries[idx].0 = val;
            } else {
                tree_queries[idx].1 = val;
            }
        }
    }

    let mut union_find = UnionFind::new(n + 1);
    let mut ret = vec![u32::MAX; q + 1];

    for bit in 0..64i64 {
        union_find.init();

        let cnt_ones = bit.count_ones();

        for i in 1..=n {
            if (bit & colors[i]) != 0 {
                union_find.union(i, graph[i]);
            }
        }

        for i in 1..=q {
            let (a, b) = tree_queries[i];

            if union_find.find(a as usize) == union_find.find(b as usize) {
                ret[i] = ret[i].min(cnt_ones);
            }
        }
    }

    for i in 1..=q {
        writeln!(out, "{}", ret[i]).unwrap();
    }
}
