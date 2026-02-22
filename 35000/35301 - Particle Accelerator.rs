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

struct Observation {
    pos: usize,
    time_start: i64,
    cost: usize,
    value: i64,
    capacity: usize,
    visited: bool,
}

impl Observation {
    fn new(
        pos: usize,
        time_start: i64,
        cost: usize,
        value: i64,
        capacity: usize,
        visited: bool,
    ) -> Self {
        Self {
            pos,
            time_start,
            cost,
            value,
            capacity,
            visited,
        }
    }
}

fn upper_bound(arr: &Vec<i64>, x: i64) -> usize {
    let mut left = 0;
    let mut right = arr.len();

    while left < right {
        let mid = (left + right) / 2;

        if arr[mid] <= x {
            left = mid + 1;
        } else {
            right = mid;
        }
    }

    left
}

fn count_up_to(times: &Vec<Vec<i64>>, period: i64, pos: usize, x: i64) -> i64 {
    if x <= 0 {
        return 0;
    }

    let (q, r) = (x / period, x % period);
    q * (times[pos].len() as i64) + upper_bound(&times[pos], r) as i64
}

fn count_range(times: &Vec<Vec<i64>>, period: i64, pos: usize, left: i64, right: i64) -> i64 {
    if right < left {
        return 0;
    }

    count_up_to(times, period, pos, right) - count_up_to(times, period, pos, left - 1)
}

fn update(
    prev: &Vec<i64>,
    next: &mut Vec<i64>,
    buf: &mut Vec<(usize, i64)>,
    cost: usize,
    value: i64,
    capacity: i64,
    b: usize,
    quantity: usize,
) {
    for x in next.iter_mut() {
        *x = i64::MIN / 4;
    }

    for remain in 0..cost {
        buf.clear();

        let mut head = 0;
        let mut time = 0;

        while remain + time * cost <= b {
            let idx = remain + time * cost;

            if prev[idx] != i64::MIN / 4 {
                let candidate = prev[idx] - (time as i64) * value;

                while buf.len() > head && buf.last().unwrap().1 <= candidate {
                    buf.pop();
                }

                buf.push((time, candidate));
            }

            while buf.len() > head && buf[head].0 + quantity < time {
                head += 1;
            }

            if buf.len() > head {
                next[idx] = (buf[head].1 + (time as i64) * value).min(capacity);
            }

            time += 1;
        }
    }
}

fn check(
    observations: &Vec<Observation>,
    times: &Vec<Vec<i64>>,
    period: i64,
    b: usize,
    target: i64,
    x: i64,
) -> bool {
    let mut dp = vec![i64::MIN / 4; b + 1];
    let mut dp_next = vec![i64::MIN / 4; b + 1];

    dp[0] = 0;

    let mut buf = Vec::with_capacity(b + 1);

    for observation in observations {
        if observation.capacity == 0 || !observation.visited || observation.time_start > x {
            continue;
        }

        let opportunity = count_range(times, period, observation.pos, observation.time_start, x);

        if opportunity <= 0 {
            continue;
        }

        let quantity = (opportunity.min(observation.capacity as i64)) as usize;

        if quantity == 0 {
            continue;
        }

        update(
            &dp,
            &mut dp_next,
            &mut buf,
            observation.cost,
            observation.value,
            target,
            b,
            quantity,
        );

        std::mem::swap(&mut dp, &mut dp_next);

        if dp.iter().any(|&x| x >= target) {
            return true;
        }
    }

    dp.iter().any(|&x| x >= target)
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let (n, a, v) = (
        scan.token::<i64>(),
        scan.token::<i64>(),
        scan.token::<i64>(),
    );
    let (m, b, g_target) = (
        scan.token::<usize>(),
        scan.token::<usize>(),
        scan.token::<i64>(),
    );
    let period = 2 * n;
    let mut times = vec![Vec::new(); n as usize];
    let mut step = v % n;
    let mut pos = 0;

    for i in 1..=period {
        pos = (pos + step) % n;
        step = (step + a) % n;
        times[pos as usize].push(i);
    }

    let mut observations = Vec::with_capacity(m);
    let mut start_max = 0;

    for _ in 0..m {
        let (p, s, c, g, k) = (
            scan.token::<usize>() - 1,
            scan.token::<i64>(),
            scan.token::<usize>(),
            scan.token::<i64>(),
            scan.token::<i64>(),
        );

        let visited = !times[p].is_empty();
        let capacity = (k.min((b / c) as i64)) as usize;
        let mut start = i64::MAX / 4;

        if visited && capacity > 0 {
            if v >= s {
                start = 1;
            } else if a != 0 {
                let diff = s - v;
                start = ((diff + a - 1) / a).max(1);
            }
        }

        if start != i64::MAX / 4 && start > start_max {
            start_max = start;
        }

        observations.push(Observation::new(p, start, c, g, capacity, visited));
    }

    if start_max == 0 {
        writeln!(out, "NO").unwrap();
        return;
    }

    let upper_bound = start_max + period * b as i64;

    if !check(&observations, &times, period, b, g_target, upper_bound) {
        writeln!(out, "NO").unwrap();
        return;
    }

    let mut left = 1;
    let mut right = upper_bound;

    while left < right {
        let mid = (left + right) / 2;

        if check(&observations, &times, period, b, g_target, mid) {
            right = mid;
        } else {
            left = mid + 1;
        }
    }

    writeln!(out, "YES").unwrap();
    writeln!(out, "{left}").unwrap();
}
