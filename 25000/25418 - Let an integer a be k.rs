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

    let (a, k) = (scan.token::<usize>(), scan.token::<usize>());
    let mut queue = VecDeque::new();
    let mut visited = vec![false; k + 1];
    let mut ret = i64::MAX;

    queue.push_back((a, 0));
    visited[a] = true;

    while let Some((val, cnt)) = queue.pop_front() {
        if val == k {
            ret = ret.min(cnt);
            break;
        }

        if val * 2 <= k && !visited[val * 2] {
            queue.push_back((val * 2, cnt + 1));
            visited[val * 2] = true;
        }

        if val + 1 <= k && !visited[val + 1] {
            queue.push_back((val + 1, cnt + 1));
            visited[val + 1] = true;
        }
    }

    writeln!(out, "{ret}").unwrap();
}
