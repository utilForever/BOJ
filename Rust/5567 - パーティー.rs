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

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let (n, m) = (scan.token::<usize>(), scan.token::<i64>());
    let mut graph = vec![Vec::new(); n + 1];

    for _ in 0..m {
        let (a, b) = (scan.token::<usize>(), scan.token::<usize>());
        graph[a].push(b);
        graph[b].push(a);
    }

    let mut queue = VecDeque::new();
    let mut visited = vec![false; n + 1];
    let mut ret = 0;

    queue.push_back((1, 0));

    while !queue.is_empty() {
        let (node, dist) = queue.pop_front().unwrap();

        if visited[node] {
            continue;
        }

        if dist > 2 {
            continue;
        }

        visited[node] = true;
        ret += 1;

        for &neighbour in &graph[node] {
            if visited[neighbour] {
                continue;
            }

            queue.push_back((neighbour, dist + 1));
        }
    }

    writeln!(out, "{}", ret - 1).unwrap();
}
