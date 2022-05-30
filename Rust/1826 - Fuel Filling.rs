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
    let mut gas_stations = vec![(0, 0); n];
    let mut idx_stations = 0;

    for i in 0..n {
        (gas_stations[i].0, gas_stations[i].1) = (scan.token::<usize>(), scan.token::<usize>());
    }

    gas_stations.sort();

    let (l, mut p) = (scan.token::<usize>(), scan.token::<usize>());
    let mut priority_queue = BinaryHeap::new();
    let mut ret = 0;

    while p < l {
        while idx_stations < n && gas_stations[idx_stations].0 <= p {
            priority_queue.push(gas_stations[idx_stations].1);
            idx_stations += 1;
        }

        if priority_queue.is_empty() {
            ret = -1;
            break;
        }

        p += priority_queue.pop().unwrap();
        ret += 1;
    }

    writeln!(out, "{}", ret).unwrap();
}
