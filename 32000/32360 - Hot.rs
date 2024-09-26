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

    let (n, m) = (scan.token::<usize>(), scan.token::<usize>());
    let (k, c) = (scan.token::<i64>(), scan.token::<i64>());
    let mut outside = vec![vec![' '; m]; n];
    let mut pos_start = (0, 0);
    let mut pos_end = (0, 0);

    for i in 0..n {
        let line = scan.token::<String>();

        for (j, c) in line.chars().enumerate() {
            outside[i][j] = c;

            if c == 'S' {
                pos_start = (i, j);
            } else if c == 'E' {
                pos_end = (i, j);
            }
        }
    }

    let dy = [-1, 0, 1, 0, 0];
    let dx = [0, 1, 0, -1, 0];

    let mut queue = VecDeque::new();
    let mut visited = vec![vec![vec![-1; 100]; m]; n];

    queue.push_back((pos_start.0, pos_start.1, 0));
    visited[pos_start.0][pos_start.1][0] = 0;

    while !queue.is_empty() {
        let (y_curr, x_curr, hot_curr) = queue.pop_front().unwrap();

        for i in 0..5 {
            let y_next = y_curr as i64 + dy[i];
            let x_next = x_curr as i64 + dx[i];

            if y_next < 0 || y_next >= n as i64 || x_next < 0 || x_next >= m as i64 {
                continue;
            }

            let y_next = y_next as usize;
            let x_next = x_next as usize;
            let mut hot_next = hot_curr;

            if outside[y_next][x_next] == '#' {
                continue;
            }

            if outside[y_next][x_next] == 'H' {
                hot_next = (hot_next - k).max(0);
            } else {
                hot_next += c;

                if hot_next >= 100 {
                    continue;
                }
            }

            let hot_curr = hot_curr as usize;
            let hot_next = hot_next as usize;

            if visited[y_next][x_next][hot_next] != -1 {
                continue;
            }

            if y_next == pos_end.0 && x_next == pos_end.1 {
                writeln!(out, "{}", visited[y_curr][x_curr][hot_curr] + 1).unwrap();
                return;
            }

            visited[y_next][x_next][hot_next] = visited[y_curr][x_curr][hot_curr] + 1;
            queue.push_back((y_next, x_next, hot_next as i64));
        }
    }

    writeln!(out, "-1").unwrap();
}
