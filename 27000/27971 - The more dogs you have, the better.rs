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

    let (n, m, a, b) = (
        scan.token::<usize>(),
        scan.token::<usize>(),
        scan.token::<i64>(),
        scan.token::<i64>(),
    );
    let mut ranges = vec![false; n + 1];

    for _ in 0..m {
        let (l, r) = (scan.token::<usize>(), scan.token::<usize>());

        for i in l..=r {
            ranges[i] = true;
        }
    }

    if ranges[n] {
        writeln!(out, "-1").unwrap();
        return;
    }

    let mut visited = vec![false; n + 1];
    let mut queue = VecDeque::new();

    visited[0] = true;
    queue.push_back((0, 0));

    while let Some((dog_curr, step_curr)) = queue.pop_front() {
        if dog_curr == n as i64 {
            writeln!(out, "{step_curr}").unwrap();
            return;
        }

        let dog_next = dog_curr + a;

        if dog_next <= n as i64 && !visited[dog_next as usize] && !ranges[dog_next as usize] {
            visited[dog_next as usize] = true;
            queue.push_back((dog_next, step_curr + 1));
        }

        let dog_next = dog_curr + b;

        if dog_next <= n as i64 && !visited[dog_next as usize] && !ranges[dog_next as usize] {
            visited[dog_next as usize] = true;
            queue.push_back((dog_next, step_curr + 1));
        }
    }

    writeln!(out, "-1").unwrap();
}
