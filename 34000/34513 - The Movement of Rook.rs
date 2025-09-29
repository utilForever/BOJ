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

    pub fn all(&mut self) -> String {
        let mut input = String::new();
        self.reader.read_to_string(&mut input).expect("Failed read");
        input
    }

    pub fn line(&mut self) -> String {
        let mut input = String::new();
        self.reader.read_line(&mut input).expect("Failed read");
        input
    }
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let n = scan.token::<usize>();
    let mut board = vec![vec!['.'; n]; n];
    let mut pos_rook = (0, 0);
    let mut pos_king = (0, 0);

    for i in 0..n {
        let line = scan.token::<String>();

        for (j, c) in line.chars().enumerate() {
            board[i][j] = c;

            if c == 'R' {
                pos_rook = (i, j);
                board[i][j] = '.';
            } else if c == 'K' {
                pos_king = (i, j);
            }
        }
    }

    let dy = [0, -1, 1, 0, 0];
    let dx = [0, 0, 0, -1, 1];
    let start = (pos_rook.0 * n + pos_rook.1) * 5;
    let end = (pos_king.0 * n + pos_king.1) * 5;

    let mut dist = vec![i64::MAX / 4; n * n * 5];
    let mut deque = VecDeque::new();

    dist[start] = 0;
    deque.push_back(start);

    while let Some(idx) = deque.pop_front() {
        if idx == end {
            writeln!(out, "{}", dist[idx]).unwrap();
            return;
        }

        let curr = idx / 5;
        let dir = idx % 5;
        let (y_curr, x_curr) = (curr / n, curr % n);

        if dir == 0 {
            for i in 1..5 {
                let y_next = y_curr as i64 + dy[i];
                let x_next = x_curr as i64 + dx[i];

                if y_next < 0 || y_next >= n as i64 || x_next < 0 || x_next >= n as i64 {
                    continue;
                }

                let y_next = y_next as usize;
                let x_next = x_next as usize;

                match board[y_next][x_next] {
                    'B' => {}
                    'W' | 'K' => {
                        let idx_next = (y_next * n + x_next) * 5;

                        if dist[idx] + 1 < dist[idx_next] {
                            dist[idx_next] = dist[idx] + 1;
                            deque.push_back(idx_next);
                        }
                    }
                    '.' => {
                        let idx_next = (y_next * n + x_next) * 5 + i;

                        if dist[idx] + 1 < dist[idx_next] {
                            dist[idx_next] = dist[idx] + 1;
                            deque.push_back(idx_next);
                        }
                    }
                    _ => unreachable!(),
                }
            }
        } else {
            let idx_idle = (y_curr * n + x_curr) * 5;

            if dist[idx] < dist[idx_idle] {
                dist[idx_idle] = dist[idx];
                deque.push_front(idx_idle);
            }

            let y_next = y_curr as i64 + dy[dir];
            let x_next = x_curr as i64 + dx[dir];

            if y_next < 0 || y_next >= n as i64 || x_next < 0 || x_next >= n as i64 {
                continue;
            }

            let y_next = y_next as usize;
            let x_next = x_next as usize;

            match board[y_next][x_next] {
                'B' => {}
                'W' | 'K' => {
                    let idx_next = (y_next * n + x_next) * 5;

                    if dist[idx] < dist[idx_next] {
                        dist[idx_next] = dist[idx];
                        deque.push_front(idx_next);
                    }
                }
                '.' => {
                    let idx_next = (y_next * n + x_next) * 5 + dir;

                    if dist[idx] < dist[idx_next] {
                        dist[idx_next] = dist[idx];
                        deque.push_front(idx_next);
                    }
                }
                _ => unreachable!(),
            }
        }
    }

    writeln!(out, "-1").unwrap();
}
