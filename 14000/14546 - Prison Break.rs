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

    let p = scan.token::<i64>();

    for _ in 0..p {
        let (l, w, a, b, c, d) = (
            scan.token::<usize>(),
            scan.token::<usize>(),
            scan.token::<usize>() - 1,
            scan.token::<usize>() - 1,
            scan.token::<usize>() - 1,
            scan.token::<usize>() - 1,
        );
        let mut labyrinth = vec![vec![' '; l]; w];

        for i in (0..w).rev() {
            let line = scan.token::<String>();

            for (j, c) in line.chars().enumerate() {
                labyrinth[i][j] = c;
            }
        }

        let dy = [0, 0, 1, -1];
        let dx = [1, -1, 0, 0];

        let mut queue = VecDeque::new();
        let mut visited = vec![vec![false; l]; w];
        let label = labyrinth[b][a];
        
        queue.push_back((b, a));
        visited[b][a] = true;

        while !queue.is_empty() {
            let (y, x) = queue.pop_front().unwrap();

            for i in 0..4 {
                let (y_next, x_next) = (y as i64 + dy[i], x as i64 + dx[i]);

                if y_next < 0 || y_next >= w as i64 || x_next < 0 || x_next >= l as i64 {
                    continue;
                }

                let (y_next, x_next) = (y_next as usize, x_next as usize);

                if visited[y_next][x_next] {
                    continue;
                }

                if labyrinth[y_next][x_next] != label {
                    continue;
                }

                queue.push_back((y_next, x_next));
                visited[y_next][x_next] = true;
            }
        }

        writeln!(out, "{}", if visited[d][c] { "YES" } else { "NO" }).unwrap();
    }
}
