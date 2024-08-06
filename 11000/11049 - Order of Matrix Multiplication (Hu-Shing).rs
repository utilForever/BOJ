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
    u: i32,
    v: i32,
    low: i32,
    bm: i64,
    bj: i64,
    base: i64,
    uv: i64,
}

impl HArc {
    fn new(u: i32, v: i32, w: &Vec<i64>, cp: &Vec<i64>) -> HArc {
        HArc {
            u,
            v,
            low: if w[u as usize] <= w[v as usize] { u } else { v },
            bm: cp[v as usize] - cp[u as usize] - w[u as usize] * w[v as usize],
            bj: 0,
            base: cp[v as usize] - cp[u as usize] - w[u as usize] * w[v as usize],
            uv: w[u as usize] * w[v as usize],
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
    n: i32,

    h: Vec<HArc>,
    child: Vec<Vec<i32>>,
    connect: Vec<Vec<i32>>,
    ceiling: Vec<Vec<i32>>,
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
        self.h.reserve(self.n as usize);
    }

    fn construct(&mut self) {
        let index_of_min: Option<usize> = self
            .w
            .iter()
            .enumerate()
            .min_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap_or(Ordering::Equal))
            .map(|(index, _)| index);
        self.w.rotate_left(index_of_min.unwrap() as usize);

        self.w.push(self.w[0]);

        for i in 1..=self.n as usize {
            self.cp[i] = self.cp[i - 1] + self.w[i] * self.w[i - 1];
        }

        self.build_tree();
    }

    fn build_tree(&mut self) {
        let arc: Vec<(i32, i32)> = self.process_one_sweep();
        let mut stack: Vec<i32> = Vec::new();

        self.h.push(HArc::new(0, self.n, &self.w, &self.cp));

        for i in 0..arc.len() {
            let u = arc[i].0;
            let v = arc[i].1;

            if u == 0 || v == 0 {
                continue;
            }

            self.h.push(HArc::new(u, v, &self.w, &self.cp));

            while !stack.is_empty()
                && self
                    .h
                    .last()
                    .unwrap()
                    .is_contain(&self.h[*stack.last().unwrap() as usize])
            {
                self.child[self.h.len() - 1].push(*stack.last().unwrap());
                stack.pop();
            }

            stack.push(self.h.len() as i32 - 1);
        }

        while !stack.is_empty() {
            self.child[0].push(*stack.last().unwrap());
            stack.pop();
        }
    }

    fn process_one_sweep(&self) -> Vec<(i32, i32)> {
        let mut stack: Vec<i32> = Vec::new();
        let mut arcs: Vec<(i32, i32)> = Vec::new();

        for i in 0..self.n as usize {
            while stack.len() >= 2 && self.w[*stack.last().unwrap() as usize] > self.w[i] {
                arcs.push((stack[stack.len() - 2], i as i32));
                stack.pop();
            }

            stack.push(i as i32);
        }

        while stack.len() >= 3 {
            arcs.push((0, stack[stack.len() - 2]));
            stack.pop();
        }

        arcs
    }

    fn process_dfs(&mut self, vertex: i32) {
        let mut heap: BinaryHeap<(i64, i32)> = BinaryHeap::new();

        for i in 0..self.child[vertex as usize].len() {
            let next = self.child[vertex as usize][i];

            self.process_dfs(next);

            self.h[vertex as usize].bm -= self.h[next as usize].base;
            heap.push((self.h[next as usize].get_s(), next));
        }

        while !heap.is_empty()
            && heap.peek().unwrap().0 >= self.w[self.h[vertex as usize].low as usize]
        {
            let idx = heap.peek().unwrap().1;

            heap.pop();

            self.h[vertex as usize].bm += self.h[idx as usize].bm;

            self.remove_arc(idx);

            for i in 0..self.ceiling[idx as usize].len() {
                let next = self.ceiling[idx as usize][i];
                heap.push((self.h[next as usize].get_s(), next));
            }
        }

        self.h[vertex as usize].bj = self.get_fan_cost(vertex);

        while !heap.is_empty() {
            let s = heap.peek().unwrap().0;
            let idx = heap.peek().unwrap().1;

            heap.pop();

            if self.h[vertex as usize].get_s() <= s {
                self.h[vertex as usize].bm += self.h[idx as usize].bm;

                self.remove_arc(idx);

                self.h[vertex as usize].bj += self.h[idx as usize].bj;

                for i in 0..self.ceiling[idx as usize].len() {
                    let next = self.ceiling[idx as usize][i];
                    heap.push((self.h[next as usize].get_s(), next));
                }
            } else {
                self.ceiling[vertex as usize].push(idx);
            }
        }

        self.add_arc(vertex);
    }

    fn add_arc(&mut self, vertex: i32) {
        self.connect[self.h[vertex as usize].u as usize].push(vertex);
        self.connect[self.h[vertex as usize].v as usize].push(vertex);
    }

    fn remove_arc(&mut self, vertex: i32) {
        self.connect[self.h[vertex as usize].u as usize].pop();
        self.connect[self.h[vertex as usize].v as usize].pop();
    }

    fn get_fan_cost(&self, vertex: i32) -> i64 {
        let arc = &self.h[vertex as usize];

        self.w[arc.low as usize] * (arc.bm + arc.uv - self.exclude_cp(vertex))
    }

    fn exclude_cp(&self, num: i32) -> i64 {
        if num == 0 {
            return self.w[0] * self.w[1] + self.w[0] * self.w[self.n as usize - 1];
        }

        let arc = &self.h[num as usize];

        if arc.low == arc.u {
            if self.connect[arc.u as usize].is_empty()
                || !arc.is_contain(&self.h[*self.connect[arc.u as usize].last().unwrap() as usize])
            {
                return self.w[arc.u as usize] * self.w[arc.u as usize + 1];
            } else {
                return self.h[*self.connect[arc.u as usize].last().unwrap() as usize].uv;
            }
        } else {
            if self.connect[arc.v as usize].is_empty()
                || !arc.is_contain(&self.h[*self.connect[arc.v as usize].last().unwrap() as usize])
            {
                return self.w[arc.v as usize] * self.w[arc.v as usize - 1];
            } else {
                return self.h[*self.connect[arc.v as usize].last().unwrap() as usize].uv;
            }
        }
    }

    fn get_ans(&self, vertex: i32) -> i64 {
        let mut sum = self.h[vertex as usize].bj;

        for i in 0..self.ceiling[vertex as usize].len() {
            let next = self.ceiling[vertex as usize][i];

            sum += self.get_ans(next);
        }

        sum
    }
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let n = scan.token::<i64>();

    let mut matrix = Vec::new();

    for i in 0..n {
        let (r, c) = (scan.token::<i64>(), scan.token::<i64>());
        matrix.push(r);

        if i == n - 1 {
            matrix.push(c);
        }
    }

    let mut hu_shing = HuShing {
        w: matrix.clone(),
        cp: vec![0; matrix.len() as usize + 1],
        n: matrix.len() as i32,
        h: Vec::new(),
        child: vec![Vec::new(); matrix.len() as usize],
        connect: vec![Vec::new(); matrix.len() as usize + 1],
        ceiling: vec![Vec::new(); matrix.len() as usize],
    };

    writeln!(out, "{}", hu_shing.solve()).unwrap();
}
