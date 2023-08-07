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

fn process_bfs(field: &Vec<Vec<char>>, visited: &mut Vec<Vec<bool>>, n: usize) -> bool {
    let dx: [i64; 2] = [1, 0];
    let dy: [i64; 2] = [0, 1];

    let mut queue = VecDeque::new();

    queue.push_back((0, 0));
    visited[0][0] = true;

    while !queue.is_empty() {
        let (x, y) = queue.pop_front().unwrap();

        for k in 0..2 {
            let next_x = x as i64 + dx[k];
            let next_y = y as i64 + dy[k];

            if next_x < 0 || next_x >= n as i64 || next_y < 0 || next_y >= n as i64 {
                continue;
            }

            let next_x = next_x as usize;
            let next_y = next_y as usize;

            if visited[next_x][next_y] || field[next_x][next_y] == 'x' {
                continue;
            }

            visited[next_x][next_y] = true;
            queue.push_back((next_x, next_y));
        }
    }

    visited[n - 1][n - 1]
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let n = scan.token::<usize>();
    let mut field = vec![vec![' '; n]; n];
    let mut visited = vec![vec![false; n]; n];

    for i in 0..n {
        let s = scan.token::<String>();

        for (j, c) in s.chars().enumerate() {
            field[i][j] = c;
        }
    }

    writeln!(
        out,
        "{}",
        if process_bfs(&field, &mut visited, n) {
            "Yes"
        } else {
            "No"
        }
    )
    .unwrap();
}
