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

type Capacity = i64;

#[derive(Clone, Copy)]
struct Edge {
    to: usize,
    rev: usize,
    cap: Capacity,
}

struct PushRelabelWithHLPP {
    n: usize,
    graph: Vec<Vec<Edge>>,
    excess: Vec<Capacity>,
    height: Vec<i32>,
    arc: Vec<i32>,
    act: Vec<Vec<i32>>,
    highest: i32,
    work: usize,
}

impl PushRelabelWithHLPP {
    fn new(n: usize) -> Self {
        Self {
            n,
            graph: vec![Vec::new(); n],
            excess: vec![0; n],
            height: vec![0; n],
            arc: vec![0; n],
            act: vec![Vec::new(); n + 1],
            highest: 0,
            work: 0,
        }
    }

    fn add_edge(&mut self, u: usize, v: usize, cap: Capacity, bidirection: bool) {
        let len_u = self.graph[u].len();
        let len_v = self.graph[v].len();

        self.graph[u].push(Edge {
            to: v,
            rev: len_v,
            cap,
        });
        self.graph[v].push(Edge {
            to: u,
            rev: len_u,
            cap: if bidirection { cap } else { 0 },
        });
    }

    fn push(&mut self, v: usize) {
        if self.height[v] > self.n as i32 {
            return;
        }

        self.act[self.height[v] as usize].push(v as i32);
        self.highest = self.highest.max(self.height[v]);
    }

    fn relabel(&mut self, sink: usize) {
        self.work = 0;

        for i in 0..self.act.len() {
            self.act[i].clear();
        }

        self.height.fill(self.n as i32 + 1);
        self.height[sink] = 0;

        let mut queue = vec![0; self.n];
        let mut start = -1;
        let mut end = -1;

        end += 1;
        queue[end as usize] = sink;

        while start < end {
            start += 1;

            let idx = queue[start as usize];
            let height = self.height[idx] + 1;

            for i in 0..self.graph[idx].len() {
                let edge = &self.graph[idx][i];

                if self.graph[edge.to][edge.rev].cap > 0 && height < self.height[edge.to] {
                    end += 1;
                    queue[end as usize] = edge.to;
                    self.height[edge.to] = height;

                    if self.excess[edge.to] > 0 {
                        self.push(edge.to);
                    }
                }
            }
        }
    }

    fn discharge(&mut self, v: usize) {
        let mut height_new = 2 * self.n as i32;

        for _ in 0..self.graph[v].len() {
            let edge = self.graph[v][self.arc[v] as usize];

            if edge.cap > 0 {
                if self.height[v] != self.height[edge.to] + 1 {
                    height_new = height_new.min(self.height[edge.to] + 1);
                } else {
                    let flow_delta = edge.cap.min(self.excess[v]);

                    self.graph[v][self.arc[v] as usize].cap -= flow_delta;
                    self.excess[v] -= flow_delta;

                    if self.excess[edge.to] == 0 {
                        self.push(edge.to);
                    }

                    self.excess[edge.to] += flow_delta;
                    self.graph[edge.to][edge.rev].cap += flow_delta;

                    if self.excess[v] == 0 {
                        return;
                    }
                }
            }

            self.arc[v] -= 1;

            if self.arc[v] < 0 {
                self.arc[v] = self.graph[v].len() as i32 - 1;
            }
        }

        self.work += 1;
        self.height[v] = height_new as i32;

        if self.height[v] < self.n as i32 && self.excess[v] > 0 {
            self.push(v);
        }
    }

    fn max_flow(&mut self, source: usize, sink: usize) -> Capacity {
        self.relabel(sink);

        self.excess[source] = Capacity::MAX;
        self.excess[sink] = -Capacity::MAX;

        self.push(source);

        while self.highest > 0 {
            while !self.act[self.highest as usize].is_empty() {
                let idx = self.act[self.highest as usize].pop().unwrap();

                if self.height[idx as usize] == self.highest {
                    self.discharge(idx as usize);
                }

                if self.work >= 4 * self.n {
                    self.relabel(sink);
                }
            }

            self.highest -= 1;
        }

        self.excess[sink] + Capacity::MAX
    }
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let (n, m) = (scan.token::<usize>(), scan.token::<usize>());
    let mut push_relabel = PushRelabelWithHLPP::new(n);

    for _ in 0..n {
        let (_, _) = (scan.token::<i32>(), scan.token::<i32>());
    }

    for _ in 0..m {
        let (u, v, w) = (
            scan.token::<usize>() - 1,
            scan.token::<usize>() - 1,
            scan.token::<i64>(),
        );

        push_relabel.add_edge(u, v, w, true);
    }

    let flow = push_relabel.max_flow(0, n - 1);

    writeln!(out, "{flow}").unwrap();
}
