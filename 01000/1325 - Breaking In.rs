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

    let (n, m) = (scan.token::<usize>(), scan.token::<i64>());
    let mut graph = vec![Vec::new(); n + 1];

    for _ in 0..m {
        let (a, b) = (scan.token::<usize>(), scan.token::<usize>());
        graph[b].push(a);
    }

    let mut ret = Vec::new();
    let mut max = 0;

    for i in 1..=n {
        let mut queue = VecDeque::new();
        let mut visited = vec![false; n + 1];
        let mut cnt = 0;

        queue.push_back(i);
        visited[i] = true;

        while !queue.is_empty() {
            let node = queue.pop_front().unwrap();
            cnt += 1;

            for &next in &graph[node] {
                if visited[next] {
                    continue;
                }

                visited[next] = true;
                queue.push_back(next);
            }
        }

        if cnt > max {
            max = cnt;
            ret.clear();
            ret.push(i);
        } else if cnt == max {
            ret.push(i);
        }
    }

    for val in ret {
        write!(out, "{val} ").unwrap();
    }

    writeln!(out).unwrap();
}
