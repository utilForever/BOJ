use io::Write;
use std::{io, str};

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

#[derive(Clone)]
enum Dimension {
    X,
    Y,
}

struct KDTree {
    tree: Vec<(i64, i64)>,
    threshold: usize,
}

impl KDTree {
    pub fn new(values: Vec<(i64, i64)>, threshold: usize) -> Self {
        let mut kd_tree = Self {
            tree: values,
            threshold,
        };
        kd_tree.construct(Dimension::X, 0, kd_tree.tree.len());
        kd_tree
    }

    fn construct(&mut self, dim: Dimension, left: usize, right: usize) {
        if right - left <= self.threshold {
            return;
        }

        let mid = (left + right) / 2;
        self.tree[left..right].select_nth_unstable_by(mid - left, |a, b| match dim {
            Dimension::X => a.0.cmp(&b.0),
            Dimension::Y => a.1.cmp(&b.1),
        });

        let next_dim = match dim {
            Dimension::X => Dimension::Y,
            Dimension::Y => Dimension::X,
        };

        self.construct(next_dim.clone(), left, mid);
        self.construct(next_dim, mid + 1, right);
    }

    pub fn dist(p: &(i64, i64), q: &(i64, i64)) -> i64 {
        (p.0 - q.0).pow(2) + (p.1 - q.1).pow(2)
    }

    pub fn nearest_without_same_point(&self, val: (i64, i64)) -> i64 {
        let mut ret = i64::MAX;
        self.nearest_without_same_point_internal(&val, Dimension::X, 0, self.tree.len(), &mut ret);
        ret
    }

    fn nearest_without_same_point_internal(
        &self,
        val: &(i64, i64),
        dim: Dimension,
        left: usize,
        right: usize,
        ret: &mut i64,
    ) {
        if right - left <= self.threshold {
            for i in left..right {
                if *val != self.tree[i] {
                    let dist = KDTree::dist(val, &self.tree[i]);

                    if *ret > dist {
                        *ret = dist;
                    }
                }
            }
        } else {
            let mid = (left + right) / 2;
            let diff = match dim {
                Dimension::X => val.0 - self.tree[mid].0,
                Dimension::Y => val.1 - self.tree[mid].1,
            };

            if *val != self.tree[mid] {
                let dist = KDTree::dist(val, &self.tree[mid]);

                if *ret > dist {
                    *ret = dist;
                }
            }

            let next_dim = match dim {
                Dimension::X => Dimension::Y,
                Dimension::Y => Dimension::X,
            };

            if diff < 0 {
                self.nearest_without_same_point_internal(val, next_dim.clone(), left, mid, ret);

                if diff * diff < *ret {
                    self.nearest_without_same_point_internal(val, next_dim, mid + 1, right, ret);
                }
            } else {
                self.nearest_without_same_point_internal(
                    val,
                    next_dim.clone(),
                    mid + 1,
                    right,
                    ret,
                );

                if diff * diff < *ret {
                    self.nearest_without_same_point_internal(val, next_dim, left, mid, ret);
                }
            }
        }
    }
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let t = scan.token::<usize>();

    for _ in 0..t {
        let n = scan.token::<usize>();
        let mut points = Vec::new();

        for _ in 0..n {
            let (x, y) = (scan.token::<i64>(), scan.token::<i64>());
            points.push((x, y));
        }

        let kd_tree = KDTree::new(points.clone(), 16);

        for point in points.iter() {
            writeln!(out, "{}", kd_tree.nearest_without_same_point(*point)).unwrap();
        }
    }
}
