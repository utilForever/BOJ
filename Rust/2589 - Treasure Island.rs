use io::Write;
use std::{collections::VecDeque, io, str, vec};

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
    let mut map = vec![vec![' '; m]; n];

    for i in 0..n {
        let line = scan.token::<String>();

        for (j, c) in line.chars().enumerate() {
            map[i][j] = c;
        }
    }

    let dy = [0, 0, 1, -1];
    let dx = [1, -1, 0, 0];
    let mut ret = 0;

    for i in 0..n {
        for j in 0..m {
            if map[i][j] == 'W' {
                continue;
            }

            let mut queue = VecDeque::new();
            let mut visited = vec![vec![false; m]; n];
            let mut dist_max = 0;

            queue.push_back((i, j, 0));

            while !queue.is_empty() {
                let (y, x, dist) = queue.pop_front().unwrap();

                if visited[y][x] {
                    continue;
                }

                visited[y][x] = true;
                dist_max = dist_max.max(dist);

                for k in 0..4 {
                    let y_next = y as i64 + dy[k];
                    let x_next = x as i64 + dx[k];

                    if y_next < 0 || y_next >= n as i64 || x_next < 0 || x_next >= m as i64 {
                        continue;
                    }

                    let y_next = y_next as usize;
                    let x_next = x_next as usize;

                    if map[y_next][x_next] == 'W' {
                        continue;
                    }

                    queue.push_back((y_next, x_next, dist + 1));
                }
            }

            ret = ret.max(dist_max);
        }
    }

    writeln!(out, "{ret}").unwrap();
}
