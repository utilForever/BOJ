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

    let mut chessboard_start = [[0; 4]; 4];
    let mut chessboard_end = [[0; 4]; 4];

    for i in 0..4 {
        let s = scan.token::<String>();

        for (j, c) in s.chars().enumerate() {
            chessboard_start[i][j] = if c == '1' { 1 } else { 0 };
        }
    }

    for i in 0..4 {
        let s = scan.token::<String>();

        for (j, c) in s.chars().enumerate() {
            chessboard_end[i][j] = if c == '1' { 1 } else { 0 };
        }
    }

    if chessboard_start == chessboard_end {
        writeln!(out, "0").unwrap();
        return;
    }

    let dx = [-2, -1, -1, -2, 2, 1, 1, 2];
    let dy = [1, 2, -2, -1, 1, 2, -2, -1];
    let mut ret = None;

    let mut queue = VecDeque::new();
    let mut visited = [[[[false; 16]; 16]; 16]; 16];

    let calculate_idxes = |chessboard: &[[i8; 4]; 4]| -> (usize, usize, usize, usize) {
        let idx1 = chessboard[0][0] as usize * 8
            + chessboard[0][1] as usize * 4
            + chessboard[0][2] as usize * 2
            + chessboard[0][3] as usize;
        let idx2 = chessboard[1][0] as usize * 8
            + chessboard[1][1] as usize * 4
            + chessboard[1][2] as usize * 2
            + chessboard[1][3] as usize;
        let idx3 = chessboard[2][0] as usize * 8
            + chessboard[2][1] as usize * 4
            + chessboard[2][2] as usize * 2
            + chessboard[2][3] as usize;
        let idx4 = chessboard[3][0] as usize * 8
            + chessboard[3][1] as usize * 4
            + chessboard[3][2] as usize * 2
            + chessboard[3][3] as usize;

        (idx1, idx2, idx3, idx4)
    };

    queue.push_back((chessboard_start, 0));

    let idxes = calculate_idxes(&chessboard_start);
    visited[idxes.0][idxes.1][idxes.2][idxes.3] = true;

    while !queue.is_empty() && ret.is_none() {
        let (chessboard, cnt) = queue.pop_front().unwrap();

        if chessboard == chessboard_end {
            ret = Some(cnt);
            break;
        }

        for i in 0..4 {
            for j in 0..4 {
                if chessboard[i][j] == 0 {
                    continue;
                }

                for k in 0..8 {
                    let mut chessboard_next = chessboard.clone();
                    let x_next = i as i8 + dx[k];
                    let y_next = j as i8 + dy[k];

                    if x_next < 0 || x_next >= 4 || y_next < 0 || y_next >= 4 {
                        continue;
                    }

                    let x_next = x_next as usize;
                    let y_next = y_next as usize;

                    if chessboard_next[x_next][y_next] == 1 {
                        continue;
                    }

                    chessboard_next[i][j] = 0;
                    chessboard_next[x_next][y_next] = 1;

                    let idxes = calculate_idxes(&chessboard_next);

                    if visited[idxes.0][idxes.1][idxes.2][idxes.3] {
                        continue;
                    }

                    visited[idxes.0][idxes.1][idxes.2][idxes.3] = true;
                    queue.push_back((chessboard_next, cnt + 1));
                }
            }
        }

        if ret.is_some() {
            break;
        }
    }

    writeln!(out, "{}", ret.unwrap()).unwrap();
}
