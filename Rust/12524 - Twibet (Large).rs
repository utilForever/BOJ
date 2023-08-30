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

fn process_bfs(followers: &Vec<Vec<usize>>, n: usize, idx: usize) -> i64 {
    let mut queue = VecDeque::new();
    let mut visited = vec![false; n + 1];

    queue.push_back(idx);
    visited[idx] = true;

    while !queue.is_empty() {
        let curr = queue.pop_front().unwrap();

        for &follower in followers[curr].iter() {
            if visited[follower] {
                continue;
            }

            queue.push_back(follower);
            visited[follower] = true;
        }
    }

    visited.iter().filter(|&&x| x).count() as i64
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let t = scan.token::<i64>();

    for i in 1..=t {
        let n = scan.token::<usize>();
        let mut followers = vec![Vec::new(); n + 1];

        for j in 1..=n {
            let f = scan.token::<usize>();
            followers[f].push(j);
        }

        writeln!(out, "Case #{i}:").unwrap();

        for j in 1..=n {
            writeln!(out, "{}", process_bfs(&followers, n, j)).unwrap();
        }
    }
}
