use io::Write;
use std::collections::VecDeque;
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

struct UnionFind {
    elements: Vec<i32>,
}

#[allow(dead_code)]
impl UnionFind {
    fn new(n: usize) -> Self {
        UnionFind {
            elements: vec![-1; n],
        }
    }

    fn find(&mut self, x: usize) -> usize {
        if self.elements[x] < 0 {
            x
        } else {
            let root = self.find(self.elements[x] as usize);
            self.elements[x] = root as i32;
            root
        }
    }

    fn same_set(&mut self, a: usize, b: usize) -> bool {
        self.find(a) == self.find(b)
    }

    fn size(&mut self, x: usize) -> i32 {
        let root = self.find(x);
        -self.elements[root]
    }

    fn join(&mut self, a: usize, b: usize) -> bool {
        let mut a = self.find(a);
        let mut b = self.find(b);

        if a == b {
            return false;
        }

        if self.elements[a] > self.elements[b] {
            std::mem::swap(&mut a, &mut b);
        }

        self.elements[a] += self.elements[b];
        self.elements[b] = a as i32;

        true
    }
}

trait Matroid {
    fn check(&mut self, x: usize) -> bool;
    fn add(&mut self, x: usize);
    fn clear(&mut self);
}

struct ColorMat {
    cnt: Vec<i32>,
    colors: Vec<usize>,
}

impl ColorMat {
    fn new(len: usize, colors: Vec<usize>) -> Self {
        ColorMat {
            cnt: vec![0; len],
            colors,
        }
    }
}

impl Matroid for ColorMat {
    fn check(&mut self, x: usize) -> bool {
        let c = self.colors[x];
        self.cnt[c] == 0
    }

    fn add(&mut self, x: usize) {
        let c = self.colors[x];
        self.cnt[c] += 1;
    }

    fn clear(&mut self) {
        for val in self.cnt.iter_mut() {
            *val = 0;
        }
    }
}

struct GraphMat {
    union_find: UnionFind,
    edges: Vec<[usize; 2]>,
}

impl GraphMat {
    fn new(n: usize, edges: Vec<[i32; 2]>) -> Self {
        let edges: Vec<[usize; 2]> = edges
            .into_iter()
            .map(|e| [e[0] as usize, e[1] as usize])
            .collect();

        GraphMat {
            union_find: UnionFind::new(n),
            edges,
        }
    }
}

impl Matroid for GraphMat {
    fn check(&mut self, x: usize) -> bool {
        !self.union_find.same_set(self.edges[x][0], self.edges[x][1])
    }

    fn add(&mut self, x: usize) {
        self.union_find.join(self.edges[x][0], self.edges[x][1]);
    }

    fn clear(&mut self) {
        let n = self.union_find.elements.len();
        self.union_find = UnionFind::new(n);
    }
}

struct MatroidIntersection<M1: Matroid, M2: Matroid> {
    n: usize,
    independent_set: Vec<bool>,
    matroid1: M1,
    matroid2: M2,
}

impl<M1: Matroid, M2: Matroid> MatroidIntersection<M1, M2> {
    fn new(m1: M1, m2: M2, n: usize) -> Self {
        MatroidIntersection {
            n,
            independent_set: vec![false; n],
            matroid1: m1,
            matroid2: m2,
        }
    }

    fn solve(&mut self) -> Vec<usize> {
        for i in 0..self.n {
            if self.matroid1.check(i) && self.matroid2.check(i) {
                self.independent_set[i] = true;
                self.matroid1.add(i);
                self.matroid2.add(i);
            }
        }

        while self.augment() {}

        let mut ret = Vec::new();

        for i in 0..self.n {
            if self.independent_set[i] {
                ret.push(i);
            }
        }

        ret
    }

    fn augment(&mut self) -> bool {
        let n = self.n;
        let mut frm = vec![-1; n];
        let mut q = VecDeque::new();
        q.push_back(n as i32);

        let mut fwd_e = |a: i32, frm: &mut [i32]| -> Vec<usize> {
            let a_usize = a as usize;
            self.matroid1.clear();
            for v in 0..n {
                if self.independent_set[v] && v != a_usize {
                    self.matroid1.add(v);
                }
            }

            let mut ans = Vec::new();
            for b in 0..n {
                if !self.independent_set[b] && frm[b] == -1 && self.matroid1.check(b) {
                    ans.push(b);
                    frm[b] = a;
                }
            }
            ans
        };

        let mut back_e = |b: i32, frm: &mut [i32], q: &mut VecDeque<i32>| -> i32 {
            self.matroid2.clear();
            for cas in 0..2 {
                for v in 0..n {
                    let cond = (v as i32 == b || self.independent_set[v])
                        && ((frm[v] == -1) == (cas == 1));
                    if cond {
                        if !self.matroid2.check(v) {
                            if cas == 1 {
                                frm[v] = b;
                                q.push_back(v as i32);
                                return v as i32;
                            } else {
                                return -1;
                            }
                        }
                        self.matroid2.add(v);
                    }
                }
            }
            n as i32
        };

        while let Some(a) = q.pop_front() {
            for b in fwd_e(a, &mut frm) {
                loop {
                    let c = back_e(b as i32, &mut frm, &mut q);
                    if c < 0 {
                        break;
                    }
                    if c == n as i32 {
                        let mut b_i32 = b as i32;
                        while b_i32 != n as i32 {
                            self.independent_set[b_i32 as usize] =
                                !self.independent_set[b_i32 as usize];
                            b_i32 = frm[b_i32 as usize];
                        }
                        return true;
                    }
                }
            }
        }

        false
    }
}

// Reference: https://codeforces.com/blog/entry/69287
// Reference: https://github.com/infossm/infossm.github.io/blob/master/_posts/2019-05-08-introduction-to-matroid.md
// Reference: https://github.com/infossm/infossm.github.io/blob/master/_posts/2019-06-17-Matroid-Intersection.md
// Reference: https://github.com/kth-competitive-programming/kactl/pull/172/files
fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    loop {
        let r = scan.token::<usize>();

        if r == 0 {
            break;
        }

        let mut coins = Vec::with_capacity(2 * r);
        let mut colors = Vec::with_capacity(2 * r);

        for i in 0..r {
            let (a, b, c, d) = (
                scan.token::<i32>(),
                scan.token::<i32>(),
                scan.token::<i32>(),
                scan.token::<i32>(),
            );

            coins.push([a, b]);
            coins.push([c, d]);
            colors.push(i);
            colors.push(i);
        }

        let graph_mat = GraphMat::new(10000, coins);
        let color_mat = ColorMat::new(r, colors);
        let mut matroid_intersection = MatroidIntersection::new(color_mat, graph_mat, 2 * r);

        writeln!(out, "{}", 2 * matroid_intersection.solve().len()).unwrap();
    }
}
