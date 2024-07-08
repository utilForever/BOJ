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
    let mut soldiers = vec![vec![' '; n]; m];
    let mut visited = vec![vec![false; n]; m];

    for i in 0..m {
        let line = scan.token::<String>();

        for (j, c) in line.chars().enumerate() {
            soldiers[i][j] = c;
        }
    }

    let dy = [0, 0, 1, -1];
    let dx = [1, -1, 0, 0];

    let mut queue = VecDeque::new();
    let mut ret_player = 0;
    let mut ret_enemy = 0;

    for i in 0..m {
        for j in 0..n {
            if visited[i][j] {
                continue;
            }

            queue.push_back((i, j));
            visited[i][j] = true;

            let mut cnt = 0;

            while !queue.is_empty() {
                let (y, x) = queue.pop_front().unwrap();

                cnt += 1;

                for k in 0..4 {
                    let (y_next, x_next) = (y as i32 + dy[k], x as i32 + dx[k]);

                    if y_next < 0 || y_next >= m as i32 || x_next < 0 || x_next >= n as i32 {
                        continue;
                    }

                    let (y_next, x_next) = (y_next as usize, x_next as usize);

                    if visited[y_next][x_next] {
                        continue;
                    }

                    if soldiers[i][j] != soldiers[y_next][x_next] {
                        continue;
                    }

                    queue.push_back((y_next, x_next));
                    visited[y_next][x_next] = true;
                }
            }

            if soldiers[i][j] == 'W' {
                ret_player += cnt * cnt;
            } else {
                ret_enemy += cnt * cnt;
            }
        }
    }

    writeln!(out, "{ret_player} {ret_enemy}").unwrap();
}
