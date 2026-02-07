#![allow(dead_code)]

use io::Write;
use std::{io, str};

use crate::affine::AffinePerm;

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

mod unit_monge {
    #[derive(Copy, Clone, PartialEq, Eq)]
    enum Side {
        Left,
        Right,
    }

    fn multiply_unit_monge_matrix_distance_internal(row: &[usize], col: &[usize]) -> Vec<i128> {
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
            ret[row] = col as i128;
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

                if (row_block_side[cursor_upper] == Side::Right) != (row_to_col[cursor_upper] > col)
                {
                    skew_upper += 1;
                }
            }

            while cursor_lower > 0 && skew_lower <= 0 {
                cursor_lower -= 1;

                if (row_block_side[cursor_lower] == Side::Right)
                    != (row_to_col[cursor_lower] >= col)
                {
                    skew_lower += 1;
                }
            }

            if skew_lower > 0 && cursor_upper == cursor_lower {
                ret[cursor_lower] = col as i128;
            }

            if (col_block_side[col] == Side::Right) != (col_to_row[col] >= cursor_lower) {
                skew_lower -= 1;
            }
        }

        ret
    }

    fn multiply_unit_monge_matrix_distance(a: &[usize], b: &[usize]) -> Vec<i128> {
        let n = a.len();
        let mut b_rev = vec![0; n];

        for i in 0..n {
            b_rev[b[i]] = i;
        }

        multiply_unit_monge_matrix_distance_internal(a, &b_rev)
    }

    pub fn boxdot(a: &[usize], b: &[usize]) -> Vec<i128> {
        multiply_unit_monge_matrix_distance(a, b)
    }

    pub fn inverse_perm(perm: &[usize]) -> Vec<usize> {
        let n = perm.len();
        let mut inv = vec![0; n];

        for i in 0..n {
            inv[perm[i]] = i;
        }

        inv
    }
}

mod affine {
    use crate::unit_monge::{boxdot, inverse_perm};

    #[derive(Clone)]
    pub struct AffinePerm {
        n: usize,
        window: Vec<i128>,
    }

    impl AffinePerm {
        pub fn new(n: usize, window: Vec<i128>) -> Self {
            Self { n, window }
        }

        pub fn identity(n: usize) -> Self {
            AffinePerm::new(n, (0..n).map(|x| x as i128).collect())
        }

        pub fn window(&self) -> &Vec<i128> {
            &self.window
        }

        pub fn eval(&self, x: i128) -> i128 {
            let q = x.div_euclid(self.n as i128);
            let r = x.rem_euclid(self.n as i128) as usize;
            q * self.n as i128 + self.window[r]
        }

        pub fn inverse(&self) -> Self {
            let mut window_inv = vec![0; self.n];

            for i in 0..self.n {
                let q = self.window[i].div_euclid(self.n as i128);
                let r = self.window[i].rem_euclid(self.n as i128) as usize;
                window_inv[r] = i as i128 - q * self.n as i128;
            }

            AffinePerm::new(self.n, window_inv)
        }

        pub fn extend_degree(&self, n_new: usize) -> Self {
            assert!(n_new % self.n == 0);
            let mut window = Vec::with_capacity(n_new);

            for i in 0..n_new {
                window.push(self.eval(i as i128));
            }

            AffinePerm::new(n_new, window)
        }

        pub fn compose(a: &Self, b: &Self) -> Self {
            assert_eq!(a.n, b.n);
            let n = a.n;
            let mut window = vec![0; n];

            for i in 0..n {
                window[i] = b.eval(a.window[i]);
            }

            AffinePerm::new(n, window)
        }
    }

    impl AffinePerm {
        fn decompose_phi_gamma(&self) -> (Self, Self) {
            let mut pairs = (0..self.n).map(|i| (self.window[i], i)).collect::<Vec<_>>();
            pairs.sort_unstable();

            let mut phi = vec![0; self.n];
            let mut gamma = vec![0; self.n];

            for (rank, (val, idx)) in pairs.into_iter().enumerate() {
                phi[idx] = rank as i128;
                gamma[rank] = val;
            }

            (AffinePerm::new(self.n, phi), AffinePerm::new(self.n, gamma))
        }

        fn decompose_gamma_phi(&self) -> (Self, Self) {
            let inv = self.inverse();
            let (phi, gamma) = inv.decompose_phi_gamma();
            (gamma.inverse(), phi.inverse())
        }

        pub fn multiply_sticky(&self, other: &Self) -> Self {
            let p3 = self.extend_degree(self.n * 3);
            let q3 = other.extend_degree(self.n * 3);

            let (_gamma_p, phi_p) = p3.decompose_gamma_phi();
            let (phi_q, gamma_q) = q3.decompose_phi_gamma();

            let p = phi_p.window.iter().map(|&x| x as usize).collect::<Vec<_>>();
            let q = phi_q.window.iter().map(|&x| x as usize).collect::<Vec<_>>();
            let r = boxdot(&p, &q);

            let p_inv = inverse_perm(&p);
            let mut q_quotient_p = vec![0; self.n * 3];

            for i in 0..self.n * 3 {
                q_quotient_p[i] = r[p_inv[i]];
            }

            let mut s = vec![0; self.n];

            for i in 0..self.n {
                let val = q_quotient_p[self.n + i];
                s[i] = gamma_q.eval(val) - self.n as i128;
            }

            let q_tilde_quotient_p_tilde = AffinePerm::new(self.n, s);
            AffinePerm::compose(self, &q_tilde_quotient_p_tilde)
        }

        pub fn pow(&self, mut exp: u64) -> Self {
            let mut curr = self.clone();
            let mut ret = AffinePerm::identity(self.n);

            while exp > 0 {
                if exp % 2 == 1 {
                    ret = ret.multiply_sticky(&curr);
                }

                curr = curr.multiply_sticky(&curr);
                exp /= 2;
            }

            ret
        }
    }
}

fn compute_kernel_perm(p: &Vec<char>, q: &Vec<char>) -> Vec<i128> {
    let n = p.len();
    let m = q.len();

    let mut left = (0..n as i128).collect::<Vec<_>>();
    let mut top = (0..m).map(|j| (n + j) as i128).collect::<Vec<_>>();

    for i in 0..n {
        let idx = n - 1 - i;
        let mut pos_left = left[idx];

        for j in 0..m {
            let pos_top = top[j];

            if p[i] == q[j] || pos_left > pos_top {
                top[j] = pos_left;
                pos_left = pos_top;
            }
        }

        left[idx] = pos_left;
    }

    let mut perm = vec![0; n + m];

    for i in 0..n {
        perm[left[i] as usize] = (m + i) as i128;
    }

    for i in 0..m {
        perm[top[i] as usize] = i as i128;
    }

    perm
}

fn is_affine(window: &Vec<i128>) -> bool {
    let n = window.len();
    let mut visited = vec![false; n];

    for &idx in window {
        let r = idx.rem_euclid(n as i128) as usize;

        if visited[r] {
            return false;
        }

        visited[r] = true;
    }

    true
}

const PERIODS_MAX: usize = 1 << 10;

fn build_base_kernel(a: &Vec<char>, b: &Vec<char>) -> AffinePerm {
    let n = a.len();
    let m = b.len();

    let scale = (n + m - 1) / m;
    let mut periods = 10.max(8 * scale + 10);
    let margin = 5 * n;

    loop {
        if periods > PERIODS_MAX {
            panic!("Failed to find stable periodic window");
        }

        let len = periods * m;

        if len < margin + 3 * m + margin {
            periods *= 2;
            continue;
        }

        let mut b_ext = Vec::with_capacity(len);

        for _ in 0..periods {
            b_ext.extend_from_slice(b);
        }

        let perm = compute_kernel_perm(a, &b_ext);
        let offset_start = ((margin + m - 1) / m) * m;
        let offset_end = len - margin - 3 * m;

        for offset in (offset_start..=offset_end).step_by(m) {
            let mut window = vec![0; m];

            for i in 0..m {
                let idx_start = n + offset + i;
                window[i] = perm[idx_start] as i128 - offset as i128;
            }

            if !is_affine(&window) {
                continue;
            }

            let offset1 = offset + m;
            let mut check = true;

            for i in 0..m {
                let idx_start = n + offset1 + i;
                let val = perm[idx_start] as i128 - offset1 as i128;

                if val != window[i] {
                    check = false;
                    break;
                }
            }

            if !check {
                continue;
            }

            let offset2 = offset + 2 * m;

            for i in 0..m {
                let idx_start = n + offset2 + i;
                let val = perm[idx_start] as i128 - offset2 as i128;

                if val != window[i] {
                    check = false;
                    break;
                }
            }

            if !check {
                continue;
            }

            return AffinePerm::new(m, window);
        }

        periods *= 2;
    }
}

// Reference: https://drops.dagstuhl.de/storage/00lipics/lipics-vol331-cpm2025/LIPIcs.CPM.2025.13/LIPIcs.CPM.2025.13.pdf
fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let (n, m) = (scan.token::<usize>(), scan.token::<usize>());
    let a = scan.token::<String>();
    let b = scan.token::<String>();

    let a = a.chars().collect::<Vec<_>>();
    let b = b.chars().collect::<Vec<_>>();
    let mut alphabets_a = [false; 26];
    let mut alphabets_b = [false; 26];

    for c in a.iter() {
        alphabets_a[(*c as u8 - b'A') as usize] = true;
    }

    for c in b.iter() {
        alphabets_b[(*c as u8 - b'A') as usize] = true;
    }

    let base = build_base_kernel(&a, &b);
    let kernel = base.pow(n as u64);
    let period = kernel.window().len() as i128;
    let mut ret = 0;

    for &val in kernel.window() {
        let q = val.div_euclid(period);
        ret += q.clamp(0, m as i128);
    }

    writeln!(out, "{ret}").unwrap();
}
