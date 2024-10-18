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
    let mut gyms = vec![(0, 0); n];

    for i in 0..n {
        gyms[i].1 = scan.token::<i64>();
    }

    for i in 0..n {
        gyms[i].0 = scan.token::<i64>();
    }

    gyms.sort_by_key(|&(cap, gain)| cap + gain);

    let mut priority_queue = BinaryHeap::new();
    let mut num_badges = 0;

    for (cap, gain) in gyms {
        num_badges += gain;

        priority_queue.push(gain);

        if num_badges > cap + gain {
            num_badges -= priority_queue.pop().unwrap();
        }
    }

    writeln!(out, "{}", priority_queue.len()).unwrap();
}
