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

fn process_dfs(
    graph: &Vec<Vec<usize>>,
    l: &Vec<i64>,
    r: &Vec<i64>,
    lr: &mut Vec<i64>,
    max: &mut Vec<i64>,
    curr: usize,
    prev: usize,
) {
    lr[curr] = l[curr] + r[curr];

    for &next in graph[curr].iter() {
        if next == prev {
            continue;
        }

        process_dfs(graph, l, r, lr, max, next, curr);

        lr[curr] += lr[next];
        max[curr] = max[curr].max(lr[next]);
    }
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let n = scan.token::<usize>();
    let mut graph = vec![Vec::new(); n + 1];

    for _ in 0..n - 1 {
        let (u, v) = (scan.token::<usize>(), scan.token::<usize>());
        graph[u].push(v);
        graph[v].push(u);
    }

    let mut l = vec![0; n + 1];
    let mut r = vec![0; n + 1];

    for i in 1..=n {
        l[i] = scan.token::<i64>();
        r[i] = scan.token::<i64>();
    }

    let sum_r = r.iter().sum::<i64>();
    let sum_lr = l.iter().zip(r.iter()).map(|(a, b)| a + b).sum::<i64>();

    let mut lr = vec![0; n + 1];
    let mut max = vec![0; n + 1];

    process_dfs(&graph, &l, &r, &mut lr, &mut max, 1, 0);

    for i in 1..=n {
        max[i] = max[i].max(sum_lr - lr[i]);

        if sum_r >= max[i] {
            write!(out, "{i} ").unwrap();
        }
    }

    writeln!(out).unwrap();
}
