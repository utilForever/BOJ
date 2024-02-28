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

#[derive(Clone, PartialEq, Eq)]
enum Color {
    Red,
    Black,
    None,
}

fn is_bipartite(graph: &Vec<Vec<usize>>, n: usize) -> bool {
    let mut queue = VecDeque::new();
    let mut colors = vec![Color::None; n];

    for i in 0..n {
        if colors[i] != Color::None {
            continue;
        }

        queue.push_back(i);
        colors[i] = Color::Red;

        while !queue.is_empty() {
            let curr = queue.pop_front().unwrap();

            for &next in &graph[curr] {
                if colors[next] == Color::None {
                    colors[next] = if colors[curr] == Color::Red {
                        Color::Black
                    } else {
                        Color::Red
                    };

                    queue.push_back(next);
                } else if colors[next] == colors[curr] {
                    return false;
                }
            }
        }
    }

    true
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let n = scan.token::<usize>();
    let mut teamworks = vec![vec![0; n]; n];

    for i in 0..n {
        for j in 0..n {
            teamworks[i][j] = scan.token::<i64>();
        }
    }

    let mut left = 0;
    let mut right = 1_000_001;

    while left + 1 < right {
        let mid = (left + right) / 2;
        let mut graph = vec![Vec::new(); n + 1];

        for i in 0..n {
            for j in i + 1..n {
                if teamworks[i][j] < mid {
                    graph[i].push(j);
                    graph[j].push(i);
                }
            }
        }

        if is_bipartite(&graph, n) {
            left = mid;
        } else {
            right = mid;
        }
    }

    writeln!(out, "{left}").unwrap();
}
