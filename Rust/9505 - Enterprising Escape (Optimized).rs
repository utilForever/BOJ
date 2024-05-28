use io::Write;
use std::{cmp::Reverse, collections::BinaryHeap, io, str};

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

    let dy = [-1, 1, 0, 0];
    let dx = [0, 0, -1, 1];

    let t = scan.token::<i64>();

    for _ in 0..t {
        let (k, w, h) = (
            scan.token::<i64>(),
            scan.token::<usize>(),
            scan.token::<usize>(),
        );
        let mut battleships = vec![0; 26];

        for _ in 0..k {
            let (name, cost) = (scan.token::<char>(), scan.token::<i64>());
            battleships[(name as u8 - b'A') as usize] = cost;
        }

        let mut grid = vec![vec![' '; w]; h];
        let mut start = (0, 0);

        for i in 0..h {
            let row = scan.token::<String>();

            for (j, c) in row.chars().enumerate() {
                grid[i][j] = c;

                if c == 'E' {
                    start = (i, j);
                }
            }
        }

        let mut queue = BinaryHeap::new();
        let mut dists = vec![vec![i64::MAX / 4; w]; h];

        queue.push(Reverse((0, (start.0, start.1))));
        dists[start.0][start.1] = 0;

        while !queue.is_empty() {
            let (cost, curr) = queue.pop().unwrap().0;

            if curr.0 == 0 || curr.0 == h - 1 || curr.1 == 0 || curr.1 == w - 1 {
                writeln!(out, "{cost}").unwrap();
                break;
            }

            if dists[curr.0][curr.1] < cost {
                continue;
            }

            for i in 0..4 {
                let (next_y, next_x) = (curr.0 as i64 + dy[i], curr.1 as i64 + dx[i]);

                if next_y < 0 || next_y >= h as i64 || next_x < 0 || next_x >= w as i64 {
                    continue;
                }

                let (next_y, next_x) = (next_y as usize, next_x as usize);
                let next_cost = dists[curr.0][curr.1]
                    + battleships[(grid[next_y][next_x] as u8 - b'A') as usize];

                if next_cost < dists[next_y][next_x] {
                    dists[next_y][next_x] = next_cost;
                    queue.push(Reverse((next_cost, (next_y, next_x))));
                }
            }
        }
    }
}
