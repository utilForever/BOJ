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

const MOD: i64 = 1_000_000_007;

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let (n, m, q) = (
        scan.token::<usize>(),
        scan.token::<usize>(),
        scan.token::<i64>(),
    );
    let mut weights = vec![0; n + 1];
    let mut vertices = vec![(0, 0); n + 1];
    let mut graph = vec![Vec::new(); n + 1];

    for i in 1..=n {
        let w = scan.token::<i64>();
        weights[i] = w;
        vertices[i] = (w, i);
    }

    for _ in 0..m {
        let (u, v) = (scan.token::<usize>(), scan.token::<usize>());
        graph[u].push(v);
        graph[v].push(u);
    }

    for i in 1..=n {
        graph[i].sort();
    }

    vertices.sort();

    let mut dp = vec![1; n + 1];
    let mut dp_rev = vec![1; n + 1];

    for &(weight, idx) in vertices.iter().skip(1) {
        for &next in graph[idx].iter() {
            if weights[next] > weight {
                dp[next] = (dp[next] + dp[idx]) % MOD;
            }
        }
    }

    for &(weight, idx) in vertices.iter().skip(1).rev() {
        for &next in graph[idx].iter() {
            if weights[next] < weight {
                dp_rev[next] = (dp_rev[next] + dp_rev[idx]) % MOD;
            }
        }
    }

    let mut sum = 0;

    for i in 1..=n {
        sum = (sum + dp[i]) % MOD;
    }

    for _ in 0..q {
        let (mut a, mut b) = (scan.token::<usize>(), scan.token::<usize>());

        if weights[a] == weights[b] {
            writeln!(out, "{sum}").unwrap();
        } else {
            if weights[a] > weights[b] {
                std::mem::swap(&mut a, &mut b);
            }

            let pos = graph[a].lower_bound(&b);
            let is_active = pos < graph[a].len() && graph[a][pos] == b;
            let sign = if is_active { -1 } else { 1 };
            let mut ret = (sum + sign * (dp[a] * dp_rev[b]) % MOD) % MOD;

            if ret < 0 {
                ret += MOD;
            }

            writeln!(out, "{ret}",).unwrap();
        }
    }
}
