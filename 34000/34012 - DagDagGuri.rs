use io::Write;
use std::{
    collections::{HashMap, VecDeque},
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

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let (n, m) = (scan.token::<usize>(), scan.token::<i64>());
    let mut graph = vec![Vec::new(); n + 1];
    let mut cnt_successors = (0..=n).map(|_| HashMap::new()).collect::<Vec<_>>();
    let mut degree_in = vec![0; n + 1];
    let mut degree_out = vec![0; n + 1];

    for _ in 0..m {
        let (u, v) = (scan.token::<usize>(), scan.token::<usize>());

        graph[u].push(v);
        degree_in[v] += 1;
        degree_out[u] += 1;

        let edge = cnt_successors[u].entry(v).or_insert(0);

        if *edge < 2 {
            *edge += 1;
        }
    }

    let mut queue = VecDeque::new();

    for i in 1..=n {
        if degree_in[i] == 0 {
            queue.push_back(i);
        }
    }

    let mut topological = Vec::with_capacity(n);
    let sink = (1..=n).find(|&v| degree_out[v] == 0).unwrap();

    while let Some(node) = queue.pop_front() {
        topological.push(node);

        for &next in graph[node].iter() {
            degree_in[next] -= 1;

            if degree_in[next] == 0 {
                queue.push_back(next);
            }
        }
    }

    let mut paths = vec![0; n + 1];
    let mut ret = 0;

    paths[sink] = 2;

    for &node in topological.iter().rev() {
        if node == sink {
            continue;
        }

        let mut cnt = 0;

        for (&next, &c) in cnt_successors[node].iter() {
            cnt += c.min(paths[next]);

            if cnt >= 2 {
                break;
            }
        }

        if cnt < 2 {
            ret += 2 - cnt;
        }

        paths[node] = 2;
    }

    writeln!(out, "{ret}").unwrap();
}
