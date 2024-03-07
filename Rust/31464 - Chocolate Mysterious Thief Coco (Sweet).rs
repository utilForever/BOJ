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

static DY: [i64; 4] = [0, 1, 0, -1];
static DX: [i64; 4] = [1, 0, -1, 0];

fn process_dfs(
    chocolate: &Vec<Vec<char>>,
    visited: &mut Vec<Vec<bool>>,
    y_curr: usize,
    x_curr: usize,
    y_prev: i64,
    x_prev: i64,
) -> bool {
    visited[y_curr][x_curr] = true;

    for i in 0..4 {
        let y_next = y_curr as i64 + DY[i];
        let x_next = x_curr as i64 + DX[i];

        if y_next < 0
            || y_next >= chocolate.len() as i64
            || x_next < 0
            || x_next >= chocolate.len() as i64
        {
            continue;
        }

        if y_next == y_prev && x_next == x_prev {
            continue;
        }

        let y_next = y_next as usize;
        let x_next = x_next as usize;

        if chocolate[y_next][x_next] == '.' {
            continue;
        }

        if visited[y_next][x_next] {
            return false;
        }

        if !process_dfs(
            chocolate,
            visited,
            y_next,
            x_next,
            y_curr as i64,
            x_curr as i64,
        ) {
            return false;
        }
    }

    true
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let n = scan.token::<usize>();
    let mut chocolate = vec![vec![' '; n]; n];

    for i in 0..n {
        let s = scan.token::<String>();

        for (j, c) in s.chars().enumerate() {
            chocolate[i][j] = c;
        }
    }

    let mut ret = Vec::new();

    for i in 0..n {
        for j in 0..n {
            if chocolate[i][j] == '.' {
                continue;
            }

            let mut visited = vec![vec![false; n]; n];
            let mut is_dfs_processed = false;
            let mut is_satisfied = true;

            chocolate[i][j] = '.';

            for k in 0..4 {
                let y_next = i as i64 + DY[k];
                let x_next = j as i64 + DX[k];

                if y_next < 0 || y_next >= n as i64 || x_next < 0 || x_next >= n as i64 {
                    continue;
                }

                let y_next = y_next as usize;
                let x_next = x_next as usize;

                if chocolate[y_next][x_next] == '.' {
                    continue;
                }

                if is_dfs_processed {
                    if !visited[y_next][x_next] {
                        is_satisfied = false;
                        break;
                    }
                } else {
                    is_dfs_processed = true;

                    if !process_dfs(&chocolate, &mut visited, y_next, x_next, -1, -1) {
                        is_satisfied = false;
                        break;
                    }
                }
            }

            chocolate[i][j] = '#';

            if is_satisfied {
                ret.push((i + 1, j + 1));
            }
        }
    }

    writeln!(out, "{}", ret.len()).unwrap();

    for val in ret {
        writeln!(out, "{} {}", val.0, val.1).unwrap();
    }
}
