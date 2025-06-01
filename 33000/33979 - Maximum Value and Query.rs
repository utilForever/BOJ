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
const MOD_INV_2: i64 = 499_122_177;

#[inline]
fn normalize(mut x: i64) -> i64 {
    x %= MOD;

    if x < 0 {
        x += MOD;
    }

    x
}

#[inline]
fn multiply(a: i64, b: i64) -> i64 {
    (normalize(a) as i128 * normalize(b) as i128 % MOD as i128) as i64
}

#[derive(Default, Clone, Copy)]
struct ParaNode {
    last_tick: i64,
    sum: i64,
    delta_sum: i64,
    delta2_sum: i64,
    len: i64,
    delta_len: i64,
}

impl ParaNode {
    fn push(&mut self, tick: i64) {
        if self.last_tick == tick {
            return;
        }

        let dt = tick - self.last_tick;
        let dt_mod = dt % MOD;

        let part = normalize(normalize(self.delta_sum * 2) + multiply(dt_mod + 1, self.delta2_sum));
        let incr = multiply(multiply(dt_mod, part), MOD_INV_2);

        self.last_tick = tick;
        self.sum = normalize(self.sum + incr);
        self.delta_sum = normalize(self.delta_sum + multiply(dt_mod, self.delta2_sum));
        self.len += dt * self.delta_len;
    }
}

struct ParaSegTree<'a> {
    size: usize,
    nodes: Vec<ParaNode>,
    tick: i64,
    row_base: &'a [i64],
}

impl<'a> ParaSegTree<'a> {
    fn new(size: usize, row_base: &'a [i64]) -> Self {
        let mut nodes = vec![ParaNode::default(); size * 2];

        for (i, &val) in row_base.iter().enumerate() {
            nodes[size + i].delta2_sum = normalize(val);
            nodes[size + i].delta_len = 1;
        }

        for idx in (1..size).rev() {
            nodes[idx].delta2_sum =
                normalize(nodes[idx * 2].delta2_sum + nodes[idx * 2 + 1].delta2_sum);
            nodes[idx].delta_len = nodes[idx * 2].delta_len + nodes[idx * 2 + 1].delta_len;
        }

        Self {
            size,
            nodes,
            tick: 0,
            row_base,
        }
    }

    fn update_leaf_len(&mut self, idx: usize, val: i64) {
        let mut pos = idx + self.size;

        while pos > 0 {
            self.nodes[pos].push(self.tick);
            self.nodes[pos].delta_len += val;
            self.nodes[pos].delta2_sum =
                normalize(self.nodes[pos].delta2_sum + multiply(val, self.row_base[idx]));

            pos >>= 1;
        }
    }

    fn prefix_sum_until(&mut self, mut target: i64) -> (usize, i64) {
        let mut node = 1;
        let mut acc = 0;

        while node < self.size {
            let left = node * 2;

            self.nodes[left].push(self.tick);

            if self.nodes[left].len >= target {
                node = left;
            } else {
                acc = normalize(acc + self.nodes[left].sum);
                target -= self.nodes[left].len;
                node = left + 1;
            }
        }

        (node - self.size, acc)
    }
}

#[derive(Default, Clone, Copy)]
struct TriNode {
    last_tick: i64,
    sum: i64,
    delta_sum: i64,
}

impl TriNode {
    fn push(&mut self, tick: i64) {
        if self.last_tick == tick {
            return;
        }

        let dt = tick - self.last_tick;

        self.last_tick = tick;
        self.sum = normalize(self.sum + multiply(dt, self.delta_sum));
    }
}

struct TriSegTree<'a> {
    size: usize,
    nodes: Vec<TriNode>,
    tick: i64,
    left_span: &'a [i64],
    row_base: &'a [i64],
}

impl<'a> TriSegTree<'a> {
    fn new(size: usize, left_span: &'a [i64], row_base: &'a [i64]) -> Self {
        Self {
            size,
            nodes: vec![TriNode::default(); size * 2],
            tick: 0,
            left_span,
            row_base,
        }
    }

    fn update_leaf(&mut self, idx: usize, dl: i64) {
        let add = multiply(dl, multiply(self.left_span[idx], self.row_base[idx]));
        let mut pos = idx + self.size;

        while pos > 0 {
            self.nodes[pos].push(self.tick);
            self.nodes[pos].delta_sum = normalize(self.nodes[pos].delta_sum + add);

            pos >>= 1;
        }
    }

    fn query(&mut self, mut left: usize, mut right: i64) -> i64 {
        let mut acc = 0;

        left += self.size;
        right += self.size as i64;

        let mut right = right as usize;

        while left <= right {
            if left & 1 == 1 {
                self.nodes[left].push(self.tick);
                acc = normalize(acc + self.nodes[left].sum);
            }

            if right & 1 == 0 {
                self.nodes[right].push(self.tick);
                acc = normalize(acc + self.nodes[right].sum);
            }

            left = (left + 1) >> 1;
            right = (right - 1) >> 1;
        }

        acc
    }
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let (n, q) = (scan.token::<usize>(), scan.token::<usize>());
    let mut nums = vec![0; n + 1];
    let mut max_info = (i64::MIN, 0);

    for i in 1..=n {
        nums[i] = scan.token::<i64>();

        if (nums[i], i) > max_info {
            max_info = (nums[i], i);
        }
    }

    let mut row_rotated = vec![0; n];

    for i in 1..=n {
        let mut j = i as i64 - max_info.1 as i64;

        if j < 0 {
            j += n as i64;
        }

        row_rotated[j as usize] = nums[i];
    }

    let mut span_left = vec![0; n];
    let mut span_right = vec![0; n];

    {
        let mut stack = Vec::with_capacity(n);
        stack.push((row_rotated[0] + 1, 0));

        for i in 1..n {
            while let Some(&(val, idx)) = stack.last() {
                if row_rotated[i] > val || (row_rotated[i] == val && i > idx) {
                    stack.pop();
                } else {
                    break;
                }
            }

            span_left[i] = (i - stack.last().unwrap().1) as i64;
            stack.push((row_rotated[i], i));
        }
    }

    {
        let mut stack = Vec::with_capacity(n);
        stack.push((row_rotated[0] + 1, n));

        for i in (1..n).rev() {
            while let Some(&(val, idx)) = stack.last() {
                if row_rotated[i] > val || (row_rotated[i] == val && i > idx) {
                    stack.pop();
                } else {
                    break;
                }
            }

            span_right[i] = (stack.last().unwrap().1 - i) as i64;
            stack.push((row_rotated[i], i));
        }
    }

    span_left[0] = n as i64;
    span_right[0] = n as i64;

    let mut size = 1;

    while size < n {
        size <<= 1;
    }

    let mut para_tree = ParaSegTree::new(size, &row_rotated);
    let mut tri_tree = TriSegTree::new(size, &span_left, &row_rotated);
    let mut ret = vec![0; q + 1];

    let mut events_para: Vec<(i64, i64, i64, usize)> = Vec::new();
    let mut events_tri: Vec<(i64, i64, usize, usize)> = Vec::new();

    for idx in 1..n {
        events_para.push((span_left[idx], 0, -1, idx));
        events_para.push((span_right[idx], 0, -1, idx));
        events_para.push((span_left[idx] + span_right[idx], 0, 1, idx));

        events_tri.push((idx as i64 - 1, 0, 1, idx));
        events_tri.push((idx as i64 + span_right[idx] - 1, 0, usize::MAX, idx));
    }

    for i in 1..=q {
        let (a, b, mut c, mut d) = (
            scan.token::<i64>(),
            scan.token::<i64>(),
            scan.token::<i64>(),
            scan.token::<i64>(),
        );

        c -= max_info.1 as i64;

        if c < 0 {
            c += n as i64;
        }

        d -= max_info.1 as i64;

        if d < 0 {
            d += n as i64;
        }

        if c > 0 && a > 1 {
            events_para.push((a - 1, 1, c - 1, i));
        }

        if a > 1 {
            events_para.push((a - 1, -1, d, i));
        }

        if c > 0 {
            events_para.push((b, -1, c - 1, i));
        }

        events_para.push((b, 1, d, i));

        if c > d {
            if a > 1 {
                events_para.push((a - 1, -1, n as i64 - 1, i));
            }

            events_para.push((b, 1, n as i64 - 1, i));
        }
    }

    events_para.sort();

    for &(time, kind, val, idx) in events_para.iter() {
        para_tree.tick = time;

        if kind == 0 {
            para_tree.update_leaf_len(idx, val);
        } else {
            para_tree.nodes[1].push(para_tree.tick);

            let (leaf_idx, prefix_sum) = para_tree.prefix_sum_until(val + 1);

            ret[idx] = normalize(ret[idx] + kind * prefix_sum);

            let mut residual = normalize((val - leaf_idx as i64 + 1) * time % MOD);
            residual = normalize(
                residual
                    - multiply(
                        (val - leaf_idx as i64) % MOD,
                        multiply((val - leaf_idx as i64 + 1) % MOD, MOD_INV_2),
                    ),
            );

            if time > span_left[leaf_idx] {
                residual = normalize(
                    residual
                        - multiply(
                            (time - span_left[leaf_idx]) % MOD,
                            multiply((time - span_left[leaf_idx] + 1) % MOD, MOD_INV_2),
                        ),
                );
            }

            ret[idx] = normalize(ret[idx] + kind * multiply(row_rotated[leaf_idx], residual));

            events_tri.push((val, kind as i64, idx, leaf_idx + 1));
        }
    }

    events_tri.sort();

    for &(time, sign, qid, leaf_idx) in events_tri.iter() {
        tri_tree.tick = time;

        if sign == 0 {
            let dl = if qid == usize::MAX { -1 } else { 1 };
            tri_tree.update_leaf(leaf_idx, dl);
        } else {
            let val = tri_tree.query(leaf_idx, time);
            ret[qid] = normalize(ret[qid] + sign * val);
        }
    }

    for i in 1..=q {
        writeln!(out, "{}", normalize(ret[i])).unwrap();
    }
}
