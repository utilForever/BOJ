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

    let n = scan.token::<i64>();

    for i in 1..=n {
        let t = scan.token::<i64>();
        let (na, nb) = (scan.token::<usize>(), scan.token::<usize>());
        let mut trains = vec![(0, 0, 0); na + nb];

        for j in 0..na {
            let (start, end) = (scan.token::<String>(), scan.token::<String>());
            let start = start
                .split(':')
                .map(|x| x.parse::<i64>().unwrap())
                .collect::<Vec<i64>>();
            let end = end
                .split(':')
                .map(|x| x.parse::<i64>().unwrap())
                .collect::<Vec<i64>>();

            let time_start = start[0] * 60 + start[1];
            let time_end = end[0] * 60 + end[1] + t;

            trains[j] = (time_start, time_end, 0);
        }

        for j in 0..nb {
            let (start, end) = (scan.token::<String>(), scan.token::<String>());
            let start = start
                .split(':')
                .map(|x| x.parse::<i64>().unwrap())
                .collect::<Vec<i64>>();
            let end = end
                .split(':')
                .map(|x| x.parse::<i64>().unwrap())
                .collect::<Vec<i64>>();

            let time_start = start[0] * 60 + start[1];
            let time_end = end[0] * 60 + end[1] + t;

            trains[na + j] = (time_start, time_end, 1);
        }

        trains.sort();

        let mut priority_queue_a: BinaryHeap<Reverse<(i64, i32)>> = BinaryHeap::new();
        let mut priority_queue_b: BinaryHeap<Reverse<(i64, i32)>> = BinaryHeap::new();
        let mut trains_ready_a = BinaryHeap::new();
        let mut trains_ready_b = BinaryHeap::new();
        let mut ret_a = 0;
        let mut ret_b = 0;

        for i in 0..na + nb {
            if trains[i].2 == 0 {
                while !priority_queue_a.is_empty() {
                    let (finish, num) = priority_queue_a.peek().unwrap().0;

                    if finish <= trains[i].0 {
                        priority_queue_a.pop();
                        trains_ready_a.push(Reverse(num));
                    } else {
                        break;
                    }
                }

                let idx = if trains_ready_a.is_empty() {
                    ret_a += 1;
                    ret_a
                } else {
                    trains_ready_a.pop().unwrap().0
                };

                priority_queue_b.push(Reverse((trains[i].1, idx)));
            } else {
                while !priority_queue_b.is_empty() {
                    let (finish, num) = priority_queue_b.peek().unwrap().0;

                    if finish <= trains[i].0 {
                        priority_queue_b.pop();
                        trains_ready_b.push(Reverse(num));
                    } else {
                        break;
                    }
                }

                let idx = if trains_ready_b.is_empty() {
                    ret_b += 1;
                    ret_b
                } else {
                    trains_ready_b.pop().unwrap().0
                };

                priority_queue_a.push(Reverse((trains[i].1, idx)));
            }
        }

        writeln!(out, "Case #{i}: {ret_a} {ret_b}").unwrap();
    }
}
