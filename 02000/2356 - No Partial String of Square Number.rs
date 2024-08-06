use io::Write;
use std::{cmp::Ordering, io, str};
use Ordering::Greater;

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

    fn upper_bound(&self, x: &Self::Item) -> usize
    where
        Self::Item: Ord;

    fn upper_bound_by<'a, F>(&'a self, f: F) -> usize
    where
        F: FnMut(&'a Self::Item) -> Ordering;
}

impl<T> Ext for [T] {
    type Item = T;

    fn upper_bound(&self, x: &Self::Item) -> usize
    where
        T: Ord,
    {
        self.upper_bound_by(|y| y.cmp(x))
    }

    fn upper_bound_by<'a, F>(&'a self, mut f: F) -> usize
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
            base = if cmp == Greater { base } else { mid };
            size -= half;
        }
        let cmp = f(unsafe { s.get_unchecked(base) });
        base + (cmp != Greater) as usize
    }
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let mut n = scan.token::<i64>();

    if n > 888_888_888_888_888_888 {
        writeln!(out, "{}", 2_222_222_222_222_222_222).unwrap();
        return;
    }

    let mut precomputed = vec![0; 19];
    precomputed[0] = 1;

    for i in 1..19 {
        precomputed[i] = precomputed[i - 1] * 10;
    }

    let pos = precomputed.upper_bound(&n);
    let mut idx = pos - 1;

    loop {
        let num = n / precomputed[idx as usize];

        if idx as usize == pos - 1 && num > 9 {
            for _ in 0..=pos {
                write!(out, "2").unwrap();
            }

            writeln!(out).unwrap();
            return;
        }

        if [0, 1, 4, 5, 6, 9].iter().find(|&&x| x == num % 10).is_some() {
            let pos_rest = pos - idx;

            for j in 1..=pos_rest {
                let num_rest = num % precomputed[j];
                let sqrt = (num_rest as f64).sqrt().round() as i64;

                if sqrt * sqrt == num_rest {
                    n = (n / precomputed[idx] + 1) * precomputed[idx];
                    idx = pos;
                    break;
                }
            }
        }

        if idx == 0 {
            writeln!(out, "{n}").unwrap();
            return;
        }

        idx -= 1;
    }
}
