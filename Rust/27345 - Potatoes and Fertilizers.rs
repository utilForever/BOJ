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
    let mut segments = vec![(0, 0); n];

    for i in 0..n {
        segments[i] = (scan.token::<i64>(), scan.token::<i64>());
    }

    let mut queue = BinaryHeap::new();
    let mut diff = 0;
    let mut ret = 0;

    // Using slope trick
    for i in 0..n {
        let (fertiliser, potato) = segments[i];
        diff += fertiliser - potato;

        if diff < 0 {
            queue.push(0);
            queue.push(0);
        } else {
            queue.push(diff);
            queue.push(diff);
        }

        queue.pop();

        ret += diff.abs();
    }

    while !queue.is_empty() {
        ret -= diff.min(queue.pop().unwrap());
    }

    writeln!(out, "{ret}").unwrap();
}
