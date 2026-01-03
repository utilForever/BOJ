use io::Write;
use std::{collections::BTreeMap, io, ptr::null_mut, str};

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
pub struct SuffixAutomaton {
    pub st: Vec<SAState>,
    pub sz: usize,
    pub last: usize,
}

#[derive(Default, Clone)]
pub struct SAState {
    pub len: usize,
    pub link: i32,
    pub next: BTreeMap<char, usize>,
}

impl SuffixAutomaton {
    pub fn from_str(s: &str) -> Self {
        let mut sa = Self::new(s.len());

        for ch in s.chars() {
            sa.add(ch);
        }

        sa
    }

    pub fn new(n: usize) -> Self {
        let mut sa = Self {
            st: vec![],
            sz: 1,
            last: 0,
        };

        for _ in 0..(2 * n) {
            sa.st.push(SAState::default());
        }

        sa.st[0].len = 0;
        sa.st[0].link = -1;

        sa
    }

    pub fn add(&mut self, c: char) {
        let cur = self.sz;

        self.sz += 1;
        self.st[cur].len = self.st[self.last].len + 1;

        let mut p = self.last as i32;

        while p != -1 && !self.st[p as usize].next.contains_key(&c) {
            self.st[p as usize].next.insert(c, cur);
            p = self.st[p as usize].link;
        }

        if p == -1 {
            self.st[cur].link = 0;
        } else {
            let pu = p as usize;
            let q = self.st[pu].next[&c];

            if self.st[pu].len + 1 == self.st[q].len {
                self.st[cur].link = q as i32;
            } else {
                let clone = self.sz;

                self.sz += 1;
                self.st[clone].len = self.st[pu].len + 1;
                self.st[clone].next = self.st[q].next.clone();
                self.st[clone].link = self.st[q].link;

                while p != -1 && *self.st[p as usize].next.get(&c).unwrap() == q {
                    self.st[p as usize].next.insert(c, clone);
                    p = self.st[p as usize].link;
                }

                self.st[cur].link = clone as i32;
                self.st[q].link = self.st[cur].link;
            }
        }

        self.last = cur;
    }

    pub fn count_unique_substring(&self) -> i64 {
        let mut tot = 0;

        for i in 1..self.sz {
            tot += (self.st[i].len - self.st[self.st[i].link as usize].len) as i64;
        }

        tot
    }
}

struct LazySegmentTree {
    size: usize,
    sum: Vec<i64>,
    sum2: Vec<i64>,
    lazy: Vec<i64>,
}

impl LazySegmentTree {
    pub fn new(n: usize) -> Self {
        let mut real_n = 1;

        while real_n < n {
            real_n *= 2;
        }

        Self {
            size: n,
            sum: vec![0; real_n * 4],
            sum2: vec![0; real_n * 4],
            lazy: vec![0; real_n * 4],
        }
    }

    fn propagate(&mut self, node: usize, start: usize, end: usize) {
        if self.lazy[node] == 0 {
            return;
        }

        if start != end {
            self.lazy[node * 2] += self.lazy[node];
            self.lazy[node * 2 + 1] += self.lazy[node];
        }

        self.sum[node] += self.lazy[node] * (end - start + 1) as i64;
        self.sum2[node] += self.lazy[node] * ((start + end) as i64 * (end - start + 1) as i64 / 2);
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

        self.sum[node] = self.sum[node * 2] + self.sum[node * 2 + 1];
        self.sum2[node] = self.sum2[node * 2] + self.sum2[node * 2 + 1];
    }

    pub fn query_sum(&mut self, start: usize, end: usize) -> i64 {
        self.query_internal(start, end, 1, 1, self.size).0
    }

    pub fn query_sum2(&mut self, start: usize, end: usize) -> i64 {
        self.query_internal(start, end, 1, 1, self.size).1
    }

    fn query_internal(
        &mut self,
        start: usize,
        end: usize,
        node: usize,
        node_start: usize,
        node_end: usize,
    ) -> (i64, i64) {
        self.propagate(node, node_start, node_end);

        if end < node_start || node_end < start {
            return (0, 0);
        }

        if start <= node_start && node_end <= end {
            return (self.sum[node], self.sum2[node]);
        }

        let mid = (node_start + node_end) / 2;
        let (sum1, sum21) = self.query_internal(start, end, node * 2, node_start, mid);
        let (sum2, sum22) = self.query_internal(start, end, node * 2 + 1, mid + 1, node_end);

        (sum1 + sum2, sum21 + sum22)
    }
}

#[derive(Debug)]
struct Node {
    left: *mut Node,
    right: *mut Node,
    parent: *mut Node,
    value: i64,
    max: i64,
    flip: bool,
    idx: usize,
    overwrite: Option<i64>,
}

impl Node {
    fn new(value: i64) -> *mut Self {
        Box::into_raw(Box::new(Self {
            left: null_mut(),
            right: null_mut(),
            parent: null_mut(),
            value,
            max: value,
            flip: false,
            idx: 0,
            overwrite: None,
        }))
    }

    unsafe fn is_root(&self) -> bool {
        self.parent.is_null()
            || ((*self.parent).left != (self as *const _ as *mut _)
                && (*self.parent).right != (self as *const _ as *mut _))
    }

    unsafe fn is_left(&self) -> bool {
        !self.parent.is_null() && (*self.parent).left == (self as *const _ as *mut _)
    }

    unsafe fn update(&mut self) {
        let mut max = self.value;

        if !self.left.is_null() {
            max = max.max((*self.left).max);
        }

        if !self.right.is_null() {
            max = max.max((*self.right).max);
        }

        self.max = max;
    }

    unsafe fn set_value(&mut self, val: i64) {
        self.value = val;
        self.max = val;
        self.overwrite = Some(val);
    }

    unsafe fn push(&mut self) {
        if let Some(val) = self.overwrite {
            if !self.left.is_null() {
                (*self.left).set_value(val);
            }

            if !self.right.is_null() {
                (*self.right).set_value(val);
            }

            self.overwrite = None;
        }

        if self.flip {
            let tmp = self.left;

            self.left = self.right;
            self.right = tmp;

            if !self.left.is_null() {
                (*self.left).flip ^= true;
            }

            if !self.right.is_null() {
                (*self.right).flip ^= true;
            }

            self.flip = false;
        }
    }

    unsafe fn rotate(&mut self) {
        let x = self as *mut Node;
        let p = (*x).parent;
        let g = (*p).parent;

        (*p).push();
        (*x).push();

        if (*x).is_left() {
            let b = (*x).right;

            (*p).left = b;

            if !b.is_null() {
                (*b).parent = p;
            }

            (*x).right = p;
            (*p).parent = x;
        } else {
            let b = (*x).left;

            (*p).right = b;

            if !b.is_null() {
                (*b).parent = p;
            }

            (*x).left = p;
            (*p).parent = x;
        }

        (*x).parent = g;

        if !g.is_null() {
            if (*g).left == p {
                (*g).left = x;
            } else if (*g).right == p {
                (*g).right = x;
            }
        }

        (*p).update();
        (*x).update();
    }
}

struct LinkCutTree {
    nodes: Vec<*mut Node>,
}

impl LinkCutTree {
    unsafe fn new(n: usize) -> Self {
        let mut nodes = vec![null_mut(); n + 1];

        for i in 1..=n {
            nodes[i] = Node::new(0);
        }

        Self { nodes }
    }

    unsafe fn push_to_root(&mut self, x: *mut Node) {
        let mut stack = Vec::new();
        let mut v = x;

        stack.push(v);

        while !(*v).is_root() {
            v = (*v).parent;
            stack.push(v);
        }

        while let Some(t) = stack.pop() {
            (*t).push();
        }
    }

    unsafe fn splay(&mut self, x: *mut Node) {
        self.push_to_root(x);

        while !(*x).is_root() {
            let p = (*x).parent;

            if !(*p).is_root() {
                let g = (*p).parent;
                let zigzig = ((*p).left == x) == ((*g).left == p);

                if zigzig {
                    (*p).rotate();
                } else {
                    (*x).rotate();
                }
            }

            (*x).rotate();
        }

        (*x).update();
    }

    unsafe fn access(
        &mut self,
        lens: &Vec<usize>,
        cache: &mut Vec<(usize, i64)>,
        x: *mut Node,
        val: i64,
    ) {
        cache.clear();

        let mut v = x;
        let mut last = null_mut();

        while !v.is_null() {
            self.splay(v);

            cache.push((lens[(*v).idx], (*v).value));

            (*v).right = last;

            if !last.is_null() {
                (*last).parent = v;
            }

            (*v).update();
            (*v).set_value(val);

            last = v;
            v = (*v).parent;
        }

        self.splay(x);
    }
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let s = scan.token::<String>().chars().collect::<Vec<_>>();
    let n = s.len();

    let mut suffix_automaton = SuffixAutomaton::new(n);
    let mut pos = vec![0; n + 1];

    for i in 1..=n {
        suffix_automaton.add(s[i - 1]);
        pos[i] = suffix_automaton.last;
    }

    let cnt = suffix_automaton.count_unique_substring() as i128;
    let mut ret = cnt * (cnt + 1) / 2;

    let mut rightmost = vec![0usize; suffix_automaton.sz];

    for i in 1..=n {
        rightmost[pos[i]] = i;
    }

    let mut len_bucket = vec![0; n + 1];

    for i in 0..suffix_automaton.sz {
        let idx = suffix_automaton.st[i].len;
        len_bucket[idx] += 1;
    }

    for i in 1..=n {
        len_bucket[i] += len_bucket[i - 1];
    }

    let mut order = vec![0; suffix_automaton.sz];

    for i in 0..suffix_automaton.sz {
        let idx = suffix_automaton.st[i].len;
        len_bucket[idx] -= 1;
        order[len_bucket[idx]] = i;
    }

    for &v in order.iter().rev() {
        let parent = suffix_automaton.st[v].link;

        if parent >= 0 {
            let parent = parent as usize;
            rightmost[parent] = rightmost[parent].max(rightmost[v]);
        }
    }

    let mut events = vec![Vec::new(); n + 1];

    for i in 1..suffix_automaton.sz {
        if rightmost[i] == 0 {
            continue;
        }

        let parent = suffix_automaton.st[i].link as usize;
        let l = rightmost[i] - suffix_automaton.st[i].len + 1;
        let r = rightmost[i] - suffix_automaton.st[parent].len;

        events[rightmost[i]].push((l, r));
    }

    let mut tree = LazySegmentTree::new(n);

    unsafe {
        let mut lct = LinkCutTree::new(suffix_automaton.sz);

        for idx in 1..=suffix_automaton.sz {
            (*lct.nodes[idx]).idx = idx - 1;
        }

        for i in 1..suffix_automaton.sz {
            let parent = suffix_automaton.st[i].link as usize;
            (*lct.nodes[i + 1]).parent = lct.nodes[parent + 1];
        }

        let mut lens = vec![0; suffix_automaton.sz];
        let mut cache = Vec::with_capacity(64);

        for i in 0..suffix_automaton.sz {
            lens[i] = suffix_automaton.st[i].len;
        }

        for i in 1..=n {
            tree.update(1, i, 1);
            lct.access(&lens, &mut cache, lct.nodes[pos[i] + 1], i as i64);

            let mut last_len = 0;

            for idx in (1..cache.len()).rev() {
                let (old_len, old_last_occur) = cache[idx];

                if old_len == 0 {
                    continue;
                }

                if old_last_occur != 0 {
                    let l = old_last_occur as usize - old_len + 1;
                    let r = old_last_occur as usize - last_len;

                    if l <= r {
                        tree.update(l, r, -1);
                    }
                }

                last_len = old_len;
            }

            for &(l, r) in &events[i] {
                let sum = tree.query_sum(l, r) as i128;
                let sum2 = tree.query_sum2(l, r) as i128;
                let mut val = sum2 - sum * (l as i128 - 1);

                if r < i {
                    val += tree.query_sum(r + 1, i) as i128 * (r as i128 - l as i128 + 1);
                }

                ret -= val;
            }
        }
    }

    writeln!(out, "{ret}").unwrap();
}
