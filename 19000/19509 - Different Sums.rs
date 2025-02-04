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

// Reference: https://en.wikipedia.org/wiki/Golomb_ruler
// Reference: https://www.cnblogs.com/duoxiao/p/5777748.html
// Reference: https://www.cnblogs.com/mcginn/p/5824000.html
fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let t = scan.token::<i64>();
    let mut is_prime = vec![true; 3001];
    let mut primes = Vec::new();

    for i in 2..=3000 {
        if !is_prime[i] {
            continue;
        }

        primes.push(i);

        for j in (i * i..=3000).step_by(i) {
            is_prime[j] = false;
        }
    }

    for _ in 0..t {
        let n = scan.token::<usize>();
        let p = primes[primes.upper_bound(&n)];
        let mut ret = vec![0; n + 1];

        for x in 1..p {
            let mut sum: Vec<usize> = vec![0; n + 1];

            for i in 1..=n {
                sum[i] = 2 * i * p + (i * (i + 1) / 2) * x % p;
            }

            let mut check = true;

            for i in 1..=n {
                ret[i] = sum[i] - sum[i - 1];

                if ret[i] >= 3 * (n + 6) {
                    check = false;
                    break;
                }
            }

            if check {
                break;
            }
        }

        for i in 1..=n {
            write!(out, "{} ", ret[i]).unwrap();
        }

        writeln!(out).unwrap();
    }
}
