use io::Write;
use std::{io, str};

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

fn lower_bound(v: &Vec<usize>, x: usize) -> usize {
    let mut left = 0;
    let mut right = v.len();

    while left < right {
        let mid = (left + right) >> 1;

        if v[mid] < x {
            left = mid + 1;
        } else {
            right = mid;
        }
    }

    left
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let t = scan.token::<i64>();

    for _ in 0..t {
        let (n, m) = (scan.token::<usize>(), scan.token::<usize>());
        let mut items = vec![0; n];
        let mut boxes = vec![0; m + 1];

        for i in 0..n {
            items[i] = scan.token::<usize>();
        }

        for i in 1..=m {
            boxes[i] = scan.token::<usize>();
        }

        let mut visited = vec![false; m + 1];
        let mut idxes = vec![usize::MAX; m + 1];
        let mut cycles = Vec::new();

        for i in 1..=m {
            if visited[i] {
                continue;
            }

            let mut curr = i;
            let mut nodes = Vec::new();

            while !visited[curr] {
                visited[curr] = true;
                nodes.push(curr);
                curr = boxes[curr];
            }

            nodes.sort_unstable();

            let idx = cycles.len();

            for &node in nodes.iter() {
                idxes[node] = idx;
            }

            cycles.push(nodes);
        }

        let mut prev = 0;
        let mut check = true;

        for item in items {
            let idx = idxes[item];
            let pos = lower_bound(&cycles[idx], prev);

            if pos == cycles[idx].len() {
                check = false;
                break;
            }

            prev = cycles[idx][pos];
        }

        writeln!(out, "{}", if check { "YES" } else { "NO" }).unwrap();
    }
}
