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
    prefix: usize,
    idx: usize,
    coeff: i64,
}

impl Event {
    fn new(prefix: usize, idx: usize, coeff: i64) -> Self {
        Event { prefix, idx, coeff }
    }
}

fn process_queries(seaweed: &[i64], queries: &[(usize, usize)], n: usize) -> Vec<i64> {
    let mut points = Vec::with_capacity(n);

    for (row, &col) in seaweed.iter().enumerate() {
        if col != i64::MAX {
            points.push((row + n, col as usize));
        }
    }

    points.sort_unstable_by_key(|&(row, _)| row);

    let mut events = Vec::with_capacity(queries.len() * 2);

    for (idx, &(l, r)) in queries.iter().enumerate() {
        events.push(Event::new(r + n, idx, 1));

        if l > 0 {
            events.push(Event::new(l + n - 1, idx, -1));
        }
    }

    events.sort_unstable_by_key(|e| e.prefix);

    let mut fenwick = FenwickTree::new(n);
    let mut idx = 0;
    let mut ret = vec![0; queries.len()];

    for event in events {
        while idx < points.len() && points[idx].0 <= event.prefix {
            fenwick.update(points[idx].1 + 1, 1);
            idx += 1;
        }

        let (l, r) = (queries[event.idx].0 + 1, queries[event.idx].1 + 1);
        let cnt = fenwick.query_range(l, r);

        ret[event.idx] += cnt * event.coeff;
    }

    ret.into_iter()
        .zip(queries)
        .map(|(cnt, &(l, r))| (r - l + 1) as i64 - cnt)
        .collect()
}

// Reference: https://koosaga.com/315
// Reference: https://koosaga.com/316
fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let (n, q) = (scan.token::<usize>(), scan.token::<usize>());
    let mut nums = vec![0; n];

    for i in 0..n {
        nums[i] = scan.token::<usize>() - 1;
    }

    let seaweed = make_seaweed_matrix(&nums);
    let mut queries = vec![(0, 0); q];

    for i in 0..q {
        let (l, r) = (scan.token::<usize>() - 1, scan.token::<usize>() - 1);
        queries[i] = (l, r);
    }

    let ret = process_queries(&seaweed, &queries, n);

    for i in 0..q {
        writeln!(out, "{}", ret[i]).unwrap();
    }
}
