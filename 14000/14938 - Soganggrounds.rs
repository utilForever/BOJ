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

    let (n, m, r) = (
        scan.token::<usize>(),
        scan.token::<i32>(),
        scan.token::<usize>(),
    );
    let mut items = vec![0; n + 1];
    let mut vertex_info = vec![Vec::new(); n + 1];

    for i in 1..=n {
        items[i] = scan.token::<i32>();
    }

    for _ in 0..r {
        let (a, b, l) = (
            scan.token::<usize>(),
            scan.token::<usize>(),
            scan.token::<i32>(),
        );

        vertex_info[a].push((b, l));
        vertex_info[b].push((a, l));
    }

    let mut ret = 0;

    for i in 1..=n {
        let mut vertices = vec![i32::MAX; n + 1];
        vertices[i] = 0;

        let mut queue = BinaryHeap::new();
        queue.push((0, i));

        while !queue.is_empty() {
            let (mut cost, vertex) = queue.pop().unwrap();
            cost *= -1;

            for info in vertex_info[vertex].iter() {
                let (next_vertex, mut next_cost) = *info;
                next_cost += cost;

                if vertices[next_vertex] > next_cost && next_cost <= m {
                    vertices[next_vertex] = next_cost;
                    queue.push((-next_cost, next_vertex));
                }
            }
        }

        let mut sum_item = 0;

        for j in 1..=n {
            if vertices[j] != i32::MAX {
                sum_item += items[j];
            }
        }

        ret = ret.max(sum_item);
    }

    writeln!(out, "{}", ret).unwrap();
}
