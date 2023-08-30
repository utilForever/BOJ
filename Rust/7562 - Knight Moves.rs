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

fn process_bfs(n: usize, start: (usize, usize), end: (usize, usize)) -> i64 {
    let dx = [-2, -1, 1, 2, 2, 1, -1, -2];
    let dy = [1, 2, 2, 1, -1, -2, -2, -1];

    let mut visited = vec![vec![false; n]; n];
    let mut dist = vec![vec![0; n]; n];

    let mut queue = VecDeque::new();
    queue.push_back((start.0 as i64, start.1 as i64));
    visited[start.0][start.1] = true;

    while !queue.is_empty() {
        let (cur_x, cur_y) = queue.pop_front().unwrap();

        if cur_x == end.0 as i64 && cur_y == end.1 as i64 {
            break;
        }

        for k in 0..8 {
            let (next_x, next_y) = (cur_x as i64 + dx[k], cur_y as i64 + dy[k]);

            if next_x < 0 || next_x >= n as i64 || next_y < 0 || next_y >= n as i64 {
                continue;
            }

            if visited[next_x as usize][next_y as usize] {
                continue;
            }

            let next_x = next_x as usize;
            let next_y = next_y as usize;

            queue.push_back((next_x as i64, next_y as i64));
            visited[next_x][next_y] = true;
            dist[next_x][next_y] = dist[cur_x as usize][cur_y as usize] + 1;
        }
    }

    dist[end.0][end.1]
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let t = scan.token::<i64>();

    for _ in 0..t {
        let n = scan.token::<usize>();
        let start = (scan.token::<usize>(), scan.token::<usize>());
        let end = (scan.token::<usize>(), scan.token::<usize>());

        writeln!(out, "{}", process_bfs(n, start, end)).unwrap();
    }
}
