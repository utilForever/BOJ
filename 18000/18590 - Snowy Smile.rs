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

static SEG_MAX: usize = 1 << 12;

fn calculate(tree1: &(i64, i64, i64, i64), tree2: &(i64, i64, i64, i64)) -> (i64, i64, i64, i64) {
    let (left_sum, left_l_sum, left_r_sum, left_max) = tree1;
    let (right_sum, right_l_sum, right_r_sum, right_max) = tree2;

    let sum = left_sum + right_sum;
    let l_sum = cmp::max(*left_l_sum, left_sum + right_l_sum);
    let r_sum = cmp::max(*right_r_sum, right_sum + left_r_sum);
    let ret = *vec![*left_max, *right_max, *left_r_sum + *right_l_sum]
        .iter()
        .max()
        .unwrap();

    (sum, l_sum, r_sum, ret)
}

fn update(seg_tree: &mut Vec<(i64, i64, i64, i64)>, mut idx: usize, weight: i64) {
    idx += SEG_MAX / 2;

    seg_tree[idx].0 += weight;
    seg_tree[idx].1 += weight;
    seg_tree[idx].2 += weight;
    seg_tree[idx].3 += weight;

    while idx > 1 {
        idx /= 2;
        seg_tree[idx] = calculate(&seg_tree[idx * 2], &seg_tree[idx * 2 + 1]);
    }
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let t = scan.token::<usize>();

    for _ in 0..t {
        let n = scan.token::<usize>();

        let mut mines = vec![(0, 0, 0); n];
        let mut x_pos = vec![0; n];
        let mut y_pos = vec![0; n];

        for i in 0..n {
            let (x, y, w) = (
                scan.token::<i64>(),
                scan.token::<i64>(),
                scan.token::<i64>(),
            );

            mines[i] = (x, y, w);
            x_pos[i] = x;
            y_pos[i] = y;
        }

        x_pos.sort();
        y_pos.sort();

        x_pos.dedup();
        y_pos.dedup();

        let mut cache = vec![Vec::new(); 2000];
        let mut y_max = 0;

        for i in 0..n {
            let (mut x, mut y, w) = mines[i];

            x = x_pos.lower_bound(&x) as i64;
            y = y_pos.lower_bound(&y) as i64;

            cache[y as usize].push((x as usize, w));
            mines[i] = (x, y, w);
            y_max = cmp::max(y as usize, y_max);
        }

        let mut seg_tree = vec![(0, 0, 0, 0); SEG_MAX];
        let mut ans = 0;

        for y1 in 0..=y_max {
            seg_tree.fill((0, 0, 0, 0));

            for y2 in y1..=y_max {
                for pos in cache[y2].iter() {
                    let (x, w) = *pos;
                    update(&mut seg_tree, x, w);
                }

                ans = cmp::max(ans, seg_tree[1].3)
            }
        }

        writeln!(out, "{}", ans).unwrap();
    }
}
