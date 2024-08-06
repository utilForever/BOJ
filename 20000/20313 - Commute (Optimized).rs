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

    let (n, m, a, b) = (
        scan.token::<usize>(),
        scan.token::<usize>(),
        scan.token::<usize>(),
        scan.token::<usize>(),
    );
    let mut graph = vec![Vec::new(); n + 1];
    let mut costs = vec![Vec::new(); m];

    for i in 0..m {
        let (u, v, t) = (
            scan.token::<usize>(),
            scan.token::<usize>(),
            scan.token::<i64>(),
        );

        graph[u].push((v, i));
        graph[v].push((u, i));
        costs[i].push(t);
    }

    let k = scan.token::<usize>();

    for _ in 0..k {
        for i in 0..m {
            costs[i].push(scan.token::<i64>());
        }
    }

    let mut time = vec![vec![i64::MAX; k + 1]; n + 1];
    time[a][0] = 0;

    // (time, next, used_magic)
    let mut queue = BinaryHeap::new();
    queue.push(Reverse((0, a, 0)));

    while !queue.is_empty() {
        let (time_curr, vertex_curr, used_magic) = queue.pop().unwrap().0;

        for &(next, idx) in graph[vertex_curr].iter() {
            for j in used_magic..=k {
                let time_next = time_curr + costs[idx][j];

                if time[next][j] > time_next {
                    time[next][j] = time_next;
                    queue.push(Reverse((time_next, next, j)));
                }
            }
        }
    }

    let mut ret = i64::MAX;

    for i in 0..=k {
        ret = ret.min(time[b][i]);
    }

    writeln!(out, "{ret}").unwrap();
}
