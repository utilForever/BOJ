#![allow(dead_code)]

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

#[derive(Copy, Clone, PartialEq, Eq)]
enum Side {
    Left,
    Right,
}

fn multiply_unit_monge_matrix_distance_internal(row: &[usize], col: &[usize]) -> Vec<i64> {
    let n = row.len();
    let half = n / 2;

    match n {
        1 => {
            return vec![0];
        }
        2 => {
            return if row[0] == 0 && col[0] == 0 {
                vec![0, 1]
            } else {
                vec![1, 0]
            };
        }
        _ => {}
    }

    let mut row_left = Vec::with_capacity(half);
    let mut row_right = Vec::with_capacity(n - half);
    let mut idx_row_left = Vec::with_capacity(half);
    let mut idx_row_right = Vec::with_capacity(n - half);

    for (idx, &val) in row.iter().enumerate() {
        if val < half {
            row_left.push(val);
            idx_row_left.push(idx);
        } else {
            row_right.push(val - half);
            idx_row_right.push(idx);
        }
    }

    let mut col_left = Vec::with_capacity(half);
    let mut col_right = Vec::with_capacity(n - half);
    let mut idx_col_left = Vec::with_capacity(half);
    let mut idx_col_right = Vec::with_capacity(n - half);

    for (idx, &val) in col.iter().enumerate() {
        if val < half {
            col_left.push(val);
            idx_col_left.push(idx);
        } else {
            col_right.push(val - half);
            idx_col_right.push(idx);
        }
    }

    let ret_left = multiply_unit_monge_matrix_distance_internal(&row_left, &col_left);
    let ret_right = multiply_unit_monge_matrix_distance_internal(&row_right, &col_right);

    let mut row_to_col = vec![0; n];
    let mut row_block_side = vec![Side::Left; n];

    for (idx_local, &row_global) in idx_row_left.iter().enumerate() {
        let col_local = ret_left[idx_local];
        let col_global = idx_col_left[col_local as usize];

        row_to_col[row_global] = col_global;
        row_block_side[row_global] = Side::Left;
    }

    for (idx_local, &row_global) in idx_row_right.iter().enumerate() {
        let col_local = ret_right[idx_local];
        let col_global = idx_col_right[col_local as usize];

        row_to_col[row_global] = col_global;
        row_block_side[row_global] = Side::Right;
    }

    let mut ret = vec![0; n];

    for (row, &col) in row_to_col.iter().enumerate() {
        ret[row] = col as i64;
    }

    let mut col_to_row = vec![0; n];
    let mut col_block_side = vec![Side::Left; n];

    for (row, &col) in row_to_col.iter().enumerate() {
        col_to_row[col as usize] = row;
        col_block_side[col as usize] = row_block_side[row];
    }

    let (mut cursor_upper, mut cursor_lower) = (n, n);
    let (mut skew_upper, mut skew_lower) = (0, 0);

    for col in 0..n {
        if (col_block_side[col] == Side::Right) != (col_to_row[col] >= cursor_upper) {
            skew_upper -= 1;
        }

        while cursor_upper > 0 && skew_upper < 0 {
            cursor_upper -= 1;

            if (row_block_side[cursor_upper] == Side::Right) != (row_to_col[cursor_upper] > col) {
                skew_upper += 1;
            }
        }

        while cursor_lower > 0 && skew_lower <= 0 {
            cursor_lower -= 1;

            if (row_block_side[cursor_lower] == Side::Right) != (row_to_col[cursor_lower] >= col) {
                skew_lower += 1;
            }
        }

        if skew_lower > 0 && cursor_upper == cursor_lower {
            ret[cursor_lower] = col as i64;
        }

        if (col_block_side[col] == Side::Right) != (col_to_row[col] >= cursor_lower) {
            skew_lower -= 1;
        }
    }

    ret
}

fn multiply_unit_monge_matrix_distance(a: &[usize], b: &[usize]) -> Vec<i64> {
    let mut b_rev = vec![0; b.len()];

    for i in 0..b.len() {
        b_rev[b[i]] = i;
    }

    multiply_unit_monge_matrix_distance_internal(a, &b_rev)
}

const SEAWEED_NAIVE_THRESHOLD: usize = 512;

fn seaweed_naive(p: &[usize]) -> Vec<usize> {
    let n = p.len();
    let mut q0 = (0..n).map(|i| [n + i, 0]).collect::<Vec<_>>();
    let mut q1 = vec![[0, 0]; n];
    let mut crossed = vec![vec![false; 2 * n]; 2 * n];
    let mut di = (0..2 * n)
        .map(|i| if i < n { 0 } else { 1 })
        .collect::<Vec<_>>();
    let mut ret = vec![0; 2 * n];

    for i in 0..n {
        q0[0][1] = n - 1 - i;

        for j in 0..n {
            let &[u, v] = &q0[j];

            if crossed[u][v] || p[j] == i {
                di[u] ^= 1;
                di[v] ^= 1;
            } else {
                crossed[u][v] = true;
                crossed[v][u] = true;
            }

            for &uv in &[u, v] {
                if di[uv] == 1 {
                    q1[j][0] = uv;
                } else if j + 1 < n {
                    q0[j + 1][1] = uv;
                } else {
                    ret[uv] = 2 * n - 1 - i;
                }
            }
        }

        std::mem::swap(&mut q0, &mut q1);
    }

    for j in 0..n {
        ret[q0[j][0]] = j;
    }

    ret
}

fn make_seaweed_matrix_full(perm: &[usize]) -> Vec<usize> {
    let n = perm.len();

    if n <= SEAWEED_NAIVE_THRESHOLD {
        return seaweed_naive(perm);
    }

    let half = n / 2;

    let mut val_left = Vec::with_capacity(half);
    let mut val_right = Vec::with_capacity(n - half);
    let mut idx_left = Vec::with_capacity(half);
    let mut idx_right = Vec::with_capacity(n - half);

    for (idx, &val) in perm.iter().enumerate() {
        if val < half {
            val_left.push(val);
            idx_left.push(idx);
        } else {
            val_right.push(val - half);
            idx_right.push(idx);
        }
    }

    let mut seaweed_left = make_seaweed_matrix_full(&val_left);
    let mut seaweed_right = make_seaweed_matrix_full(&val_right);

    for val in seaweed_left.iter_mut() {
        *val = if *val < val_left.len() {
            idx_left[*val]
        } else {
            n + (*val - val_left.len())
        }
    }

    for val in seaweed_right.iter_mut() {
        *val = if *val < val_right.len() {
            idx_right[*val]
        } else {
            n + (*val - val_right.len())
        }
    }

    let mut perm_left = vec![0; 2 * n];
    let mut perm_right = vec![0; 2 * n];

    for i in 0..val_right.len() {
        perm_left[i] = i;
    }

    for i in val_right.len()..n {
        perm_left[i] = val_right.len() + seaweed_left[i - val_right.len()];
    }

    for i in n..2 * n {
        perm_left[i] = (i - n) + val_right.len();
    }

    for i in 0..val_right.len() {
        perm_right[i] = seaweed_right[i];
    }

    for i in val_right.len()..(val_right.len() + n) {
        perm_right[i] = i - val_right.len();
    }

    for i in (val_right.len() + n)..(2 * n) {
        perm_right[i] = i;
    }

    let mut idx_left2 = 0;
    let mut idx_right2 = 0;

    for i in 0..n {
        if perm[i] < half {
            perm_left[i + n] = seaweed_left[idx_left2 + val_left.len()] + val_right.len();
            idx_left2 += 1;
        } else {
            perm_right[i + val_right.len()] = seaweed_right[idx_right2 + val_right.len()];
            idx_right2 += 1;
        }
    }

    let dist = multiply_unit_monge_matrix_distance(&perm_left, &perm_right);
    dist.into_iter().map(|x| x as usize).collect()
}

struct FenwickTree {
    n: usize,
    data: Vec<i64>,
}

impl FenwickTree {
    fn new(n: usize) -> Self {
        FenwickTree {
            n,
            data: vec![0; n + 1],
        }
    }

    fn update(&mut self, mut idx: usize, delta: i64) {
        while idx <= self.n {
            self.data[idx] += delta;
            idx += idx & (!idx + 1);
        }
    }

    fn query(&self, mut idx: usize) -> i64 {
        let mut ret = 0;

        while idx > 0 {
            ret += self.data[idx];
            idx -= idx & (!idx + 1);
        }

        ret
    }

    fn query_range(&self, start: usize, end: usize) -> i64 {
        self.query(end) - self.query(start - 1)
    }
}

#[derive(Clone, Copy)]
struct Event {
    start_l: usize,
    start_r: usize,
    end_upper: usize,
    idx: usize,
    sign: i64,
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let t = scan.token::<usize>();

    for _ in 0..t {
        let (n, q) = (scan.token::<usize>(), scan.token::<usize>());
        let mut genes = vec![0; n];

        for i in 0..n {
            genes[i] = scan.token::<usize>() - 1;
        }

        let seaweed = make_seaweed_matrix_full(&genes);
        let m = 2 * n;

        let mut points = Vec::with_capacity(m);

        for s in 0..m {
            let e = seaweed[s];

            if e < n {
                points.push((e, s));
            }
        }

        points.sort_unstable();

        let mut events = Vec::with_capacity(2 * q);
        let mut len = vec![0; q];

        for i in 0..q {
            let (l, r) = (scan.token::<usize>() - 1, scan.token::<usize>() - 1);

            len[i] = r - l + 1;

            events.push(Event {
                start_l: n + l,
                start_r: n + r,
                end_upper: r,
                idx: i,
                sign: 1,
            });

            if l > 0 {
                events.push(Event {
                    start_l: n + l,
                    start_r: n + r,
                    end_upper: l - 1,
                    idx: i,
                    sign: -1,
                });
            }
        }

        events.sort_unstable_by(|a, b| a.end_upper.cmp(&b.end_upper));

        let mut bit = FenwickTree::new(m);
        let mut idx = 0;
        let mut cnt_cross = vec![0; q];

        for event in events {
            while idx < points.len() && points[idx].0 <= event.end_upper {
                bit.update(points[idx].1 + 1, 1);
                idx += 1;
            }

            let cnt = bit.query_range(event.start_l + 1, event.start_r + 1);
            cnt_cross[event.idx] += event.sign * cnt;
        }

        for i in 0..q {
            writeln!(out, "{}", len[i] as i64 - cnt_cross[i]).unwrap();
        }
    }
}
