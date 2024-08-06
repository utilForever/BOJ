use io::Write;
use std::{cmp::Reverse, collections::BinaryHeap, io, str};

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

    let n = scan.token::<usize>();
    let mut sizes = vec![0; n];

    for i in 0..n {
        sizes[i] = scan.token::<i64>();
    }

    if n == 1 {
        writeln!(out, "{}", sizes[0]).unwrap();
        return;
    }

    if sizes.iter().all(|&x| x == sizes[0]) {
        writeln!(out, "-1").unwrap();
        return;
    }

    sizes.sort();

    // 0: from, 1: to
    let mut ret = vec![vec![i64::MAX; 2]; 1 << n];

    let mut queue = BinaryHeap::new();
    queue.push(Reverse((0, 0, 0)));

    // Dijkstra algorithm
    while !queue.is_empty() {
        let (cost_curr, pos_curr, vertex_curr) = queue.pop().unwrap().0;

        // Skip if the cost is not minimum
        if ret[vertex_curr][pos_curr] < cost_curr {
            continue;
        }

        if pos_curr == 0 {
            // from -> to
            for i in 0..n {
                for j in i + 1..n {
                    // Skip if i-th ship or j-th ship is already in the destination
                    if vertex_curr & (1 << i) != 0 || vertex_curr & (1 << j) != 0 {
                        continue;
                    }

                    // Skip if i-th ship and j-th ship have the same size
                    if sizes[i] == sizes[j] {
                        continue;
                    }

                    // Move i-th ship and j-th ship to the destination
                    let vertex_next = vertex_curr | (1 << i) | (1 << j);
                    let cost_next = cost_curr + sizes[j];

                    if ret[vertex_next][1] > cost_next {
                        ret[vertex_next][1] = cost_next;
                        queue.push(Reverse((cost_next, 1, vertex_next)));
                    }
                }
            }
        } else {
            // to <- from
            for i in 0..n {
                // Skip if i-th ship is not in the destination
                if vertex_curr & (1 << i) == 0 {
                    continue;
                }

                // Move i-th ship to the source
                let vertex_next = vertex_curr ^ (1 << i);
                let cost_next = cost_curr + sizes[i];

                if ret[vertex_next][0] > cost_next {
                    ret[vertex_next][0] = cost_next;
                    queue.push(Reverse((cost_next, 0, vertex_next)));
                }
            }
        }
    }

    writeln!(
        out,
        "{}",
        if ret[(1 << n) - 1][1] == i64::MAX {
            -1
        } else {
            ret[(1 << n) - 1][1]
        }
    )
    .unwrap();
}
