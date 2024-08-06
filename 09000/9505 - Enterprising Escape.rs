use io::Write;
use std::{
    collections::{BinaryHeap, HashMap},
    io, str,
};

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

    let t = scan.token::<i64>();

    for _ in 0..t {
        let (k, w, h) = (
            scan.token::<i64>(),
            scan.token::<usize>(),
            scan.token::<usize>(),
        );
        let mut battleships = HashMap::new();

        for _ in 0..k {
            let (name, cost) = (scan.token::<char>(), scan.token::<i64>());
            battleships.insert(name, cost);
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

        let mut graph = vec![vec![Vec::new(); w]; h];

        for i in 0..h {
            for j in 0..w {
                // Left
                if j > 0 && grid[i][j - 1] != 'E' {
                    let cost = battleships.get(&grid[i][j - 1]).unwrap();
                    graph[i][j].push((i, j - 1, *cost));
                }

                // Right
                if j < w - 1 && grid[i][j + 1] != 'E' {
                    let cost = battleships.get(&grid[i][j + 1]).unwrap();
                    graph[i][j].push((i, j + 1, *cost));
                }

                // Up
                if i > 0 && grid[i - 1][j] != 'E' {
                    let cost = battleships.get(&grid[i - 1][j]).unwrap();
                    graph[i][j].push((i - 1, j, *cost));
                }

                // Down
                if i < h - 1 && grid[i + 1][j] != 'E' {
                    let cost = battleships.get(&grid[i + 1][j]).unwrap();
                    graph[i][j].push((i + 1, j, *cost));
                }
            }
        }

        let mut queue = BinaryHeap::new();
        let mut dists = vec![vec![i64::MAX / 4; w]; h];

        queue.push((0, (start.0, start.1)));
        dists[start.0][start.1] = 0;

        while !queue.is_empty() {
            let (mut cost_curr, vertex_curr) = queue.pop().unwrap();
            cost_curr *= -1;

            if vertex_curr.0 == 0
                || vertex_curr.0 == h - 1
                || vertex_curr.1 == 0
                || vertex_curr.1 == w - 1
            {
                writeln!(out, "{cost_curr}").unwrap();
                break;
            }

            if dists[vertex_curr.0][vertex_curr.1] < cost_curr {
                continue;
            }

            for info in graph[vertex_curr.0][vertex_curr.1].iter() {
                let (vertex_next_y, vertex_next_x, mut cost_next) = *info;

                cost_next += cost_curr;

                if dists[vertex_next_y][vertex_next_x] > cost_next {
                    dists[vertex_next_y][vertex_next_x] = cost_next;
                    queue.push((-cost_next, (vertex_next_y, vertex_next_x)));
                }
            }
        }
    }
}
