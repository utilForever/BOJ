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

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let (n, m, k) = (
        scan.token::<usize>(),
        scan.token::<usize>(),
        scan.token::<i64>(),
    );
    let mut time_min = vec![i64::MAX; n + 1];
    let mut bus_info = vec![Vec::new(); n + 1];

    for _ in 0..m {
        let (s, e, t, g) = (
            scan.token::<usize>(),
            scan.token::<usize>(),
            scan.token::<i64>(),
            scan.token::<i64>(),
        );
        bus_info[s].push((e, t, g));
    }

    time_min[1] = 0;

    // (time, next_vertex, remain_skill)
    let mut queue = BinaryHeap::new();
    queue.push((0, 1, k));

    while !queue.is_empty() {
        let (mut time_curr, vertex_curr, remain_skill) = queue.pop().unwrap();
        time_curr *= -1;

        for info in bus_info[vertex_curr].iter() {
            let (next_vertex, time_move, interval) = (info.0, info.1, info.2);

            // If remain time is 0, don't use skill
            if time_curr % interval == 0 {
                let time_next = time_curr + time_move;

                if time_min[next_vertex] > time_next {
                    time_min[next_vertex] = time_next;
                    queue.push((-time_next, next_vertex, remain_skill));
                }
            } else {
                if remain_skill > 0 {
                    let time_next = time_curr + time_move;

                    if time_min[next_vertex] > time_next {
                        time_min[next_vertex] = time_next;
                        queue.push((-time_next, next_vertex, remain_skill - 1));
                    }
                } else {
                    let time_next = time_curr + time_move + interval - (time_curr % interval);

                    if time_min[next_vertex] > time_next {
                        time_min[next_vertex] = time_next;
                        queue.push((-time_next, next_vertex, remain_skill));
                    }
                }
            }
        }
    }

    writeln!(
        out,
        "{}",
        if time_min[n] == i64::MAX {
            -1
        } else {
            time_min[n]
        }
    )
    .unwrap();
}
