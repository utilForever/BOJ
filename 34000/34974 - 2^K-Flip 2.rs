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

    pub fn all(&mut self) -> String {
        let mut input = String::new();
        self.reader.read_to_string(&mut input).expect("Failed read");
        input
    }

    pub fn line(&mut self) -> String {
        let mut input = String::new();
        self.reader.read_line(&mut input).expect("Failed read");
        input
    }
}

const MOD: i64 = 998_244_353;

struct UnionFind {
    parent: Vec<usize>,
    size: Vec<usize>,
}

impl UnionFind {
    fn new(n: usize) -> Self {
        UnionFind {
            parent: vec![0; n + 1],
            size: vec![1; n + 1],
        }
    }

    fn init(&mut self) {
        for i in 0..self.parent.len() {
            self.parent[i] = i;
        }
    }

    fn find(&mut self, x: usize) -> usize {
        if self.parent[x] != x {
            self.parent[x] = self.find(self.parent[x]);
        }

        self.parent[x]
    }

    fn union(&mut self, x: usize, y: usize) -> bool {
        let mut root_x = self.find(x);
        let mut root_y = self.find(y);

        if root_x == root_y {
            return false;
        }

        if self.size[root_x] < self.size[root_y] {
            std::mem::swap(&mut root_x, &mut root_y);
        }

        self.parent[root_y] = root_x;
        self.size[root_x] += self.size[root_y];

        true
    }
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let (n, k) = (scan.token::<usize>(), scan.token::<usize>());
    let mut a = vec![0; n + 1];

    for i in 1..=n {
        a[i] = scan.token::<i64>();
    }

    let mut queries = vec![(0, 0); k];

    for i in 0..k {
        queries[i] = (scan.token::<usize>(), scan.token::<usize>());
    }

    let mut union_find = UnionFind::new(n + 1);
    union_find.init();

    for &(l, r) in queries.iter() {
        union_find.union(l, r + 1);
    }

    let mut delta = vec![0; n + 2];

    for i in 2..=n {
        delta[i] = a[i] ^ a[i - 1];
    }

    let mut root_to_idx = vec![usize::MAX; n + 2];
    let mut comp_of = vec![0; n + 2];
    let mut comp_size = Vec::new();
    let mut comp_min = Vec::new();
    let mut comp_max = Vec::new();
    let mut comp_delta_xor = Vec::new();

    for i in 1..=n + 1 {
        let root = union_find.find(i);

        let comp_idx = if root_to_idx[root] == usize::MAX {
            let idx = comp_size.len();

            root_to_idx[root] = idx;
            comp_size.push(0);
            comp_min.push(i);
            comp_max.push(i);
            comp_delta_xor.push(0);

            idx
        } else {
            root_to_idx[root]
        };

        comp_of[i] = comp_idx;
        comp_size[comp_idx] += 1;

        if i < comp_min[comp_idx] {
            comp_min[comp_idx] = i;
        }

        if i > comp_max[comp_idx] {
            comp_max[comp_idx] = i;
        }

        comp_delta_xor[comp_idx] ^= delta[i];
    }

    let mut start = vec![Vec::<usize>::new(); n + 2];

    for comp_idx in 0..comp_size.len() {
        let min = comp_min[comp_idx];

        if min <= n {
            start[min].push(comp_max[comp_idx]);
        }
    }

    let mut max_reach = vec![0; n + 1];
    let mut curr = 0;

    for l in 1..=n {
        for &max in start[l].iter() {
            curr = curr.max(max);
        }

        if l > 1 && max_reach[l - 1] > curr {
            curr = max_reach[l - 1];
        }

        max_reach[l] = curr;
    }

    let mut pow2 = vec![0; k + 1];
    pow2[0] = 1;

    for i in 1..=k {
        pow2[i] = (pow2[i - 1] * 2) % MOD;
    }

    let mut ret = 0;

    for l in 1..=n {
        let mut cnt = vec![0; comp_size.len()];
        let mut cnt_full = 0;
        let mut prefix = 0;
        let mut is_bad = false;

        for r in l..=n {
            if r > l {
                let cid = comp_of[r];

                cnt[cid] += 1;

                if cnt[cid] == comp_size[cid] {
                    cnt_full += 1;

                    if comp_delta_xor[cid] == 1 {
                        is_bad = true;
                    }
                }

                if comp_min[cid] <= l {
                    prefix ^= delta[r];
                }
            }

            if is_bad {
                continue;
            }

            let rank_fixed = (r - l) - cnt_full;

            if max_reach[l] >= r + 1 {
                let rank_total = rank_fixed + 1;
                let exp = k - rank_total;

                ret = (ret + pow2[exp]) % MOD;
            } else {
                if prefix != 1 - a[l] {
                    continue;
                }

                let rank_total = rank_fixed;
                let exp = k - rank_total;

                ret = (ret + pow2[exp]) % MOD;
            }
        }
    }

    writeln!(out, "{ret}").unwrap();
}
