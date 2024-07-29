use io::Write;
use std::{collections::HashSet, io, str};

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

#[derive(Clone, Default)]
struct RangeSet {
    ranges: HashSet<(usize, usize)>,
    total_length: Option<usize>,
}

impl RangeSet {
    fn add_range(&mut self, range: (usize, usize)) {
        self.ranges.insert(range);
        self.total_length = None;
    }

    fn merge(&mut self, other: &RangeSet) {
        self.ranges.extend(&other.ranges);
        self.total_length = None;
    }

    fn compute_total_length(&mut self) {
        if self.total_length.is_some() {
            return;
        }
        
        let mut lines = self.ranges.iter().copied().collect();
        lines.sort();

        let (mut left, mut right) = (0, 0);
        let mut ret = 0;

        for (x, y) in lines {
            if right < x {
                ret += right - left;
                left = x;
                right = y;
            } else {
                right = right.max(y);
            }
        }

        ret += right - left;
        self.total_length = Some(ret);
    }

    fn total_length(&mut self) -> usize {
        self.compute_total_length();
        self.total_length.unwrap()
    }
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let (n, q) = (scan.token::<usize>(), scan.token::<i64>());
    let mut sets = vec![RangeSet::default(); n];

    for i in 0..n {
        let (a, b) = (scan.token::<usize>(), scan.token::<usize>());
        sets[i].add_range((a, b + 1));
    }

    for _ in 0..q {
        let command = scan.token::<i64>();

        if command == 1 {
            let (a, b) = (scan.token::<usize>() - 1, scan.token::<usize>() - 1);

            let (set_a, set_b) = {
                if a < b {
                    let (set_a, set_b) = sets.split_at_mut(b);
                    (&mut set_a[a], &mut set_b[0])
                } else {
                    let (set_a, set_b) = sets.split_at_mut(a);
                    (&mut set_b[0], &mut set_a[b])
                }
            };

            if set_a.ranges.len() < set_b.ranges.len() {
                set_b.merge(set_a);
                std::mem::swap(set_a, set_b);
            } else {
                set_a.merge(set_b);
            }

            *set_b = RangeSet::default();
        } else {
            let a = scan.token::<usize>() - 1;
            let ret = sets[a].total_length();

            writeln!(out, "{ret}").unwrap();
        }
    }
}
