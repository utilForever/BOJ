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
    let mut times = vec![(0, 0); n];

    for i in 0..n {
        (times[i].0, times[i].1) = (scan.token::<usize>(), scan.token::<usize>());
    }

    times.sort();

    let mut priority_queue = BinaryHeap::new();
    let mut empty_seats = BinaryHeap::new();
    let mut ret = vec![0; 100_001];

    priority_queue.push(Reverse((times[0].1, 1)));
    ret[1] = 1;

    let mut num_seats = 1;

    for i in 1..n {
        while !priority_queue.is_empty() {
            let (finish, seat) = priority_queue.peek().unwrap().0;

            if finish <= times[i].0 {
                priority_queue.pop();
                empty_seats.push(Reverse(seat));
            } else {
                break;
            }
        }

        let idx = if empty_seats.is_empty() {
            num_seats += 1;
            num_seats
        } else {
            empty_seats.pop().unwrap().0
        };

        priority_queue.push(Reverse((times[i].1, idx)));
        ret[idx] += 1;
    }

    writeln!(out, "{num_seats}").unwrap();

    for i in 1..=num_seats {
        write!(out, "{} ", ret[i]).unwrap();
    }

    writeln!(out).unwrap();
}
