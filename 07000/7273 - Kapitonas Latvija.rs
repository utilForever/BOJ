use io::Write;
use std::{cmp::Ordering, collections::HashMap, hash::Hash, io, str};

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

#[derive(PartialEq)]
struct MinNonNan(f64);

impl Eq for MinNonNan {}

impl PartialOrd for MinNonNan {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.0.partial_cmp(&other.0)
    }
}

impl Ord for MinNonNan {
    fn cmp(&self, other: &MinNonNan) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl Hash for MinNonNan {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.0.to_bits().hash(state);
    }
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let (l, n) = (scan.token::<i64>(), scan.token::<usize>());
    let x = scan.token::<i64>();
    let mut points = vec![(0, 0); n];

    for i in 0..n {
        points[i] = (scan.token::<i64>(), scan.token::<i64>());
    }

    let mut directions = HashMap::new();

    for i in 0..n {
        directions
            .entry(MinNonNan(
                points[i].1 as f64 / (2 * l - x - points[i].0) as f64,
            ))
            .and_modify(|v| *v += 1)
            .or_insert(1);

        if points[i].0 > x {
            directions
                .entry(MinNonNan(points[i].1 as f64 / (points[i].0 - x) as f64))
                .and_modify(|v| *v += 1)
                .or_insert(1);
        }

        if points[i].0 < x {
            directions
                .entry(MinNonNan(
                    (points[i].1 * x) as f64 / ((points[i].0 - x) * (x - 2 * l)) as f64,
                ))
                .and_modify(|v| *v += 1)
                .or_insert(1);
        }
    }

    let mut ret = 0;

    for (_, v) in directions {
        ret = ret.max(v);
    }

    let x = l - x;

    for i in 0..n {
        points[i].0 = l - points[i].0;
    }

    let mut directions = HashMap::new();

    for i in 0..n {
        directions
            .entry(MinNonNan(
                points[i].1 as f64 / (2 * l - x - points[i].0) as f64,
            ))
            .and_modify(|v| *v += 1)
            .or_insert(1);

        if points[i].0 > x {
            directions
                .entry(MinNonNan(points[i].1 as f64 / (points[i].0 - x) as f64))
                .and_modify(|v| *v += 1)
                .or_insert(1);
        }

        if points[i].0 < x {
            directions
                .entry(MinNonNan(
                    (points[i].1 * x) as f64 / ((points[i].0 - x) * (x - 2 * l)) as f64,
                ))
                .and_modify(|v| *v += 1)
                .or_insert(1);
        }
    }

    for (_, v) in directions {
        ret = ret.max(v);
    }

    writeln!(out, "{}", ret).unwrap();
}
