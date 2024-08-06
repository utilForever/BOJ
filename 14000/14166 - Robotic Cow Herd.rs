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

    let (n, k) = (scan.token::<usize>(), scan.token::<usize>());
    let mut costs = vec![Vec::new(); n];
    let mut cost_base = 0;

    for i in 0..n {
        let m = scan.token::<usize>();
        let mut cost = vec![i64::MAX; 10];

        for j in 0..m {
            cost[j] = scan.token::<i64>();
        }

        cost.sort();

        cost_base += cost[0];

        if cost.len() > 1 {
            cost.iter()
                .map(|&c| c - cost[0])
                .for_each(|c| costs[i].push(c));
        }
    }

    costs.sort();

    let mut priority_queue: BinaryHeap<Reverse<(i64, usize, usize)>> = BinaryHeap::new();
    let mut ret = 0;

    priority_queue.push(Reverse((0, 0, 0)));

    for _ in 0..k {
        let (cost, location, idx) = priority_queue.pop().unwrap().0;

        ret += cost_base + cost;

        if idx + 1 < 10 && costs[location][idx + 1] != i64::MAX {
            priority_queue.push(Reverse((
                cost + costs[location][idx + 1] - costs[location][idx],
                location,
                idx + 1,
            )));
        }

        if location + 1 < costs.len() {
            if idx >= 1 {
                priority_queue.push(Reverse((
                    cost + costs[location + 1][1] - costs[location + 1][0],
                    location + 1,
                    1,
                )));

                if idx == 1 {
                    priority_queue.push(Reverse((
                        cost + costs[location + 1][1] - costs[location + 1][0] - costs[location][1]
                            + costs[location][0],
                        location + 1,
                        1,
                    )));
                }
            }
        }
    }

    writeln!(out, "{}", ret).unwrap();
}
