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
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let (n, m) = (scan.token::<i64>(), scan.token::<i64>());
    let mut priority_queue = BinaryHeap::new();

    for _ in 0..n {
        priority_queue.push(Reverse(scan.token::<i64>()));
    }

    for _ in 0..m {
        let val1 = priority_queue.pop().unwrap().0;
        let val2 = priority_queue.pop().unwrap().0;

        priority_queue.push(Reverse(val1 + val2));
        priority_queue.push(Reverse(val1 + val2));
    }

    let mut ret = 0;

    while !priority_queue.is_empty() {
        ret += priority_queue.pop().unwrap().0;
    }

    writeln!(out, "{ret}").unwrap();
}
