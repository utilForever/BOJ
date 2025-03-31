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

    let (n, m) = (scan.token::<usize>(), scan.token::<usize>());
    let mut difficulties = vec![(0, 0, 0); n];

    for i in 0..n {
        difficulties[i] = (scan.token::<i64>(), scan.token::<i64>(), i);
    }

    difficulties.sort_by(|a, b| a.0.cmp(&b.0));

    let mut priority_queue = BinaryHeap::new();
    let mut idx = 0;
    let mut val = 0;
    let mut section = 0;
    let mut ret = vec![(0, 0); n];

    while idx < n || !priority_queue.is_empty() {
        if priority_queue.is_empty() {
            val = difficulties[idx].0;
        }

        while idx < n && difficulties[idx].0 <= val {
            priority_queue.push((-difficulties[idx].1, difficulties[idx].2));
            idx += 1;
        }

        section += 1;

        loop {
            let (_, s) = priority_queue.pop().unwrap();
            ret[s] = (val, section);

            if priority_queue.is_empty() || priority_queue.peek().unwrap().0 != -val {
                break;
            }
        }

        val += 1;
    }

    if section < m {
        writeln!(out, "-1").unwrap();
    } else {
        for i in 0..n {
            writeln!(out, "{} {}", ret[i].0, ret[i].1.min(m)).unwrap();
        }
    }
}
