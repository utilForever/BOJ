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

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let k = scan.token::<i64>();

    for _ in 0..k {
        let (v, e) = (scan.token::<usize>(), scan.token::<i64>());
        let mut graph = vec![Vec::new(); v + 1];

        for _ in 0..e {
            let (u, v) = (scan.token::<usize>(), scan.token::<usize>());
            graph[u].push(v);
            graph[v].push(u);
        }

        let mut colors = vec![Color::None; v + 1];
        let mut ret = true;

        for i in 1..=v {
            if colors[i] != Color::None {
                continue;
            }

            let mut queue = VecDeque::new();

            colors[i] = Color::Red;
            queue.push_back(i);

            while !queue.is_empty() {
                let curr = queue.pop_front().unwrap();

                for &next in graph[curr].iter() {
                    if colors[next] == Color::None {
                        colors[next] = match colors[curr] {
                            Color::Red => Color::Black,
                            Color::Black => Color::Red,
                            Color::None => unreachable!(),
                        };
                        queue.push_back(next);
                    } else if colors[next] == colors[curr] {
                        ret = false;
                        break;
                    } else {
                        continue;
                    }
                }
            }

            if !ret {
                break;
            }
        }

        writeln!(out, "{}", if ret { "YES" } else { "NO" }).unwrap();
    }
}
