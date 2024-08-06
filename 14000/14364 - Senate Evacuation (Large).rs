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

    let t = scan.token::<i64>();

    for i in 1..=t {
        let n = scan.token::<usize>();
        let mut priority_queue = BinaryHeap::new();
        let mut sum = 0;

        for j in 0..n {
            let senator = scan.token::<i64>();

            priority_queue.push((senator, ('A' as u8 + j as u8) as char));
            sum += senator;
        }

        write!(out, "Case #{i}: ").unwrap();

        while !priority_queue.is_empty() {
            let (senator, party) = priority_queue.pop().unwrap();
            write!(out, "{party}").unwrap();

            if senator > 1 {
                priority_queue.push((senator - 1, party));
            }

            sum -= 1;

            if !priority_queue.is_empty() && priority_queue.peek().unwrap().0 * 2 > sum {
                let (senator, party) = priority_queue.pop().unwrap();
                write!(out, "{party} ").unwrap();

                if senator > 1 {
                    priority_queue.push((senator - 1, party));
                }

                sum -= 1;
            } else {
                write!(out, " ").unwrap();
            }
        }

        writeln!(out).unwrap();
    }
}
