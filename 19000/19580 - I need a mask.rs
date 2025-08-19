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

    let (n, m) = (scan.token::<usize>(), scan.token::<usize>());
    let mut citizens = vec![(0, 0); n];

    for i in 0..n {
        citizens[i] = (scan.token::<i64>(), scan.token::<i64>());
    }

    citizens.sort_unstable_by(|a, b| a.0.cmp(&b.0));

    let mut stores = vec![(0, 0); m];

    for i in 0..m {
        stores[i] = (scan.token::<i64>(), scan.token::<i64>());
    }

    stores.sort_unstable_by(|a, b| a.0.cmp(&b.0));

    let mut priority_queue = BinaryHeap::new();
    let mut idx_citizen = 0;
    let mut idx_store = 0;
    let mut ret = 0;

    while idx_store < stores.len() {
        let price = stores[idx_store].0;
        let mut capacity = 0;

        while idx_store < stores.len() && stores[idx_store].0 == price {
            capacity += stores[idx_store].1;
            idx_store += 1;
        }

        while idx_citizen < citizens.len() && citizens[idx_citizen].0 <= price {
            priority_queue.push(Reverse(citizens[idx_citizen].1));
            idx_citizen += 1;
        }

        while let Some(&Reverse(r)) = priority_queue.peek() {
            if r < price {
                priority_queue.pop();
            } else {
                break;
            }
        }

        if priority_queue.is_empty() || capacity <= 0 {
            continue;
        }

        let max = capacity.min(priority_queue.len() as i64);
        let mut cnt = 0;

        while cnt < max {
            priority_queue.pop();
            cnt += 1;
            ret += 1;
        }
    }

    writeln!(out, "{ret}").unwrap();
}
