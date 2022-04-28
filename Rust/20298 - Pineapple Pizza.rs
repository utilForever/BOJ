use io::Write;
use std::{
    cmp::{self, Ordering},
    io, str,
};
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

// Compare function
fn is_same(
    arr: &Vec<usize>,
    less_pos: &Vec<i64>,
    greater_pos: &Vec<i64>,
    has_same_pos: &Vec<bool>,
    left: usize,
    right: usize,
) -> bool {
    let diff = (right - left) as i64;

    if has_same_pos[left] {
        if arr[(diff + less_pos[left]) as usize] != arr[right] {
            return false;
        } else {
            return true;
        }
    }

    if less_pos[left] != -1 && arr[(diff + less_pos[left]) as usize] >= arr[right] {
        return false;
    }

    if greater_pos[left] != -1 && arr[(diff + greater_pos[left]) as usize] <= arr[right] {
        return false;
    }

    true
}

// Reference: https://blog.cube219.me/posts/2021/solve-stock-price-prediction-problem-without-segment-tree/
fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let (mut n, k) = (scan.token::<usize>(), scan.token::<usize>());
    let mut a = vec![0; n + k - 1];
    let mut b = vec![0; k];

    for i in 0..n {
        a[i] = scan.token::<usize>();
    }

    for i in 0..k - 1 {
        a[n + i] = a[i];
    }

    n += k - 1;

    for i in 0..k {
        b[i] = scan.token::<usize>();
    }

    let mut temp = b.clone();
    temp.sort();
    temp.dedup();

    for i in 0..k {
        b[i] = temp.lower_bound(&b[i]);
    }

    // Initialize values
    let temp_len = temp.len();
    let mut prev_idx = vec![0_i64; temp_len];
    let mut next_idx = vec![0_i64; temp_len];
    let mut pos = vec![k as i64; temp_len];
    let mut cnt = vec![0_i64; temp_len];
    let mut less_pos = vec![0_i64; k];
    let mut greater_pos = vec![0_i64; k];
    let mut has_same_pos = vec![false; k];

    for i in 0..temp_len {
        prev_idx[i] = i as i64 - 1;
        next_idx[i] = i as i64 + 1;
    }

    for i in 0..k {
        pos[b[i]] = cmp::min(pos[b[i]], i as i64);
        cnt[b[i]] += 1;
    }

    // Calculate less_pos and greater_pos using doubly linked list like technique
    // In addition, calculate has_same_pos
    for i in (0..=k - 1).rev() {
        if cnt[b[i]] == 1 {
            if next_idx[b[i]] == temp_len as i64 {
                greater_pos[i] = -1;
            } else {
                greater_pos[i] = pos[next_idx[b[i]] as usize];
            }

            if prev_idx[b[i]] == -1 {
                less_pos[i] = -1;
            } else {
                less_pos[i] = pos[prev_idx[b[i]] as usize];
            }
        } else {
            has_same_pos[i] = true;
            less_pos[i] = pos[b[i]];
        }

        if cnt[b[i]] == 1 {
            if prev_idx[b[i]] != -1 {
                next_idx[prev_idx[b[i]] as usize] = next_idx[b[i]];
            }

            if next_idx[b[i]] != temp_len as i64 {
                prev_idx[next_idx[b[i]] as usize] = prev_idx[b[i]];
            }
        }

        cnt[b[i]] -= 1;
    }

    // Calculate fail function
    let mut cmp = 0;
    let mut fail = vec![0; k];

    for i in 1..k {
        while cmp > 0 && !is_same(&b, &less_pos, &greater_pos, &has_same_pos, cmp, i) {
            cmp = fail[cmp - 1];
        }

        if is_same(&b, &less_pos, &greater_pos, &has_same_pos, cmp, i) {
            cmp += 1;
            fail[i] = cmp;
        }
    }

    // Use KMP alogrithm to find the answer
    let mut ret = Vec::new();
    cmp = 0;

    for i in 0..n {
        while cmp > 0 && !is_same(&a, &less_pos, &greater_pos, &has_same_pos, cmp, i) {
            cmp = fail[cmp - 1];
        }

        if is_same(&a, &less_pos, &greater_pos, &has_same_pos, cmp, i) {
            if cmp == k - 1 {
                ret.push(i - k + 1);
                cmp = fail[cmp];
            } else {
                cmp += 1;
            }
        }
    }

    writeln!(out, "{}", ret.len()).unwrap();
}
