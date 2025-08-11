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

fn make_seaweed_matrix(perm: &[usize]) -> Vec<i64> {
    let n = perm.len();
    let half = n / 2;

    if n == 1 {
        return vec![i64::MAX];
    }

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

    let seaweed_left = make_seaweed_matrix(&val_left);
    let seaweed_right = make_seaweed_matrix(&val_right);

    let mut matching_row = (0..n).collect::<Vec<_>>();
    let mut matching_col = (0..n).collect::<Vec<_>>();
    let (mut offset_left, mut offset_right) = (0, 0);

    for i in 0..n {
        if perm[i] < half {
            matching_row[i] = if seaweed_left[offset_left] == i64::MAX {
                usize::MAX
            } else {
                idx_left[seaweed_left[offset_left] as usize]
            };

            offset_left += 1;
        } else {
            matching_col[i] = if seaweed_right[offset_right] == i64::MAX {
                usize::MAX
            } else {
                idx_right[seaweed_right[offset_right] as usize]
            };

            offset_right += 1;
        }
    }

    let mut row_perm = vec![usize::MAX; n];
    let mut row_from = vec![usize::MAX; n];
    let mut matching_row_rev = vec![usize::MAX; n];

    for i in 0..n {
        if matching_row[i] != usize::MAX {
            matching_row_rev[matching_row[i]] = i;
        }
    }

    let mut pos = n as isize - 1;

    for col in (0..n).rev() {
        if matching_row_rev[col] != usize::MAX {
            row_perm[matching_row_rev[col]] = pos as usize;
            row_from[pos as usize] = col;
            pos -= 1;
        }
    }

    for row in 0..n {
        if row_perm[row] == usize::MAX {
            row_perm[row] = pos as usize;
            pos -= 1;
        }
    }

    let mut col_perm = vec![0; n];
    let mut col_from = vec![usize::MAX; n];
    let mut used = vec![false; n];
    let mut pos = 0;

    for i in 0..n {
        if matching_col[i] != usize::MAX {
            col_perm[pos] = matching_col[i];
            col_from[pos] = i;
            used[matching_col[i]] = true;
            pos += 1;
        }
    }

    for i in 0..n {
        if !used[i] {
            col_perm[pos] = i;
            pos += 1;
        }
    }

    let dist = multiply_unit_monge_matrix_distance(&col_perm, &row_perm);
    let mut ret = vec![i64::MAX; n];

    for i in 0..n {
        if col_from[i] != usize::MAX {
            let idx_col = dist[i] as usize;

            if idx_col < n && row_from[idx_col] != usize::MAX {
                ret[col_from[i]] = row_from[idx_col] as i64;
            }
        }
    }

    ret
}

fn make_seaweed_matrix_full(perm: &[usize]) -> Vec<usize> {
    let n = perm.len();

    if n == 1 {
        return vec![0, 1];
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

    let mut idx_left = 0;
    let mut idx_right = 0;

    for i in 0..n {
        if perm[i] < half {
            perm_left[i + n] = seaweed_left[idx_left + val_left.len()] + val_right.len();
            idx_left += 1;
        } else {
            perm_right[i + val_right.len()] = seaweed_right[idx_right + val_right.len()];
            idx_right += 1;
        }
    }

    let dist = multiply_unit_monge_matrix_distance(&perm_left, &perm_right);
    dist.into_iter().map(|x| x as usize).collect()
}

#[derive(Clone)]
struct Fenwick {
    n: usize,
    bit: Vec<i64>,
}

impl Fenwick {
    fn new(n: usize) -> Self {
        Self {
            n,
            bit: vec![0; n + 1],
        }
    }

    fn update(&mut self, mut idx: usize, delta: i64) {
        while idx <= self.n {
            self.bit[idx] += delta;
            idx += idx & (!idx + 1);
        }
    }

    fn query(&self, mut idx: usize) -> i64 {
        let mut s = 0;

        while idx > 0 {
            s += self.bit[idx];
            idx &= idx - 1;
        }

        s
    }

    fn query_range(&self, l: usize, r: usize) -> i64 {
        self.query(r) - self.query(l - 1)
    }
}

// Reference: https://koosaga.com/315
// Reference: https://koosaga.com/316
fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let n = scan.token::<usize>();
    let mut nums = vec![0; n];

    for i in 0..n {
        nums[i] = scan.token::<usize>() - 1;
    }

    let seaweed = make_seaweed_matrix_full(&nums);
    let m = 2 * n;

    let mut prefix_sum = vec![0; m];
    let mut acc = 0;

    for i in 0..m {
        if seaweed[i] < n {
            acc += 1;
        }

        prefix_sum[i] = acc;
    }

    let mut points = Vec::with_capacity(m);

    for s in 0..m {
        let e = seaweed[s];

        if e < n {
            points.push((e, s));
        }
    }

    points.sort_unstable();

    let mut queries = Vec::with_capacity(n - 1);
    let mut base = vec![0; n - 1];

    for idx in 1..n {
        let (left, right) = (n - idx, n + idx - 1);
        base[idx - 1] = prefix_sum[right] - prefix_sum[left - 1];
        queries.push((idx - 1, left + 1, right + 1));
    }

    queries.sort_unstable();

    let mut fenwick_tree = Fenwick::new(m);
    let mut small = vec![0; n - 1];
    let mut idx = 0;

    for query in queries {
        while idx < points.len() && points[idx].0 <= query.0 {
            fenwick_tree.update(points[idx].1 + 1, 1);
            idx += 1;
        }

        small[query.0] = fenwick_tree.query_range(query.1, query.2);
    }

    for i in 0..n - 1 {
        write!(out, "{} ", base[i] - small[i] + 1).unwrap();
    }

    writeln!(out).unwrap();
}
