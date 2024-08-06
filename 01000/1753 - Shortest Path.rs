use io::Write;
use std::{collections::BinaryHeap, io, str};

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

    let (v, e) = (scan.token::<usize>(), scan.token::<usize>());
    let k = scan.token::<usize>();
    let mut vertices = vec![i32::MAX; v + 1];
    let mut vertex_info = vec![Vec::new(); v + 1];

    for _ in 0..e {
        let (u, v, w) = (
            scan.token::<usize>(),
            scan.token::<usize>(),
            scan.token::<i32>(),
        );
        vertex_info[u].push((v, w));
    }

    vertices[k] = 0;

    let mut queue = BinaryHeap::new();
    queue.push((0, k));

    while !queue.is_empty() {
        let (mut cost, vertex) = queue.pop().unwrap();
        cost *= -1;

        for info in vertex_info[vertex].iter() {
            let (next_vertex, mut next_cost) = *info;
            next_cost += cost;

            if vertices[next_vertex] > next_cost {
                vertices[next_vertex] = next_cost;
                queue.push((-next_cost, next_vertex));
            }
        }
    }

    for i in 1..=v {
        if vertices[i] == i32::MAX {
            writeln!(out, "INF").unwrap();
        } else {
            writeln!(out, "{}", vertices[i]).unwrap();
        }
    }
}
