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

#[derive(Clone, Copy, PartialEq, Eq)]
enum Color {
    Red,
    Black,
    None,
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let n = scan.token::<usize>();
    let mut roads = vec![0; n + 1];
    let mut graph = vec![Vec::new(); n + 1];
    let mut ret = n;

    for i in 1..=n {
        let a = scan.token::<usize>();

        roads[i] = a;
        graph[a].push(i);
        graph[i].push(a);
    }

    // Check a <-> b (only 2)
    for i in 1..=n {
        if roads[i] > i && roads[roads[i]] == i {
            ret -= 1;
        }
    }

    // Process DFS
    let mut colors = vec![Color::None; n + 1];

    for i in 1..=n {
        if colors[i] != Color::None {
            continue;
        }

        let mut queue = VecDeque::new();
        queue.push_back((i, Color::Red));

        let mut has_cycle = false;

        while !queue.is_empty() {
            let (curr, color) = queue.pop_front().unwrap();

            if colors[curr] != Color::None {
                if colors[curr] != color {
                    has_cycle = true;
                }

                continue;
            }

            colors[curr] = color;

            for &next in graph[curr].iter() {
                queue.push_back((
                    next,
                    if color == Color::Red {
                        Color::Black
                    } else {
                        Color::Red
                    },
                ));
            }
        }

        if has_cycle {
            ret -= 1;
        }
    }

    writeln!(out, "{ret}").unwrap();
}
