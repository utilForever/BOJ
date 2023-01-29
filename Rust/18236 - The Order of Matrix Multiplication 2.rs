use io::Write;
use std::{cmp::Ordering, collections::BinaryHeap, io, str};

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

#[derive(Clone)]
struct HArc {
    u: usize,
    v: usize,
    low: usize,
    bm: i64,
    bj: i64,
    base: i64,
    uv: i64,
}

impl HArc {
    fn new(u: usize, v: usize, w: &Vec<i64>, cp: &Vec<i64>) -> HArc {
        HArc {
            u,
            v,
            low: if w[u] <= w[v] { u } else { v },
            bm: cp[v] - cp[u] - w[u] * w[v],
            bj: 0,
            base: cp[v] - cp[u] - w[u] * w[v],
            uv: w[u] * w[v],
        }
    }

    fn is_contain(&self, arc: &HArc) -> bool {
        self.u <= arc.u && self.v >= arc.v
    }

    fn get_s(&self) -> i64 {
        self.bj / self.bm
    }
}

struct HuShing {
    w: Vec<i64>,
    cp: Vec<i64>,
    n: usize,

    h: Vec<HArc>,
    child: Vec<Vec<usize>>,
    connect: Vec<Vec<usize>>,
    ceiling: Vec<Vec<usize>>,
}

impl HuShing {
    fn solve(&mut self) -> i64 {
        self.init();

        if self.n < 2 {
            return 0;
        }

        if self.n == 2 {
            return self.w[0] * self.w[1];
        }

        self.construct();
        self.process_dfs(0);

        self.get_ans(0)
    }

    fn init(&mut self) {
        self.h.reserve(self.n);
    }

    fn construct(&mut self) {
        let index_of_min = self
            .w
            .iter()
            .enumerate()
            .min_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap_or(Ordering::Equal))
            .map(|(index, _)| index)
            .unwrap();
        self.w.rotate_left(index_of_min);

        self.w.push(self.w[0]);

        for i in 1..=self.n {
            self.cp[i] = self.cp[i - 1] + self.w[i] * self.w[i - 1];
        }

        self.build_tree();
    }

    fn build_tree(&mut self) {
        let arc = self.process_one_sweep();
        let mut stack = Vec::with_capacity(arc.len() * 2);

        self.h.push(HArc::new(0, self.n, &self.w, &self.cp));

        for i in 0..arc.len() {
            let (u, v) = arc[i];
            if u == 0 || v == 0 {
                continue;
            }

            self.h.push(HArc::new(u, v, &self.w, &self.cp));

            while !stack.is_empty()
                && self
                    .h
                    .last()
                    .unwrap()
                    .is_contain(&self.h[*stack.last().unwrap()])
            {
                self.child[self.h.len() - 1].push(*stack.last().unwrap());
                stack.pop();
            }

            stack.push(self.h.len() - 1);
        }

        while !stack.is_empty() {
            self.child[0].push(*stack.last().unwrap());
            stack.pop();
        }
    }

    fn process_one_sweep(&self) -> Vec<(usize, usize)> {
        let mut stack = Vec::with_capacity(self.n);
        let mut arcs = Vec::with_capacity(self.n);

        for i in 0..self.n {
            while stack.len() >= 2 && self.w[*stack.last().unwrap()] > self.w[i] {
                arcs.push((stack[stack.len() - 2], i));
                stack.pop();
            }

            stack.push(i);
        }

        while stack.len() >= 3 {
            arcs.push((0, stack[stack.len() - 2]));
            stack.pop();
        }

        arcs
    }

    fn process_dfs(&mut self, vertex: usize) {
        let mut heap = BinaryHeap::new();

        for i in 0..self.child[vertex].len() {
            let next = self.child[vertex][i];
            self.process_dfs(next);

            self.h[vertex].bm -= self.h[next].base;
            heap.push((self.h[next].get_s(), next));
        }

        while !heap.is_empty() && heap.peek().unwrap().0 >= self.w[self.h[vertex].low] {
            let idx = heap.peek().unwrap().1;
            heap.pop();

            self.h[vertex].bm += self.h[idx].bm;
            self.remove_arc(idx);

            for i in 0..self.ceiling[idx].len() {
                let next = self.ceiling[idx][i];
                heap.push((self.h[next].get_s(), next));
            }
        }

        self.h[vertex].bj = self.get_fan_cost(vertex);

        while !heap.is_empty() {
            let (s, idx) = *heap.peek().unwrap();
            heap.pop();

            if self.h[vertex].get_s() <= s {
                self.h[vertex].bm += self.h[idx].bm;
                self.remove_arc(idx);
                self.h[vertex].bj += self.h[idx].bj;

                for i in 0..self.ceiling[idx].len() {
                    let next = self.ceiling[idx][i];
                    heap.push((self.h[next].get_s(), next));
                }
            } else {
                self.ceiling[vertex].push(idx);
            }
        }

        self.add_arc(vertex);
    }

    fn add_arc(&mut self, vertex: usize) {
        self.connect[self.h[vertex].u].push(vertex);
        self.connect[self.h[vertex].v].push(vertex);
    }

    fn remove_arc(&mut self, vertex: usize) {
        self.connect[self.h[vertex].u].pop();
        self.connect[self.h[vertex].v].pop();
    }

    fn get_fan_cost(&self, vertex: usize) -> i64 {
        let arc = &self.h[vertex];
        self.w[arc.low] * (arc.bm + arc.uv - self.exclude_cp(vertex))
    }

    fn exclude_cp(&self, num: usize) -> i64 {
        if num == 0 {
            return self.w[0] * self.w[1] + self.w[0] * self.w[self.n - 1];
        }

        let arc = &self.h[num];

        if arc.low == arc.u {
            if self.connect[arc.u].is_empty()
                || !arc.is_contain(&self.h[*self.connect[arc.u].last().unwrap()])
            {
                self.w[arc.u] * self.w[arc.u + 1]
            } else {
                self.h[*self.connect[arc.u].last().unwrap()].uv
            }
        } else {
            if self.connect[arc.v].is_empty()
                || !arc.is_contain(&self.h[*self.connect[arc.v].last().unwrap()])
            {
                self.w[arc.v] * self.w[arc.v - 1]
            } else {
                self.h[*self.connect[arc.v].last().unwrap()].uv
            }
        }
    }

    fn get_ans(&self, vertex: usize) -> i64 {
        let mut sum = self.h[vertex].bj;
        sum += self.ceiling[vertex]
            .iter()
            .map(|&next| self.get_ans(next))
            .sum::<i64>();

        sum
    }
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let n = scan.token::<usize>();
    let mut matrix = Vec::with_capacity(n + 1);

    for i in 0..n {
        let (r, c) = (scan.token::<i64>(), scan.token::<i64>());
        matrix.push(r);

        if i == n - 1 {
            matrix.push(c);
        }
    }

    let len_matrix = matrix.len();
    let mut hu_shing = HuShing {
        w: matrix,
        cp: vec![0; len_matrix + 1],
        n: len_matrix,
        h: Vec::new(),
        child: vec![Vec::new(); len_matrix],
        connect: vec![Vec::new(); len_matrix + 1],
        ceiling: vec![Vec::new(); len_matrix],
    };

    writeln!(out, "{}", hu_shing.solve()).unwrap();
}
