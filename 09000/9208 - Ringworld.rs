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

    pub fn line(&mut self) -> String {
        let mut input = String::new();
        self.reader.read_line(&mut input).expect("Failed read");
        input
    }
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let t = scan.token::<i64>();

    for _ in 0..t {
        let (m, n) = (scan.token::<i64>(), scan.token::<i64>());
        let mut intervals = Vec::new();

        for _ in 0..n {
            let (x, y) = (scan.token::<i64>(), scan.token::<i64>());

            if x <= y {
                intervals.push((x, y));
                intervals.push((x + m, y + m));
            } else {
                intervals.push((x, y + m));
            }
        }

        if n > m {
            writeln!(out, "NO").unwrap();
            continue;
        }

        intervals.sort();

        let mut priority_queue: BinaryHeap<Reverse<i64>> = BinaryHeap::new();
        let mut idx = 0;
        let mut ret = true;

        'outer: while idx < intervals.len() {
            let x_curr = intervals[idx].0;

            priority_queue.push(Reverse(intervals[idx].1));
            idx += 1;

            // Push all intervals with the same start
            while idx < intervals.len() && intervals[idx].0 == x_curr {
                priority_queue.push(Reverse(intervals[idx].1));
                idx += 1;
            }

            // Select the next of x (becuase the maximum value of x is 2 * m - 1)
            let x_next = if idx < intervals.len() {
                intervals[idx].0
            } else {
                2 * m
            };

            let mut offset = 0;

            // Check if the intervals are disjoint with the same start
            while offset < x_next - x_curr && !priority_queue.is_empty() {
                if priority_queue.peek().unwrap().0 < x_curr + offset {
                    ret = false;
                    break 'outer;
                }

                priority_queue.pop();
                offset += 1;
            }
        }

        if !priority_queue.is_empty() {
            ret = false;
        }

        writeln!(out, "{}", if ret { "YES" } else { "NO" }).unwrap();
    }
}
