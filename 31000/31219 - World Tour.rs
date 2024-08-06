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
}

fn backtrack(
    points: &Vec<(f64, f64)>,
    visited: &mut Vec<bool>,
    ret: &mut f64,
    limit: usize,
    index: usize,
    count: usize,
    dist: f64,
) {
    if count == limit {
        *ret =
            ret.min(dist + ((points[index].0 - points[0].0).hypot(points[index].1 - points[0].1)));
        return;
    }

    for i in 0..limit {
        if visited[i] {
            continue;
        }

        visited[i] = true;
        backtrack(
            points,
            visited,
            ret,
            limit,
            i,
            count + 1,
            dist + ((points[index].0 - points[i].0).hypot(points[index].1 - points[i].1)),
        );
        visited[i] = false;
    }
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let n = scan.token::<usize>();
    let mut points = vec![(0.0, 0.0); n];
    let mut visited = vec![false; n];
    let mut ret = f64::MAX;

    for i in 0..n {
        points[i] = (scan.token::<f64>(), scan.token::<f64>());
    }

    if n == 2 {
        writeln!(out, "-1").unwrap();
        return;
    }

    visited[0] = true;

    backtrack(&points, &mut visited, &mut ret, n, 0, 1, 0.0);

    writeln!(out, "{:.6}", ret).unwrap();
}
