use io::Write;
use std::{
    cmp::Ordering,
    collections::{HashMap, HashSet},
    io, str,
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

#[derive(Clone, Copy)]
struct Point {
    x: i64,
    y: i64,
}

impl Point {
    fn new(x: i64, y: i64) -> Self {
        Self { x, y }
    }
}

#[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Hash)]
struct Edge {
    a: i64,
    b: i64,
}

impl Edge {
    fn new(a: i64, b: i64) -> Self {
        Self { a, b }
    }

    fn from_vertices(points: &[Point]) -> Vec<Edge> {
        let n = points.len();

        (0..n)
            .map(|idx| {
                let (x1, y1) = (points[idx].x, points[idx].y);
                let (x2, y2) = (points[(idx + 1) % n].x, points[(idx + 1) % n].y);

                Edge::new(x2 - x1, y2 - y1)
            })
            .collect()
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

fn booth(seq: &[Edge]) -> usize {
    let n = seq.len();
    let mut i = 0;
    let mut j = 1;
    let mut k = 0;

    while i < n && j < n && k < n {
        let a = seq[(i + k) % n];
        let b = seq[(j + k) % n];

        match a.cmp(&b) {
            Ordering::Equal => k += 1,
            Ordering::Less => {
                j += k + 1;

                if j == i {
                    j += 1;
                }

                k = 0;
            }
            Ordering::Greater => {
                i += k + 1;

                if i == j {
                    i += 1;
                }

                k = 0;
            }
        }
    }

    i.min(j) % n
}

fn canonical(edges: &[Edge]) -> Vec<Edge> {
    let n = edges.len();
    let rev = edges
        .iter()
        .rev()
        .map(|&e| Edge::new(-e.a, -e.b))
        .collect::<Vec<_>>();

    let s1 = {
        let start = booth(edges);
        (0..n).map(|k| edges[(start + k) % n]).collect::<Vec<_>>()
    };
    let s2 = {
        let start = booth(&rev);
        (0..n).map(|k| rev[(start + k) % n]).collect::<Vec<_>>()
    };

    if s1 < s2 {
        s1
    } else {
        s2
    }
}

fn make_key_id(edges: &[Edge], pool: &mut HashMap<Vec<Edge>, usize>) -> usize {
    if let Some(&id) = pool.get(edges) {
        id
    } else {
        let id = pool.len();
        pool.insert(edges.to_vec(), id);
        id
    }
}

fn calculate_dp(
    idxes: &[usize],
    polygons: &[[usize; 3]],
    dp: &mut [Option<i64>],
    n: usize,
    mask: usize,
) -> i64 {
    if let Some(val) = dp[mask] {
        return val;
    }

    let mut moves = HashSet::new();

    for p in 0..n {
        let state = (mask >> (2 * p)) & 3;

        if state == 0 {
            for axis in 0..2 {
                let state_new = if axis == 0 { 1 } else { 2 };
                let mut mask_new = mask & !(3 << (2 * p));

                mask_new |= state_new << (2 * p);

                let polygons_curr = polygons[idxes[p]][state_new as usize];
                let mut opponent: Option<usize> = None;

                for q in 0..n {
                    if q == p {
                        continue;
                    }

                    let state_q = (mask_new >> (2 * q)) & 3;

                    if state_q == 3 {
                        continue;
                    }

                    let key_q = polygons[idxes[q]][state_q];

                    if key_q == polygons_curr {
                        opponent = Some(q);
                        break;
                    }
                }

                if let Some(q) = opponent {
                    mask_new &= !(3 << (2 * p));
                    mask_new |= 3 << (2 * p);
                    mask_new &= !(3 << (2 * q));
                    mask_new |= 3 << (2 * q);
                }

                moves.insert(calculate_dp(idxes, polygons, dp, n, mask_new));
            }
        }
    }

    let mut ret = 0;

    while moves.contains(&ret) {
        ret += 1;
    }

    dp[mask] = Some(ret);

    ret
}

fn calculate_grundy(idxes: &[usize], polygons: &[[usize; 3]]) -> i64 {
    let n = idxes.len();
    let mut dp = vec![None; 1 << (2 * n)];

    calculate_dp(idxes, polygons, &mut dp, n, 0)
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let n = scan.token::<usize>();
    let mut pool: HashMap<Vec<Edge>, usize> = HashMap::new();
    let mut polygons = vec![[0; 3]; n];

    for i in 0..n {
        let c = scan.token::<usize>();
        let mut points = Vec::with_capacity(c);

        for _ in 0..c {
            let (x, y) = (scan.token::<i64>(), scan.token::<i64>());
            points.push(Point::new(x, y));
        }

        let edges = Edge::from_vertices(&points);

        let k0 = make_key_id(&canonical(&edges), &mut pool);

        // Flip horizontally
        let flipped_h = edges
            .iter()
            .map(|e| Edge::new(-e.a, e.b))
            .collect::<Vec<_>>();
        let k1 = make_key_id(&canonical(&flipped_h), &mut pool);

        // Flip vertically
        let flipped_v = edges
            .iter()
            .map(|e| Edge::new(e.a, -e.b))
            .collect::<Vec<_>>();
        let k2 = make_key_id(&canonical(&flipped_v), &mut pool);

        polygons[i] = [k0, k1, k2];
    }

    let mut positions = HashMap::new();

    for i in 0..n {
        positions.insert(polygons[i][0], i);
    }

    let mut union_find = UnionFind::new(n);
    union_find.init();

    for i in 0..n {
        if let Some(&pos) = positions.get(&polygons[i][1]) {
            if i != pos {
                union_find.union(i, pos);
            }
        }

        if let Some(&pos) = positions.get(&polygons[i][2]) {
            if i != pos {
                union_find.union(i, pos);
            }
        }
    }

    let mut components = HashMap::new();

    for i in 0..n {
        let root = union_find.find(i);
        components.entry(root).or_insert_with(Vec::new).push(i);
    }

    let mut ret = 0;

    for comp in components.values() {
        let g = calculate_grundy(comp, &polygons);
        ret ^= g;
    }

    writeln!(out, "{}", if ret != 0 { 1 } else { 0 }).unwrap();
}
