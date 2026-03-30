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

    let mut dimensions = [0; 11];

    for i in 0..11 {
        dimensions[i] = scan.token::<usize>();
    }

    let size = dimensions.iter().product::<usize>();
    let mut nums = vec![0; size];

    for i in 0..size {
        nums[i] = scan.token::<i64>();
    }

    let mut stride = [1; 11];

    for i in (0..10).rev() {
        stride[i] = stride[i + 1] * dimensions[i + 1];
    }

    let mut visited = vec![false; size];
    let mut queue = VecDeque::new();
    let mut ret = 1;

    visited[0] = true;
    queue.push_back(0);

    while let Some(idx) = queue.pop_front() {
        for d in 0..11 {
            let coord = (idx / stride[d]) % dimensions[d];

            if coord > 0 {
                let next = idx - stride[d];

                if !visited[next] && nums[next] != nums[idx] {
                    visited[next] = true;
                    queue.push_back(next);
                    ret += 1;
                }
            }

            if coord + 1 < dimensions[d] {
                let next = idx + stride[d];

                if !visited[next] && nums[next] != nums[idx] {
                    visited[next] = true;
                    queue.push_back(next);
                    ret += 1;
                }
            }
        }
    }

    writeln!(out, "{}", if ret == size { "Yes" } else { "No" }).unwrap();
}
