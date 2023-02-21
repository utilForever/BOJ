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

fn process_bfs(spaces: &Vec<Vec<i64>>, n: usize, m: usize) -> i64 {
    let dx = [-1, 1, 0, 0, -1, -1, 1, 1];
    let dy = [0, 0, -1, 1, -1, 1, -1, 1];

    let mut queue = VecDeque::new();
    let mut dist = vec![vec![0; m]; n];

    for i in 0..n {
        for j in 0..m {
            if spaces[i][j] == 1 {
                queue.push_back((i as i64, j as i64));
                dist[i][j] = 1;
            }
        }
    }

    while !queue.is_empty() {
        let (cur_x, cur_y) = queue.pop_front().unwrap();

        for k in 0..8 {
            let (next_x, next_y) = (cur_x as i64 + dx[k], cur_y as i64 + dy[k]);

            if next_x < 0 || next_x >= n as i64 || next_y < 0 || next_y >= m as i64 {
                continue;
            }

            let next_x = next_x as usize;
            let next_y = next_y as usize;

            if dist[next_x][next_y] > 0 {
                continue;
            }

            queue.push_back((next_x as i64, next_y as i64));
            dist[next_x][next_y] = dist[cur_x as usize][cur_y as usize] + 1;
        }
    }
    
    dist.iter().flatten().max().unwrap().clone() - 1
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let (n, m) = (scan.token::<usize>(), scan.token::<usize>());
    let mut spaces = vec![vec![0; m]; n];

    for i in 0..n {
        for j in 0..m {
            spaces[i][j] = scan.token::<i64>();
        }
    }

    writeln!(out, "{}", process_bfs(&spaces, n, m)).unwrap();
}
