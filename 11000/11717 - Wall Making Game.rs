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

    pub fn line(&mut self) -> String {
        let mut input = String::new();
        self.reader.read_line(&mut input).expect("Failed read");
        input
    }
}

fn sprague_grundy(
    board: &Vec<Vec<char>>,
    dp: &mut Vec<Vec<Vec<Vec<i64>>>>,
    r1: i64,
    r2: i64,
    c1: i64,
    c2: i64,
) -> i64 {
    if r1 > r2 || c1 > c2 {
        return 0;
    }

    let r1 = r1 as usize;
    let r2 = r2 as usize;
    let c1 = c1 as usize;
    let c2 = c2 as usize;

    if dp[r1][r2][c1][c2] != -1 {
        return dp[r1][r2][c1][c2];
    }

    let mut moves = HashSet::new();

    for i in r1..=r2 {
        for j in c1..=c2 {
            if board[i][j] == '.' {
                let top_left =
                    sprague_grundy(board, dp, r1 as i64, i as i64 - 1, c1 as i64, j as i64 - 1);
                let top_right =
                    sprague_grundy(board, dp, r1 as i64, i as i64 - 1, j as i64 + 1, c2 as i64);
                let bottom_left =
                    sprague_grundy(board, dp, i as i64 + 1, r2 as i64, c1 as i64, j as i64 - 1);
                let bottom_right =
                    sprague_grundy(board, dp, i as i64 + 1, r2 as i64, j as i64 + 1, c2 as i64);

                let nim = top_left ^ top_right ^ bottom_left ^ bottom_right;
                moves.insert(nim);
            }
        }
    }

    let mut grundy = 0;

    while moves.contains(&grundy) {
        grundy += 1;
    }

    dp[r1][r2][c1][c2] = grundy;

    grundy
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let (h, w) = (scan.token::<usize>(), scan.token::<usize>());
    let mut board = vec![vec![' '; w]; h];

    for i in 0..h {
        let line = scan.token::<String>();

        for (j, c) in line.chars().enumerate() {
            board[i][j] = c;
        }
    }

    let mut dp = vec![vec![vec![vec![-1; w]; w]; h]; h];
    let ret = sprague_grundy(&board, &mut dp, 0, h as i64 - 1, 0, w as i64 - 1);

    writeln!(out, "{}", if ret != 0 { "First" } else { "Second" }).unwrap();
}
