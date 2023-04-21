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

    let (h, w) = (scan.token::<usize>(), scan.token::<usize>());
    let mut map = vec![vec!['.'; w]; h];
    let mut dist = vec![vec![i64::MAX / 2; w]; h];
    let mut start = (0, 0);
    let mut end = (0, 0);

    for i in 0..h {
        let s = scan.token::<String>();

        for (j, c) in s.chars().enumerate() {
            map[i][j] = c;

            if c == 'K' {
                start = (i as i64, j as i64);
            } else if c == '*' {
                end = (i as i64, j as i64);
            }
        }
    }

    dist[start.0 as usize][start.1 as usize] = 0;

    let dx = [-1, 1, 0, 0, -1, -1, 1, 1];
    let dy = [0, 0, -1, 1, -1, 1, -1, 1];

    let mut deque = VecDeque::new();
    deque.push_back(start);

    while !deque.is_empty() {
        let (x, y) = deque.pop_front().unwrap();

        if x == end.0 && y == end.1 {
            writeln!(out, "{}", dist[x as usize][y as usize]).unwrap();
            return;
        }

        for i in 0..8 {
            let (next_x, next_y) = (x as i64 + dx[i], y as i64 + dy[i]);

            if next_x < 0 || next_x >= h as i64 || next_y < 0 || next_y >= w as i64 {
                continue;
            }

            if map[next_x as usize][next_y as usize] == '#' {
                continue;
            }

            if next_y == y + 1 {
                if dist[next_x as usize][next_y as usize] > dist[x as usize][y as usize] {
                    dist[next_x as usize][next_y as usize] = dist[x as usize][y as usize];
                    deque.push_front((next_x, next_y));
                }
            } else {
                if dist[next_x as usize][next_y as usize] > dist[x as usize][y as usize] + 1 {
                    dist[next_x as usize][next_y as usize] = dist[x as usize][y as usize] + 1;
                    deque.push_back((next_x, next_y));
                }
            }
        }
    }

    writeln!(out, "-1").unwrap();
}
