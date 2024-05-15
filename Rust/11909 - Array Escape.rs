use io::Write;
use std::{collections::BinaryHeap, io, str};

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

fn process_dijkstra(graph: &Vec<Vec<i32>>, from: (usize, usize)) -> Vec<Vec<i32>> {
    let n = graph.len();
    let mut ret = vec![vec![i32::MAX / 4; n]; n];
    ret[from.0][from.1] = 0;

    let mut queue = BinaryHeap::new();
    queue.push((0, from));

    while !queue.is_empty() {
        let (mut cost_curr, (y_curr, x_curr)) = queue.pop().unwrap();
        cost_curr *= -1;

        // (y, x) -> (y + 1, x)
        let dy = [1, 0];
        let dx = [0, 1];

        for i in 0..2 {
            let y_next = y_curr + dy[i];
            let x_next = x_curr + dx[i];

            if y_next >= n || x_next >= n {
                continue;
            }

            let mut cost_next = cost_curr;

            if graph[y_curr][x_curr] <= graph[y_next][x_next] {
                cost_next += graph[y_next][x_next] - graph[y_curr][x_curr] + 1
            }

            if ret[y_next][x_next] > cost_next {
                ret[y_next][x_next] = cost_next;
                queue.push((-cost_next, (y_next, x_next)));
            }
        }
    }

    ret
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let n = scan.token::<usize>();
    let mut nums = vec![vec![0; n]; n];

    for i in 0..n {
        for j in 0..n {
            nums[i][j] = scan.token::<i32>();
        }
    }

    let ret = process_dijkstra(&nums, (0, 0));

    writeln!(out, "{}", ret[n - 1][n - 1]).unwrap();
}
