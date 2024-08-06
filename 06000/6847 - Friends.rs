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

fn process_bfs(friends: &Vec<Vec<usize>>, start: usize, end: usize) -> (bool, i64) {
    let mut queue = VecDeque::new();
    let mut visited = vec![false; 10000];
    let mut dist = vec![0; 10000];

    queue.push_back(start);
    visited[start] = true;

    while !queue.is_empty() {
        let curr = queue.pop_front().unwrap();

        for &next in friends[curr].iter() {
            if visited[next] {
                continue;
            }

            queue.push_back(next);
            visited[next] = true;
            dist[next] = dist[curr] + 1;
        }
    }

    if visited[end] {
        (true, dist[end] - 1)
    } else {
        (false, 0)
    }
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let n = scan.token::<usize>();
    let mut friends = vec![Vec::new(); 10000];

    for _ in 0..n {
        let (x, y) = (scan.token::<usize>(), scan.token::<usize>());
        friends[x].push(y);
    }

    loop {
        let (x, y) = (scan.token::<usize>(), scan.token::<usize>());

        if x == 0 && y == 0 {
            break;
        }

        let ret = process_bfs(&friends, x, y);

        if ret.0 {
            writeln!(out, "Yes {}", ret.1).unwrap();
        } else {
            writeln!(out, "No").unwrap();
        }
    }
}
