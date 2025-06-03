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

const INF: i64 = i64::MAX / 4;
const DIRECTIONS: [(i64, i64); 4] = [(0, -1), (0, 1), (-1, 0), (1, 0)];

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let n = scan.token::<usize>();
    let mut horizontal = vec![vec![0; n - 1]; n];
    let mut vertical = vec![vec![0; n]; n - 1];

    for i in 0..n {
        for j in 0..n - 1 {
            horizontal[i][j] = scan.token::<i64>();
        }

        if i + 1 < n {
            for j in 0..n {
                vertical[i][j] = scan.token::<i64>();
            }
        }
    }

    let idx = |r: usize, c: usize, d: usize| -> usize { (r * n + c) * 3 + d };
    let mut dist = vec![INF; 3 * n * n];
    let mut pivot = vec![-1; 3 * n * n];

    let from = idx(0, 0, 2);
    dist[from] = 0;
    pivot[from] = 0;

    let mut priority_queue = BinaryHeap::<Reverse<(i64, i32, usize, usize, usize)>>::new();
    priority_queue.push(Reverse((0, 0, 0, 0, 2)));

    while let Some(Reverse((d, t, r, c, dir))) = priority_queue.pop() {
        let idx_curr = idx(r, c, dir);
        let t = -t;

        if d != dist[idx_curr] || t != pivot[idx_curr] {
            continue;
        }

        for (k, &(dr, dc)) in DIRECTIONS.iter().enumerate() {
            let r_next = r as i64 + dr;
            let c_next = c as i64 + dc;

            if r_next < 0 || r_next >= n as i64 || c_next < 0 || c_next >= n as i64 {
                continue;
            }

            let (r_next, c_next) = (r_next as usize, c_next as usize);
            let dir_next = if k < 2 { 0 } else { 1 };

            let w = if dir_next == 0 {
                if k == 0 {
                    horizontal[r][c - 1]
                } else {
                    horizontal[r][c]
                }
            } else {
                if k == 2 {
                    vertical[r - 1][c]
                } else {
                    vertical[r][c]
                }
            };

            let t_next = t + if dir != 2 && dir != dir_next { 1 } else { 0 };
            let d_next = d + w;
            let idx_next = idx(r_next, c_next, dir_next);

            if d_next < dist[idx_next] || (d_next == dist[idx_next] && t_next > pivot[idx_next]) {
                dist[idx_next] = d_next;
                pivot[idx_next] = t_next;

                priority_queue.push(Reverse((d_next, -t_next, r_next, c_next, dir_next)));
            }
        }
    }

    let mut ret_dist = INF;
    let mut ret_pivot = -1;

    for dir in 0..=2 {
        let id = idx(n - 1, n - 1, dir);
        let d = dist[id];
        let t = pivot[id];

        if d < ret_dist || (d == ret_dist && t > ret_pivot) {
            ret_dist = d;
            ret_pivot = t;
        }
    }

    writeln!(out, "{ret_dist} {ret_pivot}").unwrap();
}
