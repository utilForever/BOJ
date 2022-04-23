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

    let (n, k) = (scan.token::<usize>(), scan.token::<usize>());
    let mut arr = vec![0; n];

    for i in 0..n {
        arr[i] = scan.token::<i64>();
    }

    let mut queue: BinaryHeap<Reverse<(i64, usize)>> = BinaryHeap::new();
    let mut dist = vec![0; n + 1];
    let mut left = vec![0; n + 1];
    let mut right = vec![0; n + 1];

    queue.push(Reverse((3_210_210_210, 0)));
    queue.push(Reverse((3_210_210_210, n)));
    dist[0] = 3_210_210_210;
    dist[n] = 3_210_210_210;
    left[n] = n - 1;
    right[0] = 1;

    for i in 1..n {
        dist[i] = arr[i] - arr[i - 1];
        left[i] = i - 1;
        right[i] = i + 1;

        queue.push(Reverse((dist[i], i)));
    }

    let mut visited = vec![false; n + 1];
    let mut ret = 0;

    for _ in 0..k {
        loop {
            let (_, i) = queue.peek().unwrap().0;

            if !visited[i] {
                break;
            }

            queue.pop();
        }

        let (d, i) = queue.pop().unwrap().0;
        ret += d as u64;

        dist[i] = dist[left[i]] + dist[right[i]] - dist[i];
        queue.push(Reverse((dist[i], i)));

        visited[left[i]] = true;
        visited[right[i]] = true;

        left[i] = left[left[i]];
        right[i] = right[right[i]];
        left[right[i]] = i;
        right[left[i]] = i;
    }

    writeln!(out, "{}", ret).unwrap();
}
