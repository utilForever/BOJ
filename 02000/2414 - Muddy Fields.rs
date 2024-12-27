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

    pub fn line(&mut self) -> String {
        let mut input = String::new();
        self.reader.read_line(&mut input).expect("Failed read");
        input
    }
}

fn process_dfs(
    graph: &Vec<Vec<usize>>,
    check: &mut Vec<bool>,
    matched: &mut Vec<i64>,
    idx: usize,
) -> bool {
    for &next in graph[idx].iter() {
        if check[next] {
            continue;
        }

        check[next] = true;

        if matched[next] == -1 || process_dfs(graph, check, matched, matched[next] as usize) {
            matched[next] = idx as i64;
            return true;
        }
    }

    false
}

fn main() {
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

    let mut idxes_horizontal = vec![vec![-1; m]; n];
    let mut cnt_horizontal = 0;

    for r in 0..n {
        let mut c = 0;

        while c < m {
            if board[r][c] == '*' {
                let idx_curr = cnt_horizontal;
                cnt_horizontal += 1;

                let mut c_new = c;

                while c_new < m && board[r][c_new] == '*' {
                    idxes_horizontal[r][c_new] = idx_curr as i64;
                    c_new += 1;
                }

                c = c_new;
            } else {
                c += 1;
            }
        }
    }

    let mut idxes_vertical = vec![vec![-1; m]; n];
    let mut cnt_vertical = 0;

    for c in 0..m {
        let mut r = 0;

        while r < n {
            if board[r][c] == '*' {
                let idx_curr = cnt_vertical;
                cnt_vertical += 1;

                let mut r_new = r;

                while r_new < n && board[r_new][c] == '*' {
                    idxes_vertical[r_new][c] = idx_curr as i64;
                    r_new += 1;
                }

                r = r_new;
            } else {
                r += 1;
            }
        }
    }

    let mut graph = vec![Vec::new(); cnt_horizontal];

    for r in 0..n {
        for c in 0..m {
            if board[r][c] == '*' {
                let h = idxes_horizontal[r][c] as usize;
                let v = idxes_vertical[r][c] as usize;

                graph[h].push(v);
            }
        }
    }

    let mut check = vec![false; cnt_vertical];
    let mut matched = vec![-1; cnt_vertical];
    let mut ret = 0;

    for u in 0..cnt_horizontal {
        check.fill(false);

        if process_dfs(&graph, &mut check, &mut matched, u) {
            ret += 1;
        }
    }

    writeln!(out, "{ret}").unwrap();
}
