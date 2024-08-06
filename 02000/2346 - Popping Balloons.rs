use io::Write;
use std::{collections::VecDeque, io, str};

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
    let mut deque = VecDeque::new();

    for i in 1..=n {
        deque.push_back((i, scan.token::<i64>()));
    }

    loop {
        let (num, offset) = deque.pop_front().unwrap();
        write!(out, "{num} ").unwrap();

        if deque.is_empty() {
            break;
        }

        if offset > 0 {
            for _ in 1..offset {
                let val = deque.pop_front().unwrap();
                deque.push_back(val);
            }
        } else {
            for _ in 0..offset.abs() {
                let val = deque.pop_back().unwrap();
                deque.push_front(val);
            }
        }
    }

    writeln!(out).unwrap();
}
