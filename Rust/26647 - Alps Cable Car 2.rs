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

fn calculate_cost(heights: &Vec<i64>, accumulated_heights: &Vec<i64>, i: usize, j: usize) -> i64 {
    (heights[j] - heights[i]).pow(2) + (accumulated_heights[j] - accumulated_heights[i]).pow(2)
}

fn cross(
    cost: &Vec<i64>,
    heights: &Vec<i64>,
    accumulated_heights: &Vec<i64>,
    i: usize,
    j: usize,
    n: usize,
) -> usize {
    let mut left = j;
    let mut right = n;

    while left < right {
        let mid = (left + right + 1) / 2;

        if cost[i] + calculate_cost(heights, accumulated_heights, i, mid)
            <= cost[j] + calculate_cost(heights, accumulated_heights, j, mid)
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
    heights: &Vec<i64>,
    accumulated_heights: &Vec<i64>,
    mid: i64,
    n: usize,
) {
    let mut deque = VecDeque::new();
    deque.push_back(1);

    for i in 2..=n {
        while deque.len() >= 2
            && cross(cost, heights, accumulated_heights, deque[0], deque[1], n) < i
        {
            deque.pop_front();
        }

        cost[i] = cost[deque[0]] + calculate_cost(heights, accumulated_heights, deque[0], i) + mid;
        cnt[i] = cnt[deque[0]] + 1;

        while deque.len() >= 2
            && cross(
                cost,
                heights,
                accumulated_heights,
                deque[deque.len() - 2],
                deque[deque.len() - 1],
                n,
            ) > cross(
                cost,
                heights,
                accumulated_heights,
                deque[deque.len() - 1],
                i,
                n,
            )
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
    let mut heights = vec![0; n + 1];
    let mut accumulated_heights = vec![0; n + 1];

    for i in 1..=n {
        heights[i] = scan.token::<i64>();
    }

    for i in 2..=n {
        accumulated_heights[i] = accumulated_heights[i - 1] + heights[i - 1] + heights[i];
    }

    let mut cost = vec![0; n + 1];
    let mut cnt = vec![0; n + 1];

    let mut left = 0;
    let mut right = 100_000_000_000_000;
    let mut ret = 0;

    while left <= right {
        let mid = (left + right) / 2;
        calculate(&mut cost, &mut cnt, &heights, &accumulated_heights, mid, n);
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
