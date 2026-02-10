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

    pub fn all(&mut self) -> String {
        let mut input = String::new();
        self.reader.read_to_string(&mut input).expect("Failed read");
        input
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

    let (n, k) = (scan.token::<usize>(), scan.token::<usize>());

    if n == 1 {
        writeln!(out, "1").unwrap();
        return;
    }

    let mut head = vec![-1; n];
    let mut next = vec![-1; n - 1];
    let mut to = vec![0; n - 1];

    for i in 1..n {
        let p = scan.token::<usize>();

        to[i - 1] = i;
        next[i - 1] = head[p];
        head[p] = i as i64 - 1;
    }

    let mut height = vec![1; n];
    let mut heavy = vec![usize::MAX; n];

    for i in (0..n).rev() {
        let mut idx = head[i];
        let mut best_height = 0;
        let mut best_child = usize::MAX;

        while idx != -1 {
            let child = to[idx as usize] as usize;
            let height = height[child];

            if height > best_height {
                best_height = height;
                best_child = child;
            }

            idx = next[idx as usize];
        }

        if best_child != usize::MAX {
            height[i] = best_height + 1;
            heavy[i] = best_child;
        }
    }

    let mut pq = BinaryHeap::new();
    pq.push((height[0], 0));

    let mut sum = 0;
    let mut ret = 0;

    while sum < (n - k) as i64 {
        let (h, v) = pq.pop().unwrap();

        sum += h;
        ret += 1;

        let mut curr = v;

        loop {
            let mut idx = head[curr];

            while idx != -1 {
                let child = to[idx as usize];

                if child != heavy[curr] {
                    pq.push((height[child], child as usize));
                }

                idx = next[idx as usize];
            }

            if heavy[curr] == usize::MAX {
                break;
            }

            curr = heavy[curr];
        }
    }

    writeln!(out, "{ret}").unwrap();
}
