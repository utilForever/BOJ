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

// Reference: https://infossm.github.io/blog/2019/01/09/wrong-dijkstra/
fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let (n, m, k) = (
        scan.token::<usize>(),
        scan.token::<i64>(),
        scan.token::<usize>(),
    );
    let mut graph = vec![Vec::new(); n + 1];
    let mut time = vec![vec![i64::MAX; k + 1]; n + 1];

    for _ in 0..m {
        let (a, b, c) = (
            scan.token::<usize>(),
            scan.token::<usize>(),
            scan.token::<i64>(),
        );
        graph[a].push((b, c));
        graph[b].push((a, c));
    }

    time[1][1] = 0;

    // (time, next, used_highways)
    let mut queue = BinaryHeap::new();
    queue.push(Reverse((0, 1, 0)));

    while !queue.is_empty() {
        let (time_curr, vertex_curr, used_highways) = queue.pop().unwrap().0;

        if time[vertex_curr][used_highways] < time_curr {
            continue;
        }

        for &(next, cost) in graph[vertex_curr].iter() {
            if used_highways < k {
                let time_next = time_curr;

                if time[next][used_highways + 1] > time_next {
                    time[next][used_highways + 1] = time_next;
                    queue.push(Reverse((time_next, next, used_highways + 1)));
                }
            }

            let time_next = time_curr + cost;

            if time[next][used_highways] > time_next {
                time[next][used_highways] = time_next;
                queue.push(Reverse((time_next, next, used_highways)));
            }
        }
    }

    let mut ret = i64::MAX;

    for i in 0..=k {
        ret = ret.min(time[n][i]);
    }

    writeln!(out, "{ret}").unwrap();
}
