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

#[derive(Clone, Default)]
struct Node {
    row: usize,
    size: usize,
    col: usize,
    up: usize,
    down: usize,
    left: usize,
    right: usize,
}

struct PentominoSolver {
    node: Vec<Node>,
    head: usize,
    solve: Vec<usize>,
}

impl PentominoSolver {
    fn new(n: usize) -> Self {
        Self {
            node: vec![Node::default(); n],
            head: n - 1,
            solve: Vec::new(),
        }
    }

    fn cover(&mut self, idx: usize) {
        let left = self.node[idx].left;
        let right = self.node[idx].right;

        self.node[right].left = self.node[idx].left;
        self.node[left].right = self.node[idx].right;

        let mut it = self.node[idx].down;

        while it != idx {
            let mut jt = self.node[it].right;

            while jt != it {
                let down = self.node[jt].down;
                let up = self.node[jt].up;
                let col = self.node[jt].col;

                self.node[down].up = self.node[jt].up;
                self.node[up].down = self.node[jt].down;
                self.node[col].size -= 1;
                jt = self.node[jt].right;
            }

            it = self.node[it].down;
        }
    }

    fn uncover(&mut self, idx: usize) {
        let mut it = self.node[idx].up;

        while it != idx {
            let mut jt = self.node[it].left;

            while jt != it {
                let col = self.node[jt].col;
                let down = self.node[jt].down;
                let up = self.node[jt].up;

                self.node[col].size += 1;
                self.node[down].up = jt;
                self.node[up].down = jt;
                jt = self.node[jt].left;
            }

            it = self.node[it].up;
        }

        let right = self.node[idx].right;
        let left = self.node[idx].left;

        self.node[right].left = idx;
        self.node[left].right = idx;
    }

    fn search(&mut self) -> bool {
        if self.node[self.head].right == self.head {
            return true;
        }

        let mut ptr = -1;
        let mut low = 1 << 30;
        let mut it = self.node[self.head].right;

        while it != self.head {
            if self.node[it].size < low {
                if self.node[it].size == 0 {
                    return false;
                }

                low = self.node[it].size;
                ptr = it as i64;
            }

            it = self.node[it].right;
        }

        self.cover(ptr as usize);

        let mut it = self.node[ptr as usize].down;

        while it as i64 != ptr {
            self.solve.push(self.node[it].row);
            let mut jt = self.node[it].right;

            while jt != it {
                self.cover(self.node[jt].col);
                jt = self.node[jt].right;
            }

            if self.search() {
                return true;
            }

            self.solve.pop();
            jt = self.node[it].left;

            while jt != it {
                self.uncover(self.node[jt].col);
                jt = self.node[jt].left;
            }

            it = self.node[it].down;
        }

        self.uncover(ptr as usize);

        false
    }

    fn solve(&mut self, board: &Vec<Vec<usize>>, size: usize) {
        self.node[self.head].left = size - 1;
        self.node[self.head].right = 0;

        for i in 0..size {
            self.node[i].up = i;
            self.node[i].down = i;
            self.node[i].size = 0;

            self.node[i].left = if i == 0 { self.head } else { i - 1 };
            self.node[i].right = if i == size - 1 { self.head } else { i + 1 };
        }

        let mut node_size = size;

        for i in 0..board.len() {
            let mut last = -1;

            for &j in board[i].iter() {
                self.node[node_size].row = i;
                self.node[node_size].col = j as usize;
                self.node[node_size].up = self.node[j as usize].up;
                self.node[node_size].down = j as usize;

                if last != -1 {
                    let right = self.node[last as usize].right;

                    self.node[node_size].left = last as usize;
                    self.node[node_size].right = self.node[last as usize].right;
                    self.node[right].left = node_size;
                    self.node[last as usize].right = node_size;
                } else {
                    self.node[node_size].left = node_size;
                    self.node[node_size].right = node_size;
                }

                let up = self.node[j as usize].up;

                self.node[up].down = node_size;
                self.node[j as usize].up = node_size;
                self.node[j as usize].size += 1;

                last = node_size as i64;
                node_size += 1;
            }
        }

        self.search();
    }
}

// Reference: http://www.secmem.org/blog/2019/12/15/knuths-algorithm-x/
fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let pentomino_kind = "ILYYLPUVPTNVPPPULNFWUZLULTFZPYWNVYNLINPWFZYFXFPNFYFTNYLNYZFTWVL".chars().collect::<Vec<_>>();
    let pentomino_data = [
        [[0, 0], [0, 1], [0, 2], [0, 3], [0, 4]],
        [[0, 0], [0, 1], [0, 2], [0, 3], [1, 0]],
        [[0, 0], [0, 1], [0, 2], [0, 3], [1, 1]],
        [[0, 0], [0, 1], [0, 2], [0, 3], [1, 2]],
        [[0, 0], [0, 1], [0, 2], [0, 3], [1, 3]],
        [[0, 0], [0, 1], [0, 2], [1, 0], [1, 1]],
        [[0, 0], [0, 1], [0, 2], [1, 0], [1, 2]],
        [[0, 0], [0, 1], [0, 2], [1, 0], [2, 0]],
        [[0, 0], [0, 1], [0, 2], [1, 1], [1, 2]],
        [[0, 0], [0, 1], [0, 2], [1, 1], [2, 1]],
        [[0, 0], [0, 1], [0, 2], [1, 2], [1, 3]],
        [[0, 0], [0, 1], [0, 2], [1, 2], [2, 2]],
        [[0, 0], [0, 1], [1, 0], [1, 1], [1, 2]],
        [[0, 0], [0, 1], [1, 0], [1, 1], [2, 0]],
        [[0, 0], [0, 1], [1, 0], [1, 1], [2, 1]],
        [[0, 0], [0, 1], [1, 0], [2, 0], [2, 1]],
        [[0, 0], [0, 1], [1, 0], [2, 0], [3, 0]],
        [[0, 0], [0, 1], [1, 1], [1, 2], [1, 3]],
        [[0, 0], [0, 1], [1, 1], [1, 2], [2, 1]],
        [[0, 0], [0, 1], [1, 1], [1, 2], [2, 2]],
        [[0, 0], [0, 1], [1, 1], [2, 0], [2, 1]],
        [[0, 0], [0, 1], [1, 1], [2, 1], [2, 2]],
        [[0, 0], [0, 1], [1, 1], [2, 1], [3, 1]],
        [[0, 0], [0, 2], [1, 0], [1, 1], [1, 2]],
        [[0, 0], [1, 0], [1, 1], [1, 2], [1, 3]],
        [[0, 0], [1, 0], [1, 1], [1, 2], [2, 0]],
        [[0, 0], [1, 0], [1, 1], [1, 2], [2, 1]],
        [[0, 0], [1, 0], [1, 1], [1, 2], [2, 2]],
        [[0, 0], [1, 0], [1, 1], [2, 0], [2, 1]],
        [[0, 0], [1, 0], [1, 1], [2, 0], [3, 0]],
        [[0, 0], [1, 0], [1, 1], [2, 1], [2, 2]],
        [[0, 0], [1, 0], [1, 1], [2, 1], [3, 1]],
        [[0, 0], [1, 0], [2, 0], [2, 1], [2, 2]],
        [[0, 0], [1, 0], [2, 0], [2, 1], [3, 0]],
        [[0, 0], [1, 0], [2, 0], [2, 1], [3, 1]],
        [[0, 0], [1, 0], [2, 0], [3, 0], [3, 1]],
        [[0, 0], [1, 0], [2, 0], [3, 0], [4, 0]],
        [[0, 1], [0, 2], [0, 3], [1, 0], [1, 1]],
        [[0, 1], [0, 2], [1, 0], [1, 1], [1, 2]],
        [[0, 1], [0, 2], [1, 0], [1, 1], [2, 0]],
        [[0, 1], [0, 2], [1, 0], [1, 1], [2, 1]],
        [[0, 1], [0, 2], [1, 1], [2, 0], [2, 1]],
        [[0, 1], [1, 0], [1, 1], [1, 2], [1, 3]],
        [[0, 1], [1, 0], [1, 1], [1, 2], [2, 0]],
        [[0, 1], [1, 0], [1, 1], [1, 2], [2, 1]],
        [[0, 1], [1, 0], [1, 1], [1, 2], [2, 2]],
        [[0, 1], [1, 0], [1, 1], [2, 0], [2, 1]],
        [[0, 1], [1, 0], [1, 1], [2, 0], [3, 0]],
        [[0, 1], [1, 0], [1, 1], [2, 1], [2, 2]],
        [[0, 1], [1, 0], [1, 1], [2, 1], [3, 1]],
        [[0, 1], [1, 1], [1, 2], [2, 0], [2, 1]],
        [[0, 1], [1, 1], [2, 0], [2, 1], [2, 2]],
        [[0, 1], [1, 1], [2, 0], [2, 1], [3, 0]],
        [[0, 1], [1, 1], [2, 0], [2, 1], [3, 1]],
        [[0, 1], [1, 1], [2, 1], [3, 0], [3, 1]],
        [[0, 2], [0, 3], [1, 0], [1, 1], [1, 2]],
        [[0, 2], [1, 0], [1, 1], [1, 2], [1, 3]],
        [[0, 2], [1, 0], [1, 1], [1, 2], [2, 0]],
        [[0, 2], [1, 0], [1, 1], [1, 2], [2, 1]],
        [[0, 2], [1, 0], [1, 1], [1, 2], [2, 2]],
        [[0, 2], [1, 1], [1, 2], [2, 0], [2, 1]],
        [[0, 2], [1, 2], [2, 0], [2, 1], [2, 2]],
        [[0, 3], [1, 0], [1, 1], [1, 2], [1, 3]],
    ];

    let (n, m) = (scan.token::<usize>(), scan.token::<usize>());
    let mut board = vec![vec![' '; m]; n];

    for i in 0..n {
        for j in 0..m {
            board[i][j] = scan.token::<char>();
        }
    }

    let mut block_encode = vec![None; 26];
    let mut block_decode = Vec::new();
    let mut id_encode = vec![vec![None; m]; n];
    let mut id_decode = Vec::new();

    for &kind in pentomino_kind.iter() {
        if block_encode[kind as usize - 'A' as usize].is_none() {
            block_encode[kind as usize - 'A' as usize] = Some(block_decode.len());
            block_decode.push(kind);
        }
    }

    let mut board_new = Vec::new();

    for i in 0..n {
        for j in 0..m {
            for k in 0..63 {
                let mut flag = true;

                for l in 0..5 {
                    let (qy, qx) = (i + pentomino_data[k][l][0], j + pentomino_data[k][l][1]);

                    if qy >= n || qx >= m || board[qy][qx] == '1' {
                        flag = false;
                        break;
                    }
                }

                if !flag {
                    continue;
                }

                let mut col = vec![block_encode[pentomino_kind[k] as usize - 'A' as usize].unwrap()];

                for l in 0..5 {
                    let (qy, qx) = (i + pentomino_data[k][l][0], j + pentomino_data[k][l][1]);

                    if id_encode[qy][qx].is_none() {
                        id_encode[qy][qx] = Some(id_decode.len());
                        id_decode.push((qy, qx));
                    }

                    col.push(12 + id_encode[qy][qx].unwrap());
                }

                board_new.push(col);
            }
        }
    }

    let mut solver = PentominoSolver::new(13500);
    solver.solve(&board_new, 72);

    for i in solver.solve.iter() {
        for j in 1..=5 {
            let (qy, qx) = id_decode[board_new[*i][j] as usize - 12];
            board[qy][qx] = block_decode[board_new[*i][0] as usize] as char;
        }
    }

    for i in 0..n {
        for j in 0..m {
            write!(out, "{} ", board[i][j]).unwrap();
        }

        writeln!(out).unwrap();
    }
}
