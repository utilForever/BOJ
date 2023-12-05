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

#[derive(Eq)]
struct RailroadTrack {
    from: usize,
    to: usize,
    cost: i64,
    time: i64,
}

impl RailroadTrack {
    fn new(from: usize, to: usize, cost: i64, time: i64) -> Self {
        Self {
            from,
            to,
            cost,
            time,
        }
    }
}

impl PartialEq for RailroadTrack {
    fn eq(&self, other: &Self) -> bool {
        self.cost == other.cost && self.time == other.time
    }
}

impl PartialOrd for RailroadTrack {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for RailroadTrack {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other
            .cost
            .cmp(&self.cost)
            .then_with(|| other.time.cmp(&self.time))
    }
}

fn find(parent: &mut Vec<usize>, node: usize) -> usize {
    if parent[node] == node {
        node
    } else {
        parent[node] = find(parent, parent[node]);
        parent[node]
    }
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let (n, q) = (scan.token::<usize>(), scan.token::<i64>());
    let mut railroad_tracks = BinaryHeap::new();

    for _ in 0..q {
        let (from, to, cost, time) = (
            scan.token::<usize>(),
            scan.token::<usize>(),
            scan.token::<i64>(),
            scan.token::<i64>(),
        );
        railroad_tracks.push(RailroadTrack::new(from, to, cost, time));
    }

    let mut parent = vec![0; n + 1];

    for i in 1..=n {
        parent[i] = i;
    }

    let mut ret_cost = 0;
    let mut ret_time = 0;

    while !railroad_tracks.is_empty() {
        let RailroadTrack {
            from,
            to,
            cost,
            time,
        } = railroad_tracks.pop().unwrap();
        let mut parent_from = find(&mut parent, from);
        let mut parent_to = find(&mut parent, to);

        if parent_from == parent_to {
            continue;
        }

        if parent_from > parent_to {
            std::mem::swap(&mut parent_from, &mut parent_to);
        }

        parent[parent_from] = parent_to;
        ret_cost += cost;
        ret_time = ret_time.max(time);
    }

    for i in 2..=n {
        if find(&mut parent, i) != find(&mut parent, 1) {
            writeln!(out, "-1").unwrap();
            return;
        }
    }

    writeln!(out, "{ret_time} {ret_cost}").unwrap();
}
