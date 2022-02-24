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

    let n = scan.token::<usize>();
    let mut max_heap: BinaryHeap<i32> = BinaryHeap::new();
    let mut min_heap: BinaryHeap<Reverse<i32>> = BinaryHeap::new();

    for _ in 0..n {
        let num = scan.token::<i32>();

        if max_heap.len() == min_heap.len() {
            max_heap.push(num);
        } else {
            min_heap.push(Reverse(num));
        }

        if !min_heap.is_empty()
            && !max_heap.is_empty()
            && min_heap.peek().unwrap().0 < *max_heap.peek().unwrap()
        {
            let max_val = max_heap.pop().unwrap();
            let min_val = min_heap.pop().unwrap().0;

            max_heap.push(min_val);
            min_heap.push(Reverse(max_val));
        }

        writeln!(out, "{}", max_heap.peek().unwrap()).ok();
    }
}
