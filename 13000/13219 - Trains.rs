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

fn process_dijkstra(graph: &Vec<Vec<i64>>, from: (usize, usize)) -> Vec<Vec<i64>> {
    let n = graph.len();
    let mut ret = vec![vec![i64::MAX / 4; n]; n];
    ret[from.0][from.1] = graph[from.0][from.1];

    let mut queue = BinaryHeap::new();
    queue.push((-ret[from.0][from.1], from));

    while !queue.is_empty() {
        let (mut cost_curr, (y_curr, x_curr)) = queue.pop().unwrap();
        cost_curr *= -1;

        let dy = [1, 0, -1, 0];
        let dx = [0, 1, 0, -1];

        for i in 0..4 {
            let y_next = y_curr as i32 + dy[i];
            let x_next = x_curr as i32 + dx[i];

            if y_next < 0 || y_next >= n as i32 || x_next < 0 || x_next >= n as i32 {
                continue;
            }

            let y_next = y_next as usize;
            let x_next = x_next as usize;

            if graph[y_next][x_next] == -1 {
                continue;
            }

            let cost_next = cost_curr + graph[y_next][x_next];

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
    let mut citizens = vec![vec![0; n]; n];
    let (ax, ay, bx, by) = (
        scan.token::<usize>() - 1,
        scan.token::<usize>() - 1,
        scan.token::<usize>() - 1,
        scan.token::<usize>() - 1,
    );

    for i in 0..n {
        for j in 0..n {
            citizens[i][j] = scan.token::<i64>();
        }
    }

    if citizens[ay][ax] == -1 || citizens[by][bx] == -1 {
        writeln!(out, "-1").unwrap();
        return;
    }

    let ret = process_dijkstra(&citizens, (ay, ax));

    writeln!(
        out,
        "{}",
        if ret[by][bx] == i64::MAX / 4 {
            -1
        } else {
            ret[by][bx]
        }
    )
    .unwrap();
}
