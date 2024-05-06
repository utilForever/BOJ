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

    let (n, m) = (scan.token::<usize>(), scan.token::<usize>());
    let (t_g, t_b, _, _) = (
        scan.token::<i64>(),
        scan.token::<i64>(),
        scan.token::<i64>(),
        scan.token::<i64>(),
    );
    let mut grid = vec![vec![' '; m]; n];
    let mut time = vec![vec![i64::MAX / 2; m]; n];
    let mut pq = BinaryHeap::new();

    for i in 0..n {
        let line = scan.token::<String>();

        for (j, c) in line.chars().enumerate() {
            grid[i][j] = c;

            if c == '*' {
                time[i][j] = 0;
                pq.push(Reverse((0, i, j)));
            }
        }
    }

    let dy: [i64; 4] = [1, -1, 0, 0];
    let dx: [i64; 4] = [0, 0, 1, -1];

    while !pq.is_empty() {
        let (t, y, x) = pq.pop().unwrap().0;

        if time[y][x] != t {
            continue;
        }

        for i in 0..4 {
            let (y_next, x_next) = (y as i64 + dy[i], x as i64 + dx[i]);

            if y_next < 0 || y_next >= n as i64 || x_next < 0 || x_next >= m as i64 {
                continue;
            }

            let (y_next, x_next) = (y_next as usize, x_next as usize);

            let t_next = if grid[y_next][x_next] == '#' {
                time[y][x] + t_b + 1
            } else {
                time[y][x] + 1
            };

            if time[y_next][x_next] > t_next {
                time[y_next][x_next] = t_next;
                pq.push(Reverse((t_next, y_next, x_next)));
            }
        }
    }

    let mut ret = Vec::new();

    for i in 0..n {
        for j in 0..m {
            if time[i][j] > t_g {
                ret.push((i, j));
            }
        }
    }

    if ret.is_empty() {
        writeln!(out, "-1").unwrap();
        return;
    }

    for (y, x) in ret {
        writeln!(out, "{} {}", y + 1, x + 1).unwrap();
    }
}
