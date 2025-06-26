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

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let (n, m, k) = (
        scan.token::<usize>(),
        scan.token::<i64>(),
        scan.token::<i64>(),
    );
    let mut problems = vec![(0, 0); n];

    for i in 0..n {
        problems[i] = (scan.token::<i64>(), i + 1);
    }

    let min_submission = (k - m).max(0) as usize;

    if min_submission > n {
        writeln!(out, "-1").unwrap();
        return;
    }

    problems.sort_by_key(|problem| problem.0);

    let days_total = problems
        .iter()
        .take(min_submission)
        .map(|problem| problem.0)
        .sum::<i64>();

    if days_total > k {
        writeln!(out, "-1").unwrap();
        return;
    }

    let days_freeze = (k - days_total) as usize;
    let mut ret = vec![0; days_freeze];

    for &(days, idx) in problems[..min_submission].iter() {
        ret.extend(std::iter::repeat(0).take((days - 1) as usize));
        ret.push(idx);
    }

    for val in ret {
        write!(out, "{val} ").unwrap();
    }

    writeln!(out).unwrap();
}
