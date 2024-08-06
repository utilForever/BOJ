use io::Write;
use std::{cmp::Ordering, io, str};
use Ordering::Less;

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

    fn lower_bound(&self, x: &Self::Item) -> usize
    where
        Self::Item: Ord;

    fn lower_bound_by<'a, F>(&'a self, f: F) -> usize
    where
        F: FnMut(&'a Self::Item) -> Ordering;
}

impl<T> Ext for [T] {
    type Item = T;

    fn lower_bound(&self, x: &Self::Item) -> usize
    where
        T: Ord,
    {
        self.lower_bound_by(|y| y.cmp(x))
    }

    fn lower_bound_by<'a, F>(&'a self, mut f: F) -> usize
    where
        F: FnMut(&'a Self::Item) -> Ordering,
    {
        let s = self;
        let mut size = s.len();

        if size == 0 {
            return 0;
        }

        let mut base = 0usize;

        while size > 1 {
            let half = size / 2;
            let mid = base + half;
            let cmp = f(unsafe { s.get_unchecked(mid) });

            base = if cmp == Less { mid } else { base };
            size -= half;
        }

        let cmp = f(unsafe { s.get_unchecked(base) });

        base + (cmp == Less) as usize
    }
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let (n, q) = (scan.token::<usize>(), scan.token::<i64>());
    let mut difficulties = vec![0; n];

    for i in 0..n {
        difficulties[i] = scan.token::<i64>();
    }

    let mut left = vec![0; n];
    let mut right = vec![0; n];
    let mut nums = Vec::new();

    // left -> right
    for i in 0..n {
        let pos = nums.lower_bound(&difficulties[i]);
        left[i] = pos + 1;

        if pos == nums.len() {
            nums.push(difficulties[i]);
        } else {
            nums[pos] = difficulties[i];
        }
    }

    nums.clear();

    // right -> left
    for i in (0..n).rev() {
        let pos = nums.lower_bound_by(|x| difficulties[i].cmp(x));
        right[i] = pos + 1;

        if pos == nums.len() {
            nums.push(difficulties[i]);
        } else {
            nums[pos] = difficulties[i];
        }
    }

    for _ in 0..q {
        let d = scan.token::<usize>();

        writeln!(out, "{}", left[d - 1] + right[d - 1] - 1).unwrap();
    }
}
