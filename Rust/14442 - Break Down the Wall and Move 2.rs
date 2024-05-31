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

    let (n, m, k) = (
        scan.token::<usize>(),
        scan.token::<usize>(),
        scan.token::<usize>(),
    );
    let mut maze = vec![vec![0; m]; n];
    let mut visited = vec![vec![vec![0; k + 1]; m]; n];

    for i in 0..n {
        let line = scan.token::<String>();

        for (j, c) in line.chars().enumerate() {
            maze[i][j] = c as u8 - b'0';
        }
    }

    let dy = [-1, 1, 0, 0];
    let dx = [0, 0, -1, 1];
    let mut queue = VecDeque::new();

    queue.push_back((0, 0, 0));
    visited[0][0][0] = 1;

    while !queue.is_empty() {
        let (y_curr, x_curr, cnt_break) = queue.pop_front().unwrap();

        if y_curr == n - 1 && x_curr == m - 1 {
            writeln!(out, "{}", visited[y_curr][x_curr][cnt_break]).unwrap();
            return;
        }

        for i in 0..4 {
            let y_next = y_curr as i32 + dy[i];
            let x_next = x_curr as i32 + dx[i];

            if y_next < 0 || y_next >= n as i32 || x_next < 0 || x_next >= m as i32 {
                continue;
            }

            let y_next = y_next as usize;
            let x_next = x_next as usize;

            if maze[y_next][x_next] == 0 {
                if visited[y_next][x_next][cnt_break] > 0 {
                    continue;
                }

                visited[y_next][x_next][cnt_break] = visited[y_curr][x_curr][cnt_break] + 1;
                queue.push_back((y_next, x_next, cnt_break));
            } else if maze[y_next][x_next] == 1 && cnt_break < k {
                if visited[y_next][x_next][cnt_break + 1] > 0 {
                    continue;
                }

                visited[y_next][x_next][cnt_break + 1] = visited[y_curr][x_curr][cnt_break] + 1;
                queue.push_back((y_next, x_next, cnt_break + 1));
            }
        }
    }

    writeln!(out, "-1").unwrap();
}
