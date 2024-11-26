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

#[derive(Debug, Clone)]
struct Interval {
    start: i64,
    end: i64,
}

fn invert_intervals(intervals: &[Interval], time_total: i64) -> Vec<Interval> {
    let mut end_prev = 0;
    let mut ret = Vec::new();

    for interval in intervals {
        if interval.start > end_prev {
            ret.push(Interval {
                start: end_prev,
                end: interval.start,
            });
        }
        end_prev = end_prev.max(interval.end);
    }

    if end_prev < time_total {
        ret.push(Interval {
            start: end_prev,
            end: time_total,
        });
    }

    ret
}

fn merge_intervals(mut intervals: Vec<Interval>) -> Vec<Interval> {
    if intervals.is_empty() {
        return intervals;
    }

    intervals.sort_by_key(|x| x.start);

    let mut curr = intervals[0].clone();
    let mut ret = Vec::new();

    for interval in intervals.into_iter().skip(1) {
        if interval.start <= curr.end {
            curr.end = curr.end.max(interval.end);
        } else {
            ret.push(curr);
            curr = interval;
        }
    }

    ret.push(curr);
    ret
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let mut room_people = [0; 7];

    for i in 0..7 {
        room_people[i] = scan.token::<usize>();
    }

    let mut room_times = [0; 7];

    for i in 0..7 {
        room_times[i] = scan.token::<i64>();
    }

    let mut room_reservations = Vec::new();

    for i in 0..7 {
        let mut reservations = Vec::with_capacity(room_people[i]);

        for _ in 0..room_people[i] {
            let (x, y) = (scan.token::<i64>(), scan.token::<i64>());
            reservations.push(Interval { start: x, end: y });
        }

        room_reservations.push(reservations);
    }

    let mut availables = Vec::new();

    for i in 0..7 {
        let intervals_busy = merge_intervals(room_reservations[i].clone());
        let intervals_free = invert_intervals(&intervals_busy, room_times[i]);

        availables.extend(intervals_free);
    }

    let free_merged = merge_intervals(availables);
    let ret = free_merged.iter().map(|x| x.end - x.start).sum::<i64>();

    writeln!(out, "{ret}").unwrap();
}
