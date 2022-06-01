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

fn find_factor(prime_numbers: &Vec<usize>, p: usize, pos: usize, val: usize) -> usize {
    find_factor_internal(prime_numbers, p, pos, val, 0)
}

fn find_factor_internal(
    prime_numbers: &Vec<usize>,
    divisor: usize,
    pos: usize,
    val: usize,
    p: usize,
) -> usize {
    if val < divisor {
        return 0;
    }

    let mut ret = val / divisor;

    for i in p..pos {
        if divisor * prime_numbers[i] > val {
            break;
        }

        ret -= find_factor_internal(prime_numbers, divisor * prime_numbers[i], pos, val, i + 1)
    }

    ret
}

// Reference: https://rkm0959.tistory.com/77
fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let (n, p) = (scan.token::<usize>(), scan.token::<usize>());
    // NOTE: sqrt(10^9) = 31622
    let mut prime_numbers = vec![2];
    let mut arr = vec![0; 31622];

    // Preprocess: Sieve of Eratosthenes
    for i in (3..31622).step_by(2) {
        if arr[i] > 0 {
            continue;
        }

        prime_numbers.push(i);
        arr[i] = 1;

        for j in (i * i..31622).step_by(i) {
            arr[j] = 1;
        }
    }

    let pos = prime_numbers.lower_bound(&p);
    if pos == prime_numbers.len() {
        writeln!(out, "{}", if n == 1 { p } else { 0 }).unwrap();
        return;
    }

    if n == 1 {
        writeln!(out, "{p}").unwrap();
        return;
    }

    // Binary Search
    let mut left = p;
    let mut right = 1_000_000_000 / p + 1;
    let end = right;

    while left < right {
        let mid = (left + right) / 2;
        if find_factor(&prime_numbers, p, pos, mid * p) >= n {
            right = mid;
        } else {
            left = mid + 1;
        }
    }

    writeln!(out, "{}", if right == end { 0 } else { right * p }).unwrap();
}
