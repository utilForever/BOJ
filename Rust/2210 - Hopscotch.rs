use io::Write;
use std::{collections::HashSet, io, str};

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

fn process_dfs(
    board: &[[char; 5]; 5],
    ret: &mut HashSet<String>,
    i: usize,
    j: usize,
    mut s: String,
) {
    if s.len() == 6 {
        ret.insert(s);
        return;
    }

    let dy = [-1, 1, 0, 0];
    let dx = [0, 0, -1, 1];

    for k in 0..4 {
        let (y_next, x_next) = (i as i32 + dy[k], j as i32 + dx[k]);

        if y_next < 0 || y_next >= 5 || x_next < 0 || x_next >= 5 {
            continue;
        }

        let (y_next, x_next) = (y_next as usize, x_next as usize);

        s.push(board[i][j]);
        process_dfs(board, ret, y_next, x_next, s.clone());
        s.pop();
    }
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let mut board = [[' '; 5]; 5];

    for i in 0..5 {
        for j in 0..5 {
            board[i][j] = scan.token::<char>();
        }
    }

    let mut ret = HashSet::new();

    for i in 0..5 {
        for j in 0..5 {
            process_dfs(&board, &mut ret, i, j, String::new());
        }
    }

    writeln!(out, "{}", ret.len()).unwrap();
}
