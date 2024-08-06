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

    let (n, k) = (scan.token::<usize>(), scan.token::<usize>());
    let mut jewels = vec![(0, 0); n];
    let mut bags = vec![0; k];

    for i in 0..n {
        jewels[i] = (scan.token::<i64>(), scan.token::<i64>());
    }

    for i in 0..k {
        bags[i] = scan.token::<i64>();
    }

    jewels.sort();
    bags.sort();

    let mut priority_queue = BinaryHeap::new();
    let mut idx = 0;
    let mut ret = 0;

    for i in 0..k {
        while idx < n && jewels[idx].0 <= bags[i] {
            priority_queue.push(jewels[idx].1);
            idx += 1;
        }

        if !priority_queue.is_empty() {
            ret += priority_queue.pop().unwrap();
        }
    }

    writeln!(out, "{}", ret).unwrap();
}
