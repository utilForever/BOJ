use io::Write;
use std::{io, str};

pub struct UnsafeScanner<R> {
    reader: R,
    buf_str: Vec<u8>,
    buf_iter: str::SplitAsciiWhitespace<'static>,
}

impl<R: io::BufRead> UnsafeScanner<R> {
    pub fn new(reader: R) -> Self {
        Self {
            reader,
            buf_str: Vec::new(),
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

// Maximum Maching in General Graph
// Reference: https://www.acmicpc.net/source/30487081 (Jiangly's code)
mod maximum_matching {
    use std::collections::VecDeque;

    pub struct Matching {
        n: usize,
        edges: Vec<Vec<usize>>,
        matching: Vec<i64>,
        visited: Vec<i64>,
        link: Vec<usize>,
        parent: Vec<usize>,
        depth: Vec<usize>,
        queue: VecDeque<usize>,
    }

    impl Matching {
        pub fn new(n: usize) -> Self {
            Self {
                n,
                edges: vec![Vec::new(); n],
                matching: vec![-1; n],
                visited: vec![0; n],
                link: vec![0; n],
                parent: vec![0; n],
                depth: vec![0; n],
                queue: VecDeque::new(),
            }
        }

        pub fn add_edge(&mut self, u: usize, v: usize) {
            self.edges[u].push(v);
            self.edges[v].push(u);
        }

        pub fn find_matching(&mut self) -> Vec<i64> {
            self.greedy();

            for u in 0..self.n {
                if self.matching[u] == -1 {
                    self.agument(u);
                }
            }

            self.matching.clone()
        }

        fn find(&mut self, mut u: usize) -> usize {
            while self.parent[u] != u {
                self.parent[u] = self.parent[self.parent[u]];
                u = self.parent[u];
            }

            u
        }

        fn lca(&mut self, mut u: usize, mut v: usize) -> usize {
            u = self.find(u);
            v = self.find(v);

            while u != v {
                if self.depth[u] < self.depth[v] {
                    std::mem::swap(&mut u, &mut v);
                }

                u = self.find(self.link[self.matching[u] as usize]);
            }

            u
        }

        fn blossom(&mut self, mut u: usize, mut v: usize, p: usize) {
            while self.find(u) != p {
                self.link[u] = v;
                v = self.matching[u] as usize;

                if self.visited[v] == 0 {
                    self.visited[v] = 1;
                    self.queue.push_back(v);
                }

                self.parent[v] = p;
                self.parent[u] = p;
                u = self.link[v];
            }
        }

        fn agument(&mut self, u: usize) {
            self.queue.clear();
            self.parent = (0..self.n).collect();
            self.visited = vec![-1; self.n];

            self.queue.push_back(u);
            self.visited[u] = 1;
            self.depth[u] = 0;

            while !self.queue.is_empty() {
                let u = self.queue.pop_front().unwrap();

                for i in 0..self.edges[u].len() {
                    let v = self.edges[u][i];

                    if self.visited[v] == -1 {
                        self.visited[v] = 0;
                        self.link[v] = u;
                        self.depth[v] = self.depth[u] + 1;

                        if self.matching[v] == -1 {
                            let mut x = v as i64;
                            let mut y = u as i64;

                            while y != -1 {
                                let temp = self.matching[y as usize] as usize;
                                self.matching[x as usize] = y as i64;
                                self.matching[y as usize] = x as i64;

                                x = temp as i64;
                                y = if x == -1 {
                                    -1
                                } else {
                                    self.link[x as usize] as i64
                                }
                            }

                            return;
                        }

                        self.visited[self.matching[v] as usize] = 1;
                        self.depth[self.matching[v] as usize] = self.depth[u] + 2;
                        self.queue.push_back(self.matching[v] as usize);
                    } else if self.visited[v] == 1 && self.find(v) != self.find(u) {
                        let p = self.lca(u, v);

                        self.blossom(u, v, p);
                        self.blossom(v, u, p);
                    }
                }
            }
        }

        fn greedy(&mut self) {
            for u in 0..self.n {
                if self.matching[u] != -1 {
                    continue;
                }

                for i in 0..self.edges[u].len() {
                    let v = self.edges[u][i];

                    if self.matching[v] == -1 {
                        self.matching[u] = v as i64;
                        self.matching[v] = u as i64;
                        break;
                    }
                }
            }
        }
    }
}

#[inline(always)]
fn convert_idx(m: usize, i: usize, j: usize) -> usize {
    i * m + j
}

#[inline(always)]
fn restore_idx(m: usize, idx: usize) -> (usize, usize) {
    (idx / m, idx % m)
}

// Reference: https://koosaga.com/258
fn main() {
    use maximum_matching::*;

    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let (n, m) = (scan.token::<usize>(), scan.token::<usize>());
    let mut board = vec![vec![' '; m]; n];

    for i in 0..n {
        let line = scan.token::<String>();

        for (j, c) in line.chars().enumerate() {
            board[i][j] = c;
        }
    }

    let dx = [0, 0, 1, -1];
    let dy = [1, -1, 0, 0];

    let mut matching = Matching::new(n * m * 2);
    let mut idx_extra = n * m;

    for i in 0..n {
        for j in 0..m {
            if board[i][j] == '+' {
                let idx_curr = convert_idx(m, i, j);
                matching.add_edge(idx_curr, idx_extra);

                for k in 0..4 {
                    let (y_next, x_next) = (i as i32 + dy[k], j as i32 + dx[k]);

                    if y_next < 0 || y_next >= n as i32 || x_next < 0 || x_next >= m as i32 {
                        continue;
                    }

                    let (y_next, x_next) = (y_next as usize, x_next as usize);

                    if board[y_next][x_next] == '.' {
                        let idx_next = convert_idx(m, y_next, x_next);

                        matching.add_edge(idx_curr, idx_next);
                        matching.add_edge(idx_extra, idx_next);
                    }
                }

                idx_extra += 1;
            } else if board[i][j] == '*' {
                let idx_curr = convert_idx(m, i, j);
                matching.add_edge(idx_curr, idx_extra);

                for k in 0..4 {
                    let (y_next, x_next) = (i as i32 + dy[k], j as i32 + dx[k]);

                    if y_next < 0 || y_next >= n as i32 || x_next < 0 || x_next >= m as i32 {
                        continue;
                    }

                    let (y_next, x_next) = (y_next as usize, x_next as usize);

                    if board[y_next][x_next] == '.' {
                        let idx_next = convert_idx(m, y_next, x_next);

                        if k == 0 || k == 1 {
                            matching.add_edge(idx_extra, idx_next);
                        } else {
                            matching.add_edge(idx_curr, idx_next);
                        }
                    }
                }

                idx_extra += 1;
            }
        }
    }

    let ret_matching = matching.find_matching();

    idx_extra = n * m;

    let mut check_letter = vec![0; 100];
    let mut identifier = 1;
    let mut ret = vec![vec![0; m]; n];

    for i in 0..n {
        for j in 0..m {
            if board[i][j] == '.' {
                continue;
            }

            let idx_curr = convert_idx(m, i, j);

            // writeln!(out, "x = {idx_curr} y = {idx_extra}").unwrap();
            // writeln!(
            //     out,
            //     "matching[x] = {} matching[y] = {}",
            //     matching[idx_curr], matching[idx_extra]
            // )
            // .unwrap();

            if ret_matching[idx_curr] == -1 || ret_matching[idx_extra] == -1 {
                idx_extra += 1;
                continue;
            }

            let matching_curr = ret_matching[idx_curr] as usize;
            let matching_extra = ret_matching[idx_extra] as usize;

            if matching_curr == idx_extra {
                idx_extra += 1;
                continue;
            }

            let idx_matching1 = restore_idx(m, matching_curr);
            let idx_matching2 = restore_idx(m, matching_extra);

            for k in 0..4 {
                let (y_next, x_next) = (i as i32 + dy[k], j as i32 + dx[k]);

                if y_next < 0 || y_next >= n as i32 || x_next < 0 || x_next >= m as i32 {
                    continue;
                }

                let (y_next, x_next) = (y_next as usize, x_next as usize);

                check_letter[ret[y_next][x_next]] = identifier;
            }

            for k in 0..4 {
                let (y_next, x_next) = (
                    idx_matching1.0 as i32 + dy[k],
                    idx_matching1.1 as i32 + dx[k],
                );

                if y_next < 0 || y_next >= n as i32 || x_next < 0 || x_next >= m as i32 {
                    continue;
                }

                let (y_next, x_next) = (y_next as usize, x_next as usize);

                check_letter[ret[y_next][x_next]] = identifier;
            }

            for k in 0..4 {
                let (y_next, x_next) = (
                    idx_matching2.0 as i32 + dy[k],
                    idx_matching2.1 as i32 + dx[k],
                );

                if y_next < 0 || y_next >= n as i32 || x_next < 0 || x_next >= m as i32 {
                    continue;
                }

                let (y_next, x_next) = (y_next as usize, x_next as usize);

                check_letter[ret[y_next][x_next]] = identifier;
            }

            let mut offset = 1;

            while check_letter[offset] == identifier {
                offset += 1;
            }

            ret[i][j] = offset;
            ret[idx_matching1.0][idx_matching1.1] = offset;
            ret[idx_matching2.0][idx_matching2.1] = offset;

            idx_extra += 1;
            identifier += 1;
        }
    }

    for i in 0..n {
        for j in 0..m {
            write!(
                out,
                "{}",
                if ret[i][j] > 0 {
                    (ret[i][j] as u8 + b'a' - 1) as char
                } else {
                    board[i][j]
                }
            )
            .unwrap();
        }

        writeln!(out).unwrap();
    }
}
