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

fn get_dist(p1: (i64, i64), p2: (i64, i64)) -> i64 {
    (p1.0 - p2.0).pow(2) + (p1.1 - p2.1).pow(2)
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let n = scan.token::<usize>();
    let mut points = vec![(0, 0); n];

    for i in 0..n {
        (points[i].0, points[i].1) = (scan.token::<i64>(), scan.token::<i64>());
    }

    let mut len = -1;
    let mut pivot = (0, 0);

    for i in 0..n - 1 {
        for j in i + 1..n {
            let dist = get_dist(points[i], points[j]);
            if dist > len {
                len = dist;
                pivot = points[i];
            }
        }
    }

    points.sort_by(|a, b| {
        let dist_a = get_dist(pivot, *a);
        let dist_b = get_dist(pivot, *b);

        dist_a.cmp(&dist_b)
    });

    let mut idxes = vec![0; n];
    let mut val_max = 0;
    let mut ret = f64::MAX;

    for i in (0..=n - 2).rev() {
        idxes[i] = idxes[i + 1];

        for j in i + 1..n {
            idxes[i] = idxes[i].max(get_dist(points[i], points[j]));
        }
    }

    for i in 0..n - 1 {
        for j in 0..i {
            val_max = val_max.max(get_dist(points[i], points[j]));
        }

        ret = ret.min((val_max as f64).sqrt() + (idxes[i + 1] as f64).sqrt());
    }

    writeln!(out, "{:.10}", ret).unwrap();
}
