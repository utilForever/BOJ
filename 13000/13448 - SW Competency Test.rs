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

#[derive(Clone, Default)]
struct Problem {
    score: i64,
    penalty: i64,
    time: i64,
}

fn calculate(
    problems: &Vec<Problem>,
    max_values: &mut Vec<Vec<i64>>,
    n: usize,
    t: usize,
    index: usize,
    time: usize,
) -> i64 {
    if index == n {
        return 0;
    }

    if max_values[index][time] != -1 {
        return max_values[index][time];
    }

    // Don't solve problem (index)
    max_values[index][time] = calculate(problems, max_values, n, t, index + 1, time);

    // Solve problem (index)
    if time + problems[index].time as usize <= t {
        max_values[index][time] = max_values[index][time].max(
            problems[index].score - (time as i64 + problems[index].time) * problems[index].penalty
                + calculate(
                    problems,
                    max_values,
                    n,
                    t,
                    index + 1,
                    time + problems[index].time as usize,
                ),
        );
    }

    max_values[index][time]
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let (n, t) = (scan.token::<usize>(), scan.token::<i64>());
    let mut problems = vec![Problem::default(); n];

    for i in 0..n {
        problems[i].score = scan.token::<i64>();
    }

    for i in 0..n {
        problems[i].penalty = scan.token::<i64>();
    }

    for i in 0..n {
        problems[i].time = scan.token::<i64>();
    }

    // NOTE: Total Score
    // a -> b: a.score - (time + a.time) * a.penalty + b.score - (time + a.time + b.time) * b.penalty - (1)
    // b -> a: b.score - (time + b.time) * b.penalty + a.score - (time + b.time + a.time) * a.penalty - (2)
    // Therefore, (1) - (2) = a.time * b.penalty - b.time * a.penalty
    // If (1) - (2) > 0, then a should be solved before b
    problems.sort_by(|a, b| (a.time * b.penalty).cmp(&(b.time * a.penalty)));

    let mut max_values = vec![vec![-1; t as usize + 1]; n + 1];
    let ret = calculate(&problems, &mut max_values, n, t as usize, 0, 0);

    writeln!(out, "{ret}").unwrap();
}
