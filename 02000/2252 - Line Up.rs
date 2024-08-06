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

    let (n, m) = (scan.token(), scan.token());

    let mut vec = vec![Vec::new(); n + 1];
    let mut degree = vec![0; n + 1];

    for _ in 0..m {
        let (a, b): (usize, usize) = (scan.token(), scan.token());

        vec[a].push(b);
        degree[b] += 1;
    }

    let mut queue = VecDeque::new();

    for i in 1..=n {
        if degree[i] == 0 {
            queue.push_back(i);
        }
    }

    while !queue.is_empty() {
        let val = queue.pop_front().unwrap();
        write!(out, "{} ", val).unwrap();

        for node in vec[val].iter() {
            degree[*node] -= 1;

            if degree[*node] == 0 {
                queue.push_back(*node);
            }
        }
    }

    writeln!(out).unwrap();
}
