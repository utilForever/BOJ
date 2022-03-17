use io::Write;
use std::{collections::VecDeque, io, str};

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

fn process_bfs(graph: &Vec<Vec<usize>>, num_vertices: usize, start: usize) -> usize {
    let mut queue = VecDeque::new();
    let mut checked = vec![false; num_vertices + 1];
    let mut count = 0;

    queue.push_back(start);
    checked[start] = true;

    while !queue.is_empty() {
        let cur_node = queue.pop_front().unwrap();

        for vertex in graph[cur_node].iter() {
            if !checked[*vertex] {
                checked[*vertex] = true;
                queue.push_back(*vertex);
                count += 1;
            }
        }
    }

    count
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let (num_computers, num_edges) = (scan.token::<usize>(), scan.token::<usize>());
    let mut graph = vec![Vec::new(); num_computers + 1];

    for _ in 0..num_edges {
        let (a, b) = (scan.token::<usize>(), scan.token::<usize>());
        graph[a].push(b);
        graph[b].push(a);
    }

    for i in 1..=num_computers {
        graph[i].sort();
    }

    writeln!(out, "{}", process_bfs(&graph, num_computers, 1)).unwrap();
}
