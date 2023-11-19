use io::Write;
use std::{cmp::Reverse, collections::BinaryHeap, io, str};

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

fn process_dfs(
    graph: &Vec<Vec<usize>>,
    priority_queue: &mut BinaryHeap<Reverse<i64>>,
    parent: usize,
    curr: usize,
    depth_orig: i64,
    depth: i64,
) {
    let mut is_leaf = true;

    for &next in graph[curr].iter() {
        if parent == next {
            continue;
        }

        is_leaf = false;
        process_dfs(graph, priority_queue, curr, next, depth_orig, depth + 1);
    }

    if is_leaf {
        priority_queue.push(Reverse(depth_orig + depth + 1));
    }
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let n = scan.token::<i64>();
    let mut priority_queue: BinaryHeap<Reverse<i64>> = BinaryHeap::new();
    let mut graph = vec![Vec::new(); 100_001];

    priority_queue.push(Reverse(0));

    for _ in 0..n {
        let a = scan.token::<usize>();
        let depth = priority_queue.pop().unwrap().0;

        for _ in 0..a - 1 {
            let (v, w) = (scan.token::<usize>(), scan.token::<usize>());
            graph[v].push(w);
            graph[w].push(v);
        }

        process_dfs(&graph, &mut priority_queue, 0, 1, depth, 0);

        for i in 1..=a {
            graph[i].clear();
        }
    }

    let ret = priority_queue
        .into_vec()
        .into_iter()
        .map(|x| x.0)
        .max()
        .unwrap();
    writeln!(out, "{ret}").unwrap();
}
