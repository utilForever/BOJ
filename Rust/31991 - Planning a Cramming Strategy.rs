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

#[derive(Default, Clone, Copy, Ord, PartialOrd, Eq, PartialEq)]
struct Course {
    time_remain: i64,
    credit: usize,
    times: [i64; 13],
}

const GRADE: [usize; 13] = [0, 7, 10, 13, 17, 20, 23, 27, 30, 33, 37, 40, 43];

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let n = scan.token::<usize>();
    let mut courses = vec![Course::default(); n];

    for i in 0..n {
        courses[i].time_remain = scan.token::<i64>();
        courses[i].credit = scan.token::<usize>();

        for j in 0..13 {
            courses[i].times[j] = scan.token::<i64>();
        }
    }

    // Sort in descending order of the course that has the most time remaining
    courses.sort_by(|a, b| b.time_remain.cmp(&a.time_remain));

    // DP table for the minimum time to complete the course according to the credit
    let mut times = vec![i64::MAX / 2; 43 * 4 * (n + 1) + 1];
    let mut offset = 43 * 4;

    times[0] = 0;

    for i in 0..n {
        // The reason why the time is calculated in reverse order is
        // to adjust the time to the minimum value according to previous courses
        for j in (0..=offset).rev() {
            for k in 0..13 {
                // idx: The index of the DP table to be updated
                // time: The consumed time to prepare the exam (excluding this course)
                // val: The consumed time to prepare the exam (including this course)
                let idx = j + courses[i].credit * GRADE[k];
                let time = (courses[0].time_remain - courses[i].time_remain).max(times[j]);
                let val = time + courses[i].times[k];

                times[idx] = times[idx].min(val);
            }
        }

        offset += 43 * 4;
    }

    let credits_total = courses.iter().map(|c| c.credit).sum::<usize>();

    for idx in (0..=n * 43 * 4).rev() {
        // Check the time is satisfied
        if times[idx] <= courses[0].time_remain {
            writeln!(out, "{:.9}", idx as f64 / (10.0 * credits_total as f64)).unwrap();
            break;
        }
    }
}
