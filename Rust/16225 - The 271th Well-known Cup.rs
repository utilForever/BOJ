use io::Write;
use std::{collections::BinaryHeap, io, str};

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

    let n = scan.token::<usize>();
    let mut problems = vec![(0, 0); n];

    for i in 0..n {
        problems[i].0 = scan.token::<usize>();
    }
    for i in 0..n {
        problems[i].1 = scan.token::<usize>();
    }

    problems.sort_by(|a, b| a.1.cmp(&b.1));

    let mut priority_queue = BinaryHeap::new();
    let mut ret = problems[0].0;

    for i in (1..n - 1).step_by(2) {
        priority_queue.push(problems[i].0);
        priority_queue.push(problems[i + 1].0);
        ret += priority_queue.pop().unwrap();
    }

    writeln!(out, "{}", ret).unwrap();
}
