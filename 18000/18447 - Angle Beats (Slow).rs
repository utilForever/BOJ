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
// Reference: https://judge.yosupo.jp/submission/71040
mod maximum_matching {
    use std::mem::swap;

    pub struct Matching {
        n: usize,
        adjacency_matrix: Vec<Vec<bool>>,
        matched_partner: Vec<usize>,
        predecessor: Vec<usize>,
        label: Vec<usize>,
        parent: Vec<usize>,
        queue: Vec<usize>,
        queue_head: usize,
        queue_tail: usize,
        scanned_timestamp: Vec<usize>,
        current_timestamp: usize,
    }

    impl Matching {
        pub fn new(n: usize) -> Self {
            let len = n + 1;

            Self {
                n,
                adjacency_matrix: vec![vec![false; len]; len],
                matched_partner: vec![0; len],
                predecessor: vec![0; len],
                label: vec![0; len],
                parent: vec![0; len],
                queue: vec![0; len],
                queue_head: 0,
                queue_tail: 0,
                scanned_timestamp: vec![0; len],
                current_timestamp: 0,
            }
        }

        fn find(&mut self, x: usize) -> usize {
            if self.parent[x] != x {
                self.parent[x] = self.find(self.parent[x]);
            }

            self.parent[x]
        }

        fn lca(&mut self, mut x: usize, mut y: usize) -> usize {
            self.current_timestamp += 1;

            while self.scanned_timestamp[x] != self.current_timestamp {
                if x != 0 {
                    self.scanned_timestamp[x] = self.current_timestamp;
                    x = self.find(self.predecessor[self.matched_partner[x]]);
                }

                swap(&mut x, &mut y);
            }

            x
        }

        fn blossom(&mut self, mut x: usize, mut y: usize, root: usize) {
            while self.find(x) != root {
                self.predecessor[x] = y;
                y = self.matched_partner[x];
                self.parent[y] = root;
                self.parent[x] = root;

                if self.label[y] == 1 {
                    self.queue[self.queue_tail] = y;
                    self.label[y] = 2;
                    self.queue_tail += 1;
                }

                x = self.predecessor[y];
            }
        }

        pub fn try_match(&mut self, mut x: usize) -> bool {
            self.queue_head = 0;
            self.queue_tail = 0;

            for i in 1..=self.n {
                self.parent[i] = i;
                self.label[i] = 0;
            }

            self.queue[self.queue_tail] = x;
            self.label[x] = 2;
            self.queue_tail += 1;

            while self.queue_head < self.queue_tail {
                x = self.queue[self.queue_head];
                self.queue_head += 1;

                for mut u in 1..=self.n {
                    if !self.adjacency_matrix[x][u] {
                        continue;
                    }

                    if self.label[u] == 0 {
                        self.label[u] = 1;
                        self.predecessor[u] = x;

                        if self.matched_partner[u] == 0 {
                            while x != 0 {
                                x = self.matched_partner[self.predecessor[u]];

                                self.matched_partner[u] = self.predecessor[u];
                                self.matched_partner[self.predecessor[u]] = u;

                                u = x;
                            }

                            return true;
                        } else {
                            self.queue[self.queue_tail] = self.matched_partner[u];
                            self.label[self.matched_partner[u]] = 2;
                            self.queue_tail += 1;
                        }
                    } else if self.label[u] == 2 && self.find(u) != self.find(x) {
                        let root = self.lca(x, u);

                        self.blossom(x, u, root);
                        self.blossom(u, x, root);
                    }
                }
            }

            false
        }

        pub fn mate(&self, i: usize) -> Option<usize> {
            let i = i + 1;

            if self.matched_partner[i] == 0 {
                None
            } else {
                Some(self.matched_partner[i] - 1)
            }
        }

        pub fn add_edge(&mut self, x: usize, y: usize) {
            let x = x + 1;
            let y = y + 1;

            self.adjacency_matrix[x][y] = true;
            self.adjacency_matrix[y][x] = true;
        }

        pub fn max_match(&mut self, greedy: bool, perm: &[usize]) -> usize {
            let mut total = 0;

            if greedy {
                for i in 1..=self.n {
                    for j in i + 1..=self.n {
                        if self.adjacency_matrix[i][j]
                            && self.matched_partner[i] == 0
                            && self.matched_partner[j] == 0
                        {
                            self.matched_partner[i] = j;
                            self.matched_partner[j] = i;
                            total += 1;
                        }
                    }
                }
            }

            for i in perm {
                let i = i + 1;

                if self.matched_partner[i] == 0 && self.try_match(i) {
                    total += 1;
                }
            }

            total
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
 
    let perm = (0..n * m * 2).into_iter().collect::<Vec<_>>();
    matching.max_match(true, &perm);
 
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
 
            if matching.mate(idx_curr).is_none() || matching.mate(idx_extra).is_none() {
                idx_extra += 1;
                continue;
            }
 
            let matching_curr = matching.mate(idx_curr).unwrap();
            let matching_extra = matching.mate(idx_extra).unwrap();
 
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
