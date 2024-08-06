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

fn init(
    arr: &Vec<(usize, usize)>,
    holes: &Vec<bool>,
    tree: &mut Vec<(usize, usize, usize)>,
    node: usize,
    start: usize,
    end: usize,
) {
    if start == end {
        tree[node] = (arr[start].1, start, if holes[start] { 1 } else { 0 });
    } else {
        init(arr, holes, tree, node * 2, start, (start + end) / 2);
        init(arr, holes, tree, node * 2 + 1, (start + end) / 2 + 1, end);

        tree[node].0 = if tree[node * 2].0 < tree[node * 2 + 1].0 {
            tree[node * 2].0
        } else {
            tree[node * 2 + 1].0
        };
        tree[node].1 = if tree[node * 2].0 < tree[node * 2 + 1].0 {
            tree[node * 2].1
        } else {
            tree[node * 2 + 1].1
        };
        tree[node].2 = tree[node * 2].2 + tree[node * 2 + 1].2;
    }
}

fn query(
    tree: &Vec<(usize, usize, usize)>,
    node: usize,
    start: usize,
    end: usize,
    i: usize,
    j: usize,
) -> (usize, usize, usize) {
    if i > end || j < start {
        return (1_000_000_007, 0, 0);
    }

    if i <= start && j >= end {
        return tree[node];
    }

    let left = query(tree, node * 2, start, (start + end) / 2, i, j);
    let right = query(tree, node * 2 + 1, (start + end) / 2 + 1, end, i, j);

    let mut ret = (0, 0, 0);
    ret.0 = if left.0 < right.0 { left.0 } else { right.0 };
    ret.1 = if left.0 < right.0 { left.1 } else { right.1 };
    ret.2 = left.2 + right.2;

    ret
}

fn get_area(
    arr: &Vec<(usize, usize)>,
    tree: &Vec<(usize, usize, usize)>,
    n: &usize,
    start: usize,
    end: usize,
    accumulated_height: usize,
) -> (f64, usize, usize) {
    if start > end {
        return (0.0, 0, 0);
    }

    let (height, idx, num_holes) = query(tree, 1, 1, n / 2 - 1, start, end);
    let mut cur_area = 0;
    let mut time = 0.0;

    if num_holes > 0 {
        cur_area = (arr[end + 1].0 - arr[start].0) * (height - accumulated_height);
        time = cur_area as f64 / num_holes as f64;
    }

    let whole_area = (arr[end + 1].0 - arr[start].0) * (height - accumulated_height);
    let (left_area, right_area) = (
        get_area(arr, tree, n, start, idx - 1, height),
        get_area(arr, tree, n, idx + 1, end, height),
    );

    let mut ret = (0.0, 0, 0);
    ret.0 = time + f64::max(left_area.0, right_area.0);
    ret.1 = cur_area + left_area.1 + right_area.1;
    ret.2 = whole_area + left_area.2 + right_area.2;

    ret
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let n = scan.token::<usize>();
    let mut coords = vec![(0, 0); n / 2];

    for i in 0..(n / 2) {
        (coords[i].0, coords[i].1) = (scan.token::<usize>(), scan.token::<usize>());
        (coords[i].0, coords[i].1) = (scan.token::<usize>(), scan.token::<usize>());
    }

    let k = scan.token::<usize>();
    let mut holes = vec![false; n / 2 + 1];

    for _ in 0..k {
        let (a, _) = (scan.token::<usize>(), scan.token::<usize>());
        (_, _) = (scan.token::<usize>(), scan.token::<usize>());

        holes[coords.lower_bound_by(|&val| val.0.cmp(&a)) + 1] = true;
    }

    coords.insert(0, (0, 0));

    let mut tree = vec![(0, 0, 0); 2 * n];

    init(&coords, &holes, &mut tree, 1, 1, n / 2 - 1);

    let area = get_area(&coords, &tree, &n, 1, n / 2 - 1, 0);

    writeln!(out, "{:.2}", area.0).unwrap();
    writeln!(out, "{}", area.2 - area.1).unwrap();
}
