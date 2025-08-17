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

mod bit {
    #[inline(always)]
    pub fn get(row: &[u64], col: usize) -> bool {
        let w = col >> 6;
        let b = col & 63;

        ((row[w] >> b) & 1) != 0
    }

    #[inline(always)]
    pub fn set(row: &mut [u64], col: usize, val: bool) {
        let w = col >> 6;
        let b = col & 63;
        let mask = 1u64 << b;

        if val {
            row[w] |= mask;
        } else {
            row[w] &= !mask;
        }
    }

    #[inline(always)]
    pub fn clear(row: &mut [u64], col: usize) {
        let w = col >> 6;
        let b = col & 63;

        row[w] &= !(1u64 << b);
    }

    #[inline(always)]
    pub fn xor(dst: &mut Vec<u64>, src: &Vec<u64>) {
        for (a, b) in dst.iter_mut().zip(src.iter()) {
            *a ^= *b;
        }
    }
}

fn rref_with_pivot_rows(
    mat: &mut Vec<Vec<u64>>,
    idxes_row: &mut Vec<usize>,
    cnt_cols: usize,
) -> (usize, Vec<usize>, Vec<usize>) {
    let rows = mat.len();
    let mut rank = 0;
    let mut pivot_rows = Vec::new();
    let mut pivot_cols = Vec::new();

    for col in 0..cnt_cols {
        let mut pivot = None;

        for r in rank..rows {
            if bit::get(&mat[r], col) {
                pivot = Some(r);
                break;
            }
        }

        if let Some(pivot) = pivot {
            mat.swap(rank, pivot);
            idxes_row.swap(rank, pivot);

            let pivot_row = mat[rank].clone();

            for r in 0..rows {
                if r != rank && bit::get(&mat[r], col) {
                    bit::xor(&mut mat[r], &pivot_row);
                }
            }

            pivot_rows.push(idxes_row[rank]);
            pivot_cols.push(col);

            rank += 1;

            if rank == cnt_cols {
                break;
            }
        }
    }

    (rank, pivot_rows, pivot_cols)
}

fn rref_with_ops(
    mat: &mut [Vec<u64>],
    ops: &mut [Vec<u64>],
    cnt_cols: usize,
    rank_target: usize,
    skip_col_a: usize,
    skip_col_b: usize,
) -> usize {
    let n = mat.len();
    let mut rank = 0;

    for col in 0..cnt_cols {
        if col == skip_col_a || col == skip_col_b {
            continue;
        }

        let mut pivot = None;

        for r in rank..n {
            if bit::get(&mat[r], col) {
                pivot = Some(r);
                break;
            }
        }

        if let Some(pivot) = pivot {
            if pivot != rank {
                mat.swap(rank, pivot);
                ops.swap(rank, pivot);
            }

            let piv_mat = mat[rank].clone();
            let piv_ops = ops[rank].clone();

            for r in (rank + 1)..n {
                if bit::get(&mat[r], col) {
                    bit::xor(&mut mat[r], &piv_mat);
                    bit::xor(&mut ops[r], &piv_ops);
                }
            }

            rank += 1;

            if rank == rank_target {
                break;
            }
        }
    }
    rank
}

fn parity_det(mut mat: Vec<Vec<u64>>, n: usize) -> bool {
    let rows = mat.len();
    let mut idxes_row = (0..rows).collect::<Vec<_>>();
    let (rank, _, _) = rref_with_pivot_rows(&mut mat, &mut idxes_row, n);

    rank == n
}

#[inline(always)]
fn parity_and2(a: &Vec<u64>, b: &Vec<u64>) -> u32 {
    let mut ret = 0;

    for i in 0..a.len() {
        ret ^= (a[i] & b[i]).count_ones() & 1;
    }

    ret & 1
}

#[inline(always)]
fn parity_and3(a: &Vec<u64>, b: &Vec<u64>, c: &Vec<u64>) -> u32 {
    let mut ret = 0;

    for i in 0..a.len() {
        ret ^= (a[i] & b[i] & c[i]).count_ones() & 1;
    }

    ret & 1
}

fn one_null_vector(mut mat: Vec<Vec<u64>>, n: usize) -> Vec<u64> {
    let mut idxes_row = (0..n).collect::<Vec<_>>();
    let (_, _, pivot_cols) = rref_with_pivot_rows(&mut mat, &mut idxes_row, n);
    let mut is_pivot = vec![false; n];

    for &col in pivot_cols.iter() {
        is_pivot[col] = true;
    }

    let bit = (n + 63) >> 6;

    for i in 0..n {
        if is_pivot[i] {
            continue;
        }

        let mut z = vec![0; bit];
        bit::set(&mut z, i, true);

        for (j, &pivot_col) in pivot_cols.iter().enumerate() {
            if bit::get(&mat[j], i) {
                let val = bit::get(&z, pivot_col);
                bit::set(&mut z, pivot_col, !val);
            }
        }

        return z;
    }

    vec![0; bit]
}

fn swap_columns(
    bits_row: &mut Vec<Vec<u64>>,
    bits_col: &mut Vec<Vec<u64>>,
    bits_a: &mut Vec<u64>,
    n: usize,
    c1: usize,
    c2: usize,
) {
    if c1 == c2 {
        return;
    }

    for r in 0..n {
        let bit1 = bit::get(&bits_row[r], c1);
        let bit2 = bit::get(&bits_row[r], c2);

        if bit1 != bit2 {
            bit::set(&mut bits_row[r], c1, bit2);
            bit::set(&mut bits_row[r], c2, bit1);
        }
    }

    bits_col.swap(c1, c2);

    let a1 = bit::get(bits_a, c1);
    let a2 = bit::get(bits_a, c2);

    if a1 != a2 {
        bit::set(bits_a, c1, a2);
        bit::set(bits_a, c2, a1);
    }
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let z = scan.token::<i64>();

    'outer: for _ in 0..z {
        let n = scan.token::<usize>();
        let mut profiles = vec![vec![0; n]; n];

        for i in 0..n {
            let line = scan.token::<String>();

            for (j, c) in line.chars().enumerate() {
                profiles[i][j] = if c == '1' { 1 } else { 0 };
            }
        }

        let bit = (n + 63) >> 6;
        let mut bits_row = vec![vec![0; bit]; n];
        let mut bits_col = vec![vec![0; bit]; n];

        for i in 0..n {
            for j in 0..n {
                if profiles[i][j] == 1 {
                    bit::set(&mut bits_row[i], j, true);
                    bit::set(&mut bits_col[j], i, true);
                }
            }
        }

        if parity_det(bits_row.clone(), n) {
            writeln!(out, "NO").unwrap();
            continue 'outer;
        }

        let mut vec_a = one_null_vector(bits_row.clone(), n);

        if !bit::get(&vec_a, 0) {
            if let Some(i) = (1..n).find(|&i| bit::get(&vec_a, i)) {
                swap_columns(&mut bits_row, &mut bits_col, &mut vec_a, n, 0, i);
            } else {
                writeln!(out, "YES").unwrap();
                continue 'outer;
            }
        }

        let mut col_w = vec![0; bit];

        for i in 0..n {
            let mut cnt_mod4 = 0;

            for j in 0..bit {
                let val = bits_row[i][j] & vec_a[j];
                cnt_mod4 = (cnt_mod4 + (val.count_ones() & 3)) & 3;
            }

            if ((cnt_mod4 >> 1) & 1) != 0 {
                bit::set(&mut col_w, i, true);
            }
        }

        let mut mat_w = bits_row.clone();

        for i in 0..n {
            bit::set(&mut mat_w[i], 0, bit::get(&col_w, i));
        }

        let parity = parity_det(mat_w, n) as u32;
        let mut mat = vec![vec![0; bit]; n];
        let mut ops = vec![vec![0; bit]; n];
        let mut sum = 0;

        for i in 1..n {
            if !bit::get(&vec_a, i) {
                continue;
            }

            for j in 0..n {
                mat[j].copy_from_slice(&bits_row[j]);
                bit::clear(&mut mat[j], 0);
                bit::clear(&mut mat[j], i);
            }

            for j in 0..n {
                for w in ops[j].iter_mut() {
                    *w = 0;
                }

                bit::set(&mut ops[j], j, true);
            }

            let rank = rref_with_ops(&mut mat, &mut ops, n, n - 2, 0, i);

            if rank < n - 2 {
                continue;
            }

            let s1 = parity_and2(&ops[n - 2], &bits_col[i]);
            let s2 = parity_and2(&ops[n - 1], &bits_col[i]);
            let s3 = parity_and3(&ops[n - 2], &ops[n - 1], &bits_col[i]);

            sum ^= ((s1 & s2) ^ s3) & 1;
        }

        writeln!(
            out,
            "{}",
            if ((parity ^ sum) & 1) == 0 {
                "YES"
            } else {
                "NO"
            }
        )
        .unwrap();
    }
}
