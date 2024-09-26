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

    pub fn line(&mut self) -> String {
        let mut input = String::new();
        self.reader.read_line(&mut input).expect("Failed read");
        input
    }
}

fn find(parent: &mut Vec<usize>, node: usize) -> usize {
    if parent[node] == node {
        node
    } else {
        parent[node] = find(parent, parent[node]);
        parent[node]
    }
}

fn process_union(parent: &mut Vec<usize>, mut a: usize, mut b: usize) {
    a = find(parent, a);
    b = find(parent, b);
    parent[a] = b;
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let n = scan.token::<usize>();
    let mut rectangles = vec![(0, 0, 0, 0); n];
    let mut ret = 0;

    for i in 0..n {
        rectangles[i] = (
            scan.token::<i64>(),
            scan.token::<i64>(),
            scan.token::<i64>(),
            scan.token::<i64>(),
        );
    }

    let mut parent = vec![0; n];

    for i in 0..n {
        parent[i] = i;
    }

    let is_overlap = |(x1, y1, x2, y2): (i64, i64, i64, i64),
                      (x3, y3, x4, y4): (i64, i64, i64, i64)| {
        // Check if the rectangles are not overlapping
        if x2 < x3 || x4 < x1 || y2 < y3 || y4 < y1 {
            return false;
        }

        // Check if one rectangle is inside the other (Case 1)
        if x3 < x1 && x4 > x2 && y3 < y1 && y4 > y2 {
            return false;
        }

        // Check if one rectangle is inside the other (Case 2)
        if x1 < x3 && x2 > x4 && y1 < y3 && y2 > y4 {
            return false;
        }

        true
    };

    for i in 0..n - 1 {
        for j in i + 1..n {
            if is_overlap(rectangles[i], rectangles[j]) {
                process_union(&mut parent, i, j);
            }
        }
    }

    let mut visited = vec![false; n];

    for i in 0..n {
        let p = find(&mut parent, i);

        if !visited[p] {
            visited[p] = true;
            ret += 1;
        }
    }

    for i in 0..n {
        if is_overlap(rectangles[i], (0, 0, 0, 0)) {
            ret -= 1;
            break;
        }
    }

    writeln!(out, "{ret}").unwrap();
}
