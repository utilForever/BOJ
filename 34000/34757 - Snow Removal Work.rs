use io::Write;
use std::{io, str};

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

struct FenwickTree {
    n: usize,
    data: Vec<i64>,
}

impl FenwickTree {
    fn new(n: usize) -> Self {
        FenwickTree {
            n,
            data: vec![0; n + 1],
        }
    }

    fn update(&mut self, mut idx: usize, delta: i64) {
        while idx <= self.n {
            self.data[idx] += delta;
            idx += idx & (!idx + 1);
        }
    }

    fn query(&self, mut idx: usize) -> i64 {
        let mut ret = 0;

        while idx > 0 {
            ret += self.data[idx];
            idx -= idx & (!idx + 1);
        }

        ret
    }

    fn query_range(&self, left: usize, right: usize) -> i64 {
        self.query(right) - self.query(left - 1)
    }
}

fn find(parent: &mut Vec<usize>, mut x: usize) -> usize {
    while parent[x] != x {
        let parent_x = parent[x];
        let parent_p = parent[parent_x];

        if parent_x != parent_p {
            parent[x] = parent_p;
        }

        x = parent_x;
    }

    x
}

#[derive(Clone, Copy)]
struct Event {
    time: usize,
    idx: usize,
    delta: i64,
}

impl Event {
    fn new(t: usize, idx: usize, delta: i64) -> Self {
        Self {
            time: t,
            idx,
            delta,
        }
    }
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let (n, m, q) = (
        scan.token::<usize>(),
        scan.token::<usize>(),
        scan.token::<usize>(),
    );
    let mut snows = vec![0; n + 1];

    for i in 1..=n {
        snows[i] = scan.token::<i64>();
    }

    let mut tasks = vec![(0, 0, 0); m + 1];

    for i in 1..=m {
        tasks[i] = (
            scan.token::<usize>(),
            scan.token::<usize>(),
            scan.token::<i64>(),
        );
    }

    let mut queries = vec![(0, 0, 0); q];

    for i in 0..q {
        queries[i] = (
            scan.token::<usize>(),
            scan.token::<usize>(),
            scan.token::<i64>(),
        );
    }

    let mut parent = vec![0; n + 2];

    for i in 1..=n + 1 {
        parent[i] = i;
    }

    let mut events = Vec::with_capacity(n + m + 1);

    for i in 1..=m {
        let mut capacity = tasks[i].2;

        if capacity <= 0 {
            continue;
        }

        let mut pos = find(&mut parent, tasks[i].0);

        while capacity > 0 && pos <= tasks[i].1 {
            if snows[pos] <= 0 {
                let next = find(&mut parent, pos + 1);
                parent[pos] = next;
                pos = next;
                continue;
            }

            if snows[pos] <= capacity {
                events.push(Event::new(i, pos, snows[pos]));
                capacity -= snows[pos];
                snows[pos] = 0;

                let next = find(&mut parent, pos + 1);
                parent[pos] = next;
                pos = next;
            } else {
                events.push(Event::new(i, pos, capacity));
                snows[pos] -= capacity;
                capacity = 0;
            }
        }
    }

    let mut left = vec![1; q];
    let mut right = vec![m + 1; q];
    let mut queries_remain = (0..q).collect::<Vec<_>>();

    while !queries_remain.is_empty() {
        let mut tasks_local = Vec::with_capacity(queries_remain.len());
        let mut tree = FenwickTree::new(n);

        for &idx in queries_remain.iter() {
            if left[idx] < right[idx] {
                let mid = (left[idx] + right[idx]) / 2;
                tasks_local.push((mid, idx));
            }
        }

        if tasks_local.is_empty() {
            break;
        }

        tasks_local.sort_unstable_by_key(|&(mid, _)| mid);

        let mut idx_event = 0;

        for (mid, idx) in tasks_local {
            while idx_event < events.len() && events[idx_event].time <= mid {
                let event = events[idx_event];
                tree.update(event.idx, event.delta);
                idx_event += 1;
            }

            let sum = tree.query_range(queries[idx].0, queries[idx].1);

            if sum >= queries[idx].2 {
                right[idx] = mid;
            } else {
                left[idx] = mid + 1;
            }
        }

        let mut next = Vec::with_capacity(queries_remain.len());

        for idx in queries_remain {
            if left[idx] < right[idx] {
                next.push(idx);
            }
        }

        queries_remain = next;
    }

    for i in 0..q {
        if left[i] == m + 1 {
            writeln!(out, "-1").unwrap();
        } else {
            writeln!(out, "{}", left[i]).unwrap();
        }
    }
}
