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
}

fn calculate_dist(a: (usize, usize), b: (usize, usize)) -> i64 {
    (a.0 as i64 - b.0 as i64).abs() + (a.1 as i64 - b.1 as i64).abs()
}

fn calculate_dp(
    events: &Vec<(usize, usize)>,
    dp: &mut Vec<Vec<i64>>,
    police_selected: &mut Vec<Vec<i64>>,
    event_police1: usize,
    event_police2: usize,
    n: usize,
) -> i64 {
    let next = event_police1.max(event_police2) + 1;

    if next > events.len() {
        return 0;
    }

    if dp[event_police1][event_police2] != -1 {
        return dp[event_police1][event_police2];
    }

    let pos1 = if event_police1 == 0 {
        (1, 1)
    } else {
        events[event_police1 - 1]
    };
    let pos2 = if event_police2 == 0 {
        (n, n)
    } else {
        events[event_police2 - 1]
    };

    let cost1 = calculate_dist(pos1, events[next - 1])
        + calculate_dp(events, dp, police_selected, next, event_police2, n);
    let cost2 = calculate_dist(pos2, events[next - 1])
        + calculate_dp(events, dp, police_selected, event_police1, next, n);

    if cost1 < cost2 {
        dp[event_police1][event_police2] = cost1;
        police_selected[event_police1][event_police2] = 1;
    } else {
        dp[event_police1][event_police2] = cost2;
        police_selected[event_police1][event_police2] = 2;
    }

    dp[event_police1][event_police2]
}

fn process_backtrack(
    police_selected: &Vec<Vec<i64>>,
    tracking: &mut Vec<i64>,
    event_police1: usize,
    event_police2: usize,
) {
    let next = event_police1.max(event_police2) + 1;

    if next >= police_selected.len() {
        return;
    }

    let selected = police_selected[event_police1][event_police2];
    tracking.push(selected);

    if selected == 1 {
        process_backtrack(police_selected, tracking, next, event_police2);
    } else {
        process_backtrack(police_selected, tracking, event_police1, next);
    }
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let n = scan.token::<usize>();
    let w = scan.token::<usize>();
    let mut events = vec![(0, 0); w];

    for i in 0..w {
        let (a, b) = (scan.token::<usize>(), scan.token::<usize>());
        events[i] = (a, b);
    }

    let mut dp = vec![vec![-1; w + 1]; w + 1];
    let mut police_selected = vec![vec![0; w + 1]; w + 1];
    let mut tracking = Vec::new();

    let ret = calculate_dp(&events, &mut dp, &mut police_selected, 0, 0, n);
    process_backtrack(&police_selected, &mut tracking, 0, 0);

    writeln!(out, "{ret}").unwrap();

    for val in tracking.iter() {
        writeln!(out, "{val}").unwrap();
    }
}
