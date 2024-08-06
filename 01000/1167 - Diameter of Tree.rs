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

fn process_bfs(tree: &Vec<Vec<(usize, usize)>>, dist: &mut Vec<usize>, start: usize) {
    let mut checked = vec![false; tree.len()];
    let mut queue = VecDeque::new();

    queue.push_back(start);
    checked[start] = true;

    while !queue.is_empty() {
        let start_vertex = queue.pop_front().unwrap();

        for i in 0..tree[start_vertex].len() {
            let (end_vertex, dist_to) = tree[start_vertex][i];

            if !checked[end_vertex] {
                dist[end_vertex] = dist[start_vertex] + dist_to;
                queue.push_back(end_vertex);
                checked[end_vertex] = true;
            }
        }
    }
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let v = scan.token::<usize>();
    let mut tree = vec![Vec::new(); v + 1];
    let mut dist = vec![0; v + 1];

    for _ in 0..v {
        let start_vertex = scan.token::<usize>();

        loop {
            let end_vertex = scan.token::<i32>();
            if end_vertex == -1 {
                break;
            }

            let dist = scan.token::<usize>();
            tree[start_vertex].push((end_vertex as usize, dist));
            tree[end_vertex as usize].push((start_vertex, dist));
        }
    }

    process_bfs(&tree, &mut dist, 1);

    let mut start = 1;

    for i in 2..=v {
        if dist[i] > dist[start] {
            start = i;
        }
    }

    dist.fill(0);
    process_bfs(&tree, &mut dist, start);

    let mut ans = dist[1];

    for i in 2..=v {
        if dist[i] > ans {
            ans = dist[i];
        }
    }

    writeln!(out, "{}", ans).unwrap();
}
