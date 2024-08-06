use io::Write;
use std::{
    cmp::Ordering::{self, Greater, Less},
    io, str,
};

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

pub trait Ext {
    type Item;

    fn equal_range(&self, x: &Self::Item) -> core::ops::Range<usize>
    where
        Self::Item: Ord;

    fn equal_range_by<'a, F>(&'a self, f: F) -> core::ops::Range<usize>
    where
        F: FnMut(&'a Self::Item) -> Ordering;
}

impl<T> Ext for [T] {
    type Item = T;

    fn equal_range(&self, x: &Self::Item) -> core::ops::Range<usize>
    where
        T: Ord,
    {
        self.equal_range_by(|y| y.cmp(x))
    }

    fn equal_range_by<'a, F>(&'a self, mut f: F) -> core::ops::Range<usize>
    where
        F: FnMut(&'a Self::Item) -> Ordering,
    {
        let s = self;
        let mut size = s.len();

        if size == 0 {
            return 0..0;
        }

        let mut base = (0usize, 0usize);

        while size > 1 {
            let half = size / 2;
            let mid = (base.0 + half, base.1 + half);
            let cmp = (
                f(unsafe { s.get_unchecked(mid.0) }),
                f(unsafe { s.get_unchecked(mid.1) }),
            );

            base = (
                if cmp.0 == Less { mid.0 } else { base.0 },
                if cmp.1 == Greater { base.1 } else { mid.1 },
            );
            size -= half;
        }

        let cmp = (
            f(unsafe { s.get_unchecked(base.0) }),
            f(unsafe { s.get_unchecked(base.1) }),
        );

        base.0 + (cmp.0 == Less) as usize..base.1 + (cmp.1 != Greater) as usize
    }
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let n = scan.token::<usize>();
    let mut a = vec![0; n];
    let mut b = vec![0; n];
    let mut c = vec![0; n];
    let mut d = vec![0; n];

    for i in 0..n {
        (a[i], b[i], c[i], d[i]) = (
            scan.token::<i64>(),
            scan.token::<i64>(),
            scan.token::<i64>(),
            scan.token::<i64>(),
        );
    }

    // Observation
    // a + b + c + d = 0 -> a + b = -(c + d)
    // First, calculate all possible sum of a + b and c + d
    // Then, sort both arrays
    // For each element in a + b, find the number of elements in c + d
    // that satisfy the condition using lower_bound and upper_bound

    let mut sum_ab = vec![0; n * n];
    let mut sum_cd = vec![0; n * n];

    for i in 0..n {
        for j in 0..n {
            sum_ab[i * n + j] = a[i] + b[j];
            sum_cd[i * n + j] = c[i] + d[j];
        }
    }

    sum_ab.sort();
    sum_cd.sort();

    let mut ret = 0;

    for i in 0..n * n {
        let range = sum_cd.equal_range(&-sum_ab[i]);
        ret += range.end - range.start;
    }

    writeln!(out, "{ret}").unwrap();
}
