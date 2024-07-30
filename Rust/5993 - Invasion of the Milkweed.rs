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

    let (x, y, mx, my) = (
        scan.token::<usize>(),
        scan.token::<usize>(),
        scan.token::<usize>() - 1,
        scan.token::<usize>() - 1,
    );
    let mut field = vec![vec![' '; x]; y];

    for i in 0..y {
        let line = scan.token::<String>();

        for (j, c) in line.chars().enumerate() {
            field[i][j] = c;
        }
    }

    let mut queue = VecDeque::new();
    let mut visited = vec![vec![false; x]; y];
    let mut ret = 0;

    queue.push_back((y - my - 1, mx, 0));
    visited[y - my - 1][mx] = true;

    let dy = [0, 0, -1, -1, -1, 1, 1, 1];
    let dx = [-1, 1, -1, 0, 1, -1, 0, 1];

    while !queue.is_empty() {
        let (y_curr, x_curr, depth) = queue.pop_front().unwrap();

        ret = ret.max(depth);

        for i in 0..8 {
            let y_next = y_curr as i32 + dy[i];
            let x_next = x_curr as i32 + dx[i];

            if y_next < 0 || y_next >= y as i32 || x_next < 0 || x_next >= x as i32 {
                continue;
            }

            let y_next = y_next as usize;
            let x_next = x_next as usize;

            if visited[y_next][x_next] || field[y_next][x_next] == '*' {
                continue;
            }

            visited[y_next][x_next] = true;
            queue.push_back((y_next, x_next, depth + 1));
        }
    }

    writeln!(out, "{ret}").unwrap();
}
