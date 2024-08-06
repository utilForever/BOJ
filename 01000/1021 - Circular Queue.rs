use io::Write;
use std::{io, str, collections::VecDeque};

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

    let (n, m) = (scan.token::<i64>(), scan.token::<usize>());
    let mut idxes = vec![0; m];
    let mut deque = VecDeque::new();

    for i in 0..m {
        idxes[i] = scan.token::<i64>();
    }

    for i in 1..=n {
        deque.push_back(i);
    }

    let mut ret = 0;

    for idx in idxes {
        let mut pos = 0;

        for i in 0..deque.len() {
            if deque[i] == idx {
                pos = i;
                break;
            }
        }

        if pos <= deque.len() / 2 {
            while deque.front().unwrap() != &idx {
                let tmp = deque.pop_front().unwrap();
                deque.push_back(tmp);
                ret += 1;
            }
        } else {
            while deque.front().unwrap() != &idx {
                let tmp = deque.pop_back().unwrap();
                deque.push_front(tmp);
                ret += 1;
            }
        }

        deque.pop_front();
    }

    writeln!(out, "{ret}").unwrap();
}
