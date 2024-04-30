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

    let (n, m) = (scan.token::<usize>(), scan.token::<usize>());
    let mut grid = vec![vec![0; m]; n];

    for i in 0..n {
        for j in 0..m {
            grid[i][j] = scan.token::<i32>();
        }
    }

    let dy: [i32; 4] = [1, -1, 0, 0];
    let dx: [i32; 4] = [0, 0, 1, -1];
    let mut ret = 0;

    loop {
        if grid.iter().all(|row| row.iter().all(|&x| x == 0)) {
            ret = 0;
            break;
        }

        let mut queue: VecDeque<(usize, usize)> = VecDeque::new();
        let mut visited = vec![vec![false; m]; n];
        let mut cnt = 0;

        for i in 0..n {
            for j in 0..m {
                if grid[i][j] == 0 {
                    continue;
                }

                if visited[i][j] {
                    continue;
                }

                queue.push_back((i, j));
                cnt += 1;

                while !queue.is_empty() {
                    let (y, x) = queue.pop_front().unwrap();

                    for k in 0..4 {
                        let y_next = y as i32 + dy[k];
                        let x_next = x as i32 + dx[k];

                        if y_next < 0 || y_next >= n as i32 || x_next < 0 || x_next >= m as i32 {
                            continue;
                        }

                        let y_next = y_next as usize;
                        let x_next = x_next as usize;

                        if visited[y_next][x_next] {
                            continue;
                        }

                        if grid[y_next][x_next] == 0 {
                            continue;
                        }

                        visited[y_next][x_next] = true;
                        queue.push_back((y_next, x_next));
                    }
                }
            }
        }

        if cnt > 1 {
            break;
        }

        ret += 1;

        let mut grid_next = grid.clone();

        for i in 0..n {
            for j in 0..m {
                if grid[i][j] == 0 {
                    continue;
                }

                let mut cnt = 0;

                for k in 0..4 {
                    let y_next = i as i32 + dy[k];
                    let x_next = j as i32 + dx[k];

                    if y_next < 0 || y_next >= n as i32 || x_next < 0 || x_next >= m as i32 {
                        continue;
                    }

                    let y_next = y_next as usize;
                    let x_next = x_next as usize;

                    if grid[y_next][x_next] == 0 {
                        cnt += 1;
                    }
                }

                grid_next[i][j] = (grid[i][j] - cnt).max(0);
            }
        }

        grid = grid_next.clone();
    }

    writeln!(out, "{ret}").unwrap();
}
