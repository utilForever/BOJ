use io::Write;
use std::{collections::BinaryHeap, io, str, cmp::Reverse};

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

    let (n, l) = (scan.token::<usize>(), scan.token::<i64>());
    let mut arr = vec![(0, 0); n];

    for i in 0..n {
        let a = scan.token::<i64>();
        arr[i] = (a, i as i64);
    }

    let mut priority_queue = BinaryHeap::new();

    for i in 0..n {
        priority_queue.push(Reverse((arr[i].0, arr[i].1)));

        while i as i64 - l + 1 >= 0 && priority_queue.peek().unwrap().0.1 < i as i64 - l + 1 {
            priority_queue.pop();
        }

        write!(out, "{} ", priority_queue.peek().unwrap().0.0).unwrap();
    }

    writeln!(out).unwrap();
}
