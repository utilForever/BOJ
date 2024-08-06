use io::Write;
use std::{collections::VecDeque, io, str};

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

fn calculate_cost(accumulated_cost: &Vec<i64>, i: usize, j: usize) -> i64 {
    (j as i64 - i as i64) * (accumulated_cost[j] - accumulated_cost[i])
}

fn cross(cost: &Vec<i64>, accumulated_cost: &Vec<i64>, i: usize, j: usize, n: usize) -> usize {
    let mut left = j;
    let mut right = n;

    while left < right {
        let mid = (left + right + 1) / 2;

        if cost[i] + calculate_cost(accumulated_cost, i, mid)
            <= cost[j] + calculate_cost(accumulated_cost, j, mid)
        {
            left = mid;
        } else {
            right = mid - 1;
        }
    }

    left
}

fn calculate(
    cost: &mut Vec<i64>,
    cnt: &mut Vec<i64>,
    accumulated_cost: &Vec<i64>,
    mid: i64,
    n: usize,
) {
    let mut deque = VecDeque::new();
    deque.push_back(0);

    for i in 1..=n {
        while deque.len() >= 2 && cross(cost, accumulated_cost, deque[0], deque[1], n) < i {
            deque.pop_front();
        }

        cost[i] = cost[deque[0]] + calculate_cost(accumulated_cost, deque[0], i) + mid;
        cnt[i] = cnt[deque[0]] + 1;

        while deque.len() >= 2
            && cross(
                cost,
                accumulated_cost,
                deque[deque.len() - 2],
                deque[deque.len() - 1],
                n,
            ) > cross(cost, accumulated_cost, deque[deque.len() - 1], i, n)
        {
            deque.pop_back();
        }

        deque.push_back(i);
    }
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let (n, k) = (scan.token::<usize>(), scan.token::<i64>());
    let mut cost = vec![0; n + 1];
    let mut accumulated_cost = vec![0; n + 1];

    for i in 1..=n {
        cost[i] = scan.token::<i64>();
    }

    for i in 1..=n {
        accumulated_cost[i] = accumulated_cost[i - 1] + cost[i];
    }

    let mut cost = vec![0; n + 1];
    let mut cnt = vec![0; n + 1];

    let mut left = 0;
    let mut right = 100_000_000_000_000;
    let mut ret = 0;

    while left <= right {
        let mid = (left + right) / 2;
        calculate(&mut cost, &mut cnt, &accumulated_cost, mid, n);
        ret = ret.max(cost[n] - mid * k);

        if cnt[n] > k {
            left = mid + 1;
        } else if cnt[n] < k {
            right = mid - 1;
        } else {
            break;
        }
    }

    writeln!(out, "{ret}").unwrap();
}
