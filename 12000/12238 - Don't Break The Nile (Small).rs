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

fn gap(a1: i64, a2: i64, b1: i64, b2: i64) -> i64 {
    ((a1 - b2).max(b1 - a2) - 1).max(0)
}

fn dist(a: &(i64, i64, i64, i64), b: &(i64, i64, i64, i64)) -> i64 {
    let dx = gap(a.0, a.2, b.0, b.2);
    let dy = gap(a.1, a.3, b.1, b.3);
    dx.max(dy)
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let t = scan.token::<i64>();

    for i in 1..=t {
        let (w, h, b) = (
            scan.token::<i64>(),
            scan.token::<i64>(),
            scan.token::<usize>(),
        );
        let mut rectangles = vec![(0, 0, 0, 0); b];

        for j in 0..b {
            let (x0, y0, x1, y1) = (
                scan.token::<i64>(),
                scan.token::<i64>(),
                scan.token::<i64>(),
                scan.token::<i64>(),
            );

            rectangles[j] = (x0, y0, x1, y1);
        }

        let idx_left = b;
        let idx_right = b + 1;

        rectangles.push((-1, 0, -1, h - 1));
        rectangles.push((w, 0, w, h - 1));

        let n = rectangles.len();
        let mut dists = vec![0; n * n];

        for j in 0..n {
            for k in j + 1..n {
                let d = dist(&rectangles[j], &rectangles[k]);
                dists[j * n + k] = d;
                dists[k * n + j] = d;
            }
        }

        let mut ret = vec![i64::MAX / 4; n];
        let mut heap = BinaryHeap::<Reverse<(i64, usize)>>::new();

        ret[idx_left] = 0;
        heap.push(Reverse((0, idx_left)));

        while let Some(Reverse((d, u))) = heap.pop() {
            if d != ret[u] {
                continue;
            }

            if u == idx_right {
                break;
            }

            for v in 0..n {
                let d = dists[u * n + v];

                if ret[v] > ret[u] + d {
                    ret[v] = ret[u] + d;
                    heap.push(Reverse((ret[v], v)));
                }
            }
        }

        writeln!(out, "Case #{i}: {}", ret[idx_right]).unwrap();
    }
}
