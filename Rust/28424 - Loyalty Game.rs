use io::Write;
use std::{cmp::Ordering, collections::VecDeque, io, str};

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

pub trait LowerBound {
    type Item;

    fn lower_bound(&self, x: &Self::Item) -> Result<usize, usize>;
}

impl<T: Ord> LowerBound for [T] {
    type Item = T;

    fn lower_bound(&self, x: &Self::Item) -> Result<usize, usize> {
        let mut left = 0;
        let len = self.len();
        let mut right = len;

        while left < right {
            let mid = left + (right - left) / 2;

            match self[mid].cmp(x) {
                Ordering::Less => left = mid + 1,
                _ => right = mid,
            }
        }

        assert_eq!(left, right);

        if left == len {
            Err(left)
        } else {
            Ok(left)
        }
    }
}

impl<T: Ord> LowerBound for VecDeque<T> {
    type Item = T;

    fn lower_bound(&self, x: &Self::Item) -> Result<usize, usize> {
        self.as_slices().0.lower_bound(x)
    }
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let (n, q) = (scan.token::<usize>(), scan.token::<i64>());
    let mut capacities_curr = vec![0; n];
    let mut capacities_total = vec![0; n];
    let mut people = VecDeque::new();

    for i in 0..n {
        capacities_total[i] = scan.token::<i64>();
        people.push_back(i);
    }

    for _ in 0..q {
        let command = scan.token::<i64>();

        if command == 1 {
            let (i, mut x) = (scan.token::<usize>() - 1, scan.token::<i64>());
            let idx = match people.lower_bound(&i) {
                Ok(idx) => idx,
                Err(idx) => idx,
            };

            while idx < people.len() && x > 0 {
                let consumed =
                    (capacities_total[people[idx]] - capacities_curr[people[idx]]).min(x);
                x -= consumed;
                capacities_curr[people[idx]] += consumed;

                if capacities_curr[people[idx]] == capacities_total[people[idx]] {
                    people.remove(idx);
                }
            }
        } else {
            let i = scan.token::<usize>() - 1;
            writeln!(out, "{}", capacities_curr[i]).unwrap();
        }
    }
}
