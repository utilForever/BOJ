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

fn process_dfs(paper: &mut Vec<Vec<usize>>, n: usize, m: usize) {
    let mut queue = VecDeque::new();
    let mut visited = vec![vec![false; m]; n];

    let dx: [i64; 4] = [1, 0, -1, 0];
    let dy: [i64; 4] = [0, 1, 0, -1];

    queue.push_back((0, 0));
    visited[0][0] = true;

    while !queue.is_empty() {
        let (x, y) = queue.pop_front().unwrap();

        for i in 0..4 {
            let next_x = x + dx[i];
            let next_y = y + dy[i];

            if next_x < 0 || next_x >= n as i64 || next_y < 0 || next_y >= m as i64 {
                continue;
            }

            let next_x = next_x as usize;
            let next_y = next_y as usize;

            if visited[next_x][next_y] {
                continue;
            }

            if paper[next_x][next_y] >= 1 {
                paper[next_x][next_y] += 1;
                continue;
            }

            queue.push_back((next_x as i64, next_y as i64));
            visited[next_x][next_y] = true;
        }
    }
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let (n, m) = (scan.token::<usize>(), scan.token::<usize>());
    let mut paper = vec![vec![0; m]; n];

    for i in 0..n {
        for j in 0..m {
            paper[i][j] = scan.token::<usize>();
        }
    }

    let mut ret = 0;

    loop {
        process_dfs(&mut paper, n, m);

        let mut is_melted = false;

        for i in 0..n {
            for j in 0..m {
                if paper[i][j] >= 3 {
                    paper[i][j] = 0;
                    is_melted = true;
                } else if paper[i][j] == 2 {
                    paper[i][j] = 1;
                }
            }
        }

        if is_melted {
            ret += 1;
        } else {
            break;
        }
    }

    writeln!(out, "{}", ret).unwrap();
}
