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
    let mut queue = VecDeque::new();
    let mut visited = vec![vec![-1; 2]; 500_001];
    queue.push_back((n, 0));

    while !queue.is_empty() {
        let (curr_pos, curr_time) = queue.pop_front().unwrap();

        if visited[curr_pos][curr_time % 2] != -1 {
            continue;
        }

        visited[curr_pos][curr_time % 2] = curr_time as i64;

        if curr_pos * 2 <= 500_000 {
            queue.push_back((curr_pos * 2, curr_time + 1));
        }

        if curr_pos > 0 {
            queue.push_back((curr_pos - 1, curr_time + 1));
        }

        if curr_pos < 500_000 {
            queue.push_back((curr_pos + 1, curr_time + 1));
        }
    }

    for t in 0..500000 {
        let pos_younger = k + t * (t + 1) / 2;

        if pos_younger > 500_000 {
            break;
        }

        if visited[pos_younger][t % 2] != -1 && visited[pos_younger][t % 2] <= t as i64 {
            writeln!(out, "{t}").unwrap();
            return;
        }
    }

    writeln!(out, "-1").unwrap();
}
