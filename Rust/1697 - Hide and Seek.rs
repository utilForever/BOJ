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

    let (n, k) = (scan.token::<usize>(), scan.token::<usize>());
    let mut positions = vec![0; 100001];

    let mut queue = VecDeque::new();
    queue.push_back(n);

    while !queue.is_empty() {
        let curr = queue.pop_front().unwrap();

        if curr == k {
            break;
        }

        if curr as i64 - 1 >= 0 && positions[curr - 1] == 0 {
            queue.push_back(curr - 1);
            positions[curr - 1] = positions[curr] + 1;
        }

        if curr + 1 <= 100_000 && positions[curr + 1] == 0 {
            queue.push_back(curr + 1);
            positions[curr + 1] = positions[curr] + 1;
        }

        if curr * 2 <= 100_000 && positions[curr * 2] == 0 {
            queue.push_back(curr * 2);
            positions[curr * 2] = positions[curr] + 1;
        }
    }

    writeln!(out, "{}", positions[k]).unwrap();
}
