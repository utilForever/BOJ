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
    let mut castle = vec![vec![0; m]; n];

    for i in 0..n {
        for j in 0..m {
            castle[i][j] = scan.token::<i64>();
        }
    }

    let mut idx_row = vec![vec![-1; m]; n];
    let mut cnt_row = 0;

    for i in 0..n {
        let mut curr = -1;

        for j in 0..m {
            if castle[i][j] == 2 {
                curr = -1;
                continue;
            }

            if curr == -1 {
                curr = cnt_row;
                cnt_row += 1;
            }

            idx_row[i][j] = curr;
        }
    }

    let mut idx_col = vec![vec![-1; m]; n];
    let mut cnt_col = 0;

    for j in 0..m {
        let mut curr = -1;

        for i in 0..n {
            if castle[i][j] == 2 {
                curr = -1;
                continue;
            }

            if curr == -1 {
                curr = cnt_col;
                cnt_col += 1;
            }

            idx_col[i][j] = curr;
        }
    }

    let mut graph = vec![Vec::new(); cnt_row as usize];

    for i in 0..n {
        for j in 0..m {
            if castle[i][j] == 0 {
                let r = idx_row[i][j];
                let c = idx_col[i][j];

                if r >= 0 && c >= 0 {
                    graph[r as usize].push(c as usize);
                }
            }
        }
    }

    let mut check = vec![false; cnt_col as usize];
    let mut matched = vec![-1; cnt_col as usize];
    let mut ret = 0;

    for i in 0..cnt_row as usize {
        check.fill(false);

        if process_dfs(&graph, &mut check, &mut matched, i) {
            ret += 1;
        }
    }

    writeln!(out, "{ret}").unwrap();
}
