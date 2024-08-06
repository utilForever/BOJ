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

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let (n, m) = (scan.token::<usize>(), scan.token::<usize>());
    let mut grid = vec![vec![0; m]; n];

    for i in 0..n {
        for j in 0..m {
            grid[i][j] = scan.token::<i64>();
        }
    }

    let dy: [i64; 4] = [0, 1, 0, -1];
    let dx: [i64; 4] = [1, 0, -1, 0];
    let mut ret = Vec::new();

    for i in 0..n {
        let mut is_found = false;
        let mut visited = vec![vec![vec![false; 4]; m]; n];
        let mut direction = 0;
        let (mut curr_y, mut curr_x) = (i, 0);

        visited[curr_y][curr_x][direction] = true;

        loop {
            let (next_y, next_x) = (
                curr_y as i64 + dy[direction] * grid[curr_y][curr_x],
                curr_x as i64 + dx[direction] * grid[curr_y][curr_x],
            );

            if next_y < 0 || next_y >= n as i64 || next_x < 0 || next_x >= m as i64 {
                break;
            }

            let (next_y, next_x) = (next_y as usize, next_x as usize);

            if visited[next_y][next_x][direction] {
                is_found = true;
                break;
            }

            visited[next_y][next_x][direction] = true;
            direction = (direction + 1) % 4;
            curr_y = next_y;
            curr_x = next_x;
        }

        if is_found {
            ret.push(i + 1);
        }
    }

    if ret.is_empty() {
        writeln!(out, "0").unwrap();
        return;
    }

    writeln!(out, "{}", ret.len()).unwrap();

    for val in ret {
        write!(out, "{val} ").unwrap();
    }

    writeln!(out).unwrap();
}
