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

fn process_dfs(graph: &Vec<Vec<usize>>, visited: &mut Vec<bool>, cnt: &mut usize, curr: usize) {
    visited[curr] = true;
    *cnt += 1;

    for &next in graph[curr].iter() {
        if visited[next] {
            continue;
        }

        process_dfs(graph, visited, cnt, next);
    }
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let n = scan.token::<usize>();
    let mut board = vec![vec![0; n * n]; n * n];

    for i in 0..n * n {
        for j in 0..n * n {
            board[i][j] = scan.token::<i64>();
        }
    }

    let row1 = n * n - 2;
    let row2 = n * n - 1;
    let mut graph = vec![Vec::new(); n * n];

    for col1 in 0..n * n {
        for col2 in col1 + 1..n * n {
            let check =
                board[row1][col1] == board[row2][col2] || board[row1][col2] == board[row2][col1];

            if check {
                graph[col1].push(col2);
                graph[col2].push(col1);
            }
        }
    }

    let mut visited = vec![false; n * n];
    let mut cnt = 0;
    let mut idx = 0;

    while idx < n * n {
        if !visited[idx] {
            process_dfs(&graph, &mut visited, &mut cnt, idx);
        }

        if cnt == n * n {
            break;
        }

        idx += 1;
    }

    writeln!(out, "{}", n.pow(2) * (n.pow(2) - 2) + idx + 1).unwrap();
}
