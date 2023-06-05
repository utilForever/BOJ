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

    let (n, m) = (scan.token::<usize>(), scan.token::<usize>());
    let mut map = vec![vec![0; m]; n];
    let mut dest = (0, 0);

    for i in 0..n {
        for j in 0..m {
            map[i][j] = scan.token::<i64>();

            if map[i][j] == 2 {
                dest = (i as i64, j as i64);
            }
        }
    }

    let dx: [i64; 4] = [1, 0, -1, 0];
    let dy: [i64; 4] = [0, 1, 0, -1];

    let mut visited = vec![vec![false; m]; n];
    let mut ret = vec![vec![0; m]; n];
    let mut queue = VecDeque::new();

    visited[dest.0 as usize][dest.1 as usize] = true;
    queue.push_back((dest.0, dest.1));

    while !queue.is_empty() {
        let (x, y) = queue.pop_front().unwrap();

        for i in 0..4 {
            let next_x = x + dx[i];
            let next_y = y + dy[i];

            if next_x < 0 || next_x >= n as i64 || next_y < 0 || next_y >= m as i64 {
                continue;
            }

            let next_x = next_x as usize;
            let next_y = next_y as usize;

            if visited[next_x][next_y] {
                continue;
            }

            if map[next_x][next_y] == 0 {
                continue;
            }

            visited[next_x][next_y] = true;
            ret[next_x][next_y] = ret[x as usize][y as usize] + 1;

            queue.push_back((next_x as i64, next_y as i64));
        }
    }

    for i in 0..n {
        for j in 0..m {
            write!(
                out,
                "{} ",
                if ret[i][j] == 0 && map[i][j] == 1 {
                    -1
                } else {
                    ret[i][j]
                }
            )
            .unwrap();
        }

        writeln!(out).unwrap();
    }
}
