use io::Write;
use std::{collections::VecDeque, io, str, vec};

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

    let p = scan.token::<usize>();
    let mut graph = vec![Vec::new(); p];

    for _ in 0..p - 1 {
        let (c, d1, d2) = (
            scan.token::<usize>(),
            scan.token::<usize>(),
            scan.token::<usize>(),
        );

        if d1 != 0 {
            graph[c].push(d1);
            graph[d1].push(c);
        }

        if d2 != 0 {
            graph[c].push(d2);
            graph[d2].push(c);
        }
    }

    let mut queue = VecDeque::new();
    let mut visited = vec![false; p];
    let mut ret = 0;

    queue.push_back((1, 1));
    visited[1] = true;

    while !queue.is_empty() {
        let (node, depth) = queue.pop_front().unwrap();

        ret = ret.max(depth);

        for &next in graph[node].iter() {
            if !visited[next] {
                visited[next] = true;
                queue.push_back((next, depth + 1));
            }
        }
    }

    writeln!(out, "{ret}").unwrap();
}
